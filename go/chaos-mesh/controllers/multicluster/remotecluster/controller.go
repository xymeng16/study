// Copyright 2022 Chaos Mesh Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

package remotecluster

import (
	"context"
	"encoding/json"

	"github.com/go-logr/logr"
	"github.com/pkg/errors"
	"helm.sh/helm/v3/pkg/storage/driver"
	corev1 "k8s.io/api/core/v1"
	apierrors "k8s.io/apimachinery/pkg/api/errors"
	"k8s.io/apimachinery/pkg/types"
	"k8s.io/client-go/tools/clientcmd"
	"k8s.io/client-go/util/retry"
	ctrl "sigs.k8s.io/controller-runtime"
	"sigs.k8s.io/controller-runtime/pkg/client"

	"github.com/chaos-mesh/chaos-mesh/api/v1alpha1"
	"github.com/chaos-mesh/chaos-mesh/controllers/multicluster/clusterregistry"
	"github.com/chaos-mesh/chaos-mesh/pkg/helm"
)

const remoteClusterControllerFinalizer = "chaos-mesh/remotecluster-controllers"
const chaosMeshReleaseName = "chaos-mesh"
const chaosMeshReleaseVersion = "2.4.1"

type Reconciler struct {
	Log      logr.Logger
	registry *clusterregistry.RemoteClusterRegistry

	client.Client
}

func (r *Reconciler) getRestConfig(ctx context.Context, secretRef v1alpha1.RemoteClusterSecretRef) (clientcmd.ClientConfig, error) {
	var secret corev1.Secret
	err := r.Client.Get(ctx, types.NamespacedName{
		Namespace: secretRef.Namespace,
		Name:      secretRef.Name,
	}, &secret)
	if err != nil {
		return nil, errors.Wrapf(err, "get secret %s/%s", secretRef.Namespace, secretRef.Name)
	}

	kubeconfig := secret.Data[secretRef.Key]

	config, err := clientcmd.Load(kubeconfig)
	if err != nil {
		return nil, errors.Wrap(err, "load kubeconfig")
	}

	return clientcmd.NewDefaultClientConfig(*config, nil), nil
}

func (r *Reconciler) Reconcile(ctx context.Context, req ctrl.Request) (ctrl.Result, error) {
	var obj v1alpha1.RemoteCluster
	err := r.Client.Get(ctx, req.NamespacedName, &obj)
	if err != nil {
		if apierrors.IsNotFound(err) {
			r.Log.Info("remote cluster not found", "namespace", req.Namespace, "name", req.Name)
		} else {
			// TODO: handle this error
			r.Log.Error(err, "unable to get remote cluster", "namespace", req.Namespace, "name", req.Name)
		}
		return ctrl.Result{}, nil
	}

	clientConfig, err := r.getRestConfig(ctx, obj.Spec.KubeConfig.SecretRef)
	if err != nil {
		r.Log.Error(err, "fail to get clientConfig from secret")
		return ctrl.Result{Requeue: true}, nil
	}

	// if the remoteCluster itself is being deleted, we should remove the cluster controller manager
	if !obj.DeletionTimestamp.IsZero() {
		err := r.registry.Stop(ctx, obj.Name)
		if err != nil {
			if !errors.Is(err, clusterregistry.ErrNotExist) {
				r.Log.Error(err, "fail to stop cluster")
				return ctrl.Result{Requeue: true}, nil
			}
		}

		err = r.uninstallHelmRelease(ctx, &obj, clientConfig)
		if err != nil {
			r.Log.Error(err, "fail to uninstall helm release")
			return ctrl.Result{Requeue: true}, nil
		}

		err = retry.RetryOnConflict(retry.DefaultRetry, func() error {
			var newObj v1alpha1.RemoteCluster
			r.Client.Get(ctx, req.NamespacedName, &newObj)

			newObj.Finalizers = []string{}
			setRemoteClusterCondition(&newObj, v1alpha1.RemoteClusterConditionInstalled, corev1.ConditionFalse, "")

			return r.Client.Update(ctx, &newObj)
		})
		if err != nil {
			r.Log.Error(err, "fail to update finalizer", "name", obj.Name)
			return ctrl.Result{Requeue: true}, nil
		}
		return ctrl.Result{}, nil
	}

	err = r.ensureHelmRelease(ctx, &obj, clientConfig)
	if err != nil {
		r.Log.Error(err, "fail to list or install remote helm release")
		return ctrl.Result{Requeue: true}, nil
	}

	err = r.ensureClusterControllerManager(ctx, &obj, clientConfig)
	if err != nil {
		r.Log.Error(err, "fail to boot remote cluster controller manager")
		return ctrl.Result{Requeue: true}, nil
	}
	obj.Finalizers = []string{remoteClusterControllerFinalizer}

	if err != nil {
		r.Log.Error(err, "fail to operate the helm release in remote cluster")
		return ctrl.Result{Requeue: true}, nil
	}

	err = retry.RetryOnConflict(retry.DefaultRetry, func() error {
		var newObj v1alpha1.RemoteCluster
		r.Client.Get(ctx, req.NamespacedName, &newObj)

		newObj.Finalizers = obj.Finalizers
		setRemoteClusterCondition(&newObj, v1alpha1.RemoteClusterConditionInstalled, corev1.ConditionTrue, "")
		// TODO: do auto config migration
		newObj.Status.CurrentVersion = chaosMeshReleaseVersion
		return r.Client.Update(ctx, &newObj)
	})
	if err != nil {
		r.Log.Error(err, "fail to update finalizer", "name", obj.Name)
		return ctrl.Result{Requeue: true}, nil
	}

	return ctrl.Result{}, nil
}

