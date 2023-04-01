// Copyright 2021 Chaos Mesh Authors.
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

package main

import (
	"github.com/go-logr/logr"
	"github.com/pkg/errors"
	"sync"
)

// clockGettimeSkewFakeImage is the filename of fake image after compiling
const clockGettimeSkewFakeImage = "fake_clock_gettime.o"

// clockGettime is the target function would be replaced
const clockGettime = "clock_gettime"

// These three consts corresponding to the three extern variables in the fake_clock_gettime.c
const (
	externVarClockIdsMask = "CLOCK_IDS_MASK"
	externVarTvSecDelta   = "TV_SEC_DELTA"
	externVarTvNsecDelta  = "TV_NSEC_DELTA"
)

// timeofdaySkewFakeImage is the filename of fake image after compiling
const timeOfDaySkewFakeImage = "fake_gettimeofday.o"

// getTimeOfDay is the target function would be replaced
const getTimeOfDay = "gettimeofday"

// Config is the summary config of get_time_of_day and clock_get_time.
// Config here is only for injector of k8s pod.
// We divide group injector on linux process , pod injector for k8s and
// the base injector , so we can simply create another config struct just
// for linux process for chaos-mesh/chaosd or watchmaker.
type Config struct {
	deltaSeconds     int64
	deltaNanoSeconds int64
	clockIDsMask     uint64
}

func NewConfig(deltaSeconds int64, deltaNanoSeconds int64, clockIDsMask uint64) Config {
	return Config{
		deltaSeconds:     deltaSeconds,
		deltaNanoSeconds: deltaNanoSeconds,
		clockIDsMask:     clockIDsMask,
	}
}

type ConfigCreatorParas struct {
	Logger logr.Logger
	Config Config
}

// Skew implements ChaosOnProcessGroup.
// We locked Skew injecting and recovering to avoid conflict.
type Skew struct {
	SkewConfig   Config
	clockGetTime *FakeImage
	getTimeOfDay *FakeImage

	locker sync.Mutex
	logger logr.Logger
}

func GetSkew(logger logr.Logger, c Config) (Skew, error) {
	clockGetTimeImage, err := LoadFakeImageFromEmbedFs(clockGettimeSkewFakeImage, clockGettime, logger)
	if err != nil {
		return Skew{}, errors.Wrap(err, "load fake image")
	}

	getTimeOfDayimage, err := LoadFakeImageFromEmbedFs(timeOfDaySkewFakeImage, getTimeOfDay, logger)
	if err != nil {
		return Skew{}, errors.Wrap(err, "load fake image")
	}

	return Skew{
		SkewConfig:   c,
		clockGetTime: clockGetTimeImage,
		getTimeOfDay: getTimeOfDayimage,
		locker:       sync.Mutex{},
		logger:       logger,
	}, nil
}

func (s *Skew) Inject(pid int) error {
	s.locker.Lock()
	defer s.locker.Unlock()

	s.logger.Info("injecting time skew", "pid", pid)

	err := s.clockGetTime.AttachToProcess(pid, map[string]uint64{
		externVarClockIdsMask: s.SkewConfig.clockIDsMask,
		externVarTvSecDelta:   uint64(s.SkewConfig.deltaSeconds),
		externVarTvNsecDelta:  uint64(s.SkewConfig.deltaNanoSeconds),
	})
	if err != nil {
		return err
	}

	err = s.getTimeOfDay.AttachToProcess(pid, map[string]uint64{
		externVarTvSecDelta:  uint64(s.SkewConfig.deltaSeconds),
		externVarTvNsecDelta: uint64(s.SkewConfig.deltaNanoSeconds),
	})
	if err != nil {
		return err
	}
	return nil
}

// Recover clock_get_time & get_time_of_day one by one ,
// if error comes from clock_get_time.Recover we will continue recover another fake image
// and merge errors.
func (s *Skew) Recover(pid int) error {
	s.locker.Lock()
	defer s.locker.Unlock()

	s.logger.Info("recovering time skew", "pid", pid)

	err1 := s.clockGetTime.Recover(pid, map[string]uint64{
		externVarClockIdsMask: s.SkewConfig.clockIDsMask,
		externVarTvSecDelta:   uint64(s.SkewConfig.deltaSeconds),
		externVarTvNsecDelta:  uint64(s.SkewConfig.deltaNanoSeconds),
	})
	if err1 != nil {
		err2 := s.getTimeOfDay.Recover(pid, map[string]uint64{
			externVarTvSecDelta:  uint64(s.SkewConfig.deltaSeconds),
			externVarTvNsecDelta: uint64(s.SkewConfig.deltaNanoSeconds),
		})
		if err2 != nil {
			return errors.Wrapf(err1, "time skew all failed %v", err2)
		}
		return err1
	}

	err2 := s.getTimeOfDay.Recover(pid, map[string]uint64{
		externVarTvSecDelta:  uint64(s.SkewConfig.deltaSeconds),
		externVarTvNsecDelta: uint64(s.SkewConfig.deltaNanoSeconds),
	})
	if err2 != nil {
		return err2
	}

	return nil
}