func (r *Reconciler) ensureClusterControllerManager(ctx context.Context, obj *v1alpha1.RemoteCluster, config clientcmd.ClientConfig) error {
	restConfig, err := config.ClientConfig()
	if err != nil {
		return errors.Wrap(err, "get rest config from client config")
	}

	err = r.registry.Spawn(obj.Name, restConfig)
	if err != nil {
		if !errors.Is(err, clusterregistry.ErrAlreadyExist) {
			return err
		}
	}

	return nil
}

func (r *Reconciler) getHelmClient(ctx context.Context, clientConfig clientcmd.ClientConfig) (*helm.HelmClient, error) {
	restClientGetter := helm.NewRESTClientGetter(clientConfig)

	helmClient, err := helm.NewHelmClient(restClientGetter, r.Log)
	if err != nil {
		return nil, err
	}

	return helmClient, nil
}

func (r *Reconciler) ensureHelmRelease(ctx context.Context, obj *v1alpha1.RemoteCluster, clientConfig clientcmd.ClientConfig) error {
	helmClient, err := r.getHelmClient(ctx, clientConfig)
	if err != nil {
		return err
	}

	_, err = helmClient.GetRelease(obj.Spec.Namespace, chaosMeshReleaseName)
	if err != nil {
		if errors.Is(err, driver.ErrReleaseNotFound) {
			chart, err := helm.FetchChaosMeshChart(ctx, chaosMeshReleaseVersion)
			if err != nil {
				return err
			}

			values := make(map[string]interface{})
			if obj.Spec.ConfigOverride != nil {
				err = json.Unmarshal(obj.Spec.ConfigOverride, &values)
				if err != nil {
					return err
				}
			}
			_, err = helmClient.UpgradeOrInstall(obj.Spec.Namespace, chaosMeshReleaseName, chart, values)
			if err != nil {
				return err
			}
		} else {
			return err
		}
	}
	return nil
}

func (r *Reconciler) uninstallHelmRelease(ctx context.Context, obj *v1alpha1.RemoteCluster, clientConfig clientcmd.ClientConfig) error {
	helmClient, err := r.getHelmClient(ctx, clientConfig)
	if err != nil {
		return err
	}

	_, err = helmClient.GetRelease(obj.Spec.Namespace, chaosMeshReleaseName)
	if err != nil {
		if errors.Is(err, driver.ErrReleaseNotFound) {
			return nil
		}

		return err
	}

	// the release still exist
	_, err = helmClient.UninstallRelease(obj.Spec.Namespace, chaosMeshReleaseName)
	if err != nil {
		if errors.Is(err, driver.ErrReleaseNotFound) {
			return nil
		}

		return err
	}

	return nil
}
