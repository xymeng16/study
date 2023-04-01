/**
 * This file was auto-generated by @ui/openapi.
 * Do not make direct changes to the file.
 */

export const actions = [],
  data = [
    {
      field: 'label',
      label: 'containerNames',
      value: [],
      helperText:
        'Optional. ContainerNames indicates list of the name of affected container. If not set, the first container will be injected',
    },
    {
      field: 'text',
      label: 'remoteCluster',
      value: '',
      helperText: 'Optional. RemoteCluster represents the remote cluster where the chaos will be deployed',
    },
    {
      field: 'text',
      label: 'stressngStressors',
      value: '',
      helperText:
        "Optional. StressngStressors defines plenty of stressors just like `Stressors` except that it's an experimental feature and more powerful. You can define stressors in `stress-ng` (see also `man stress-ng`) dialect, however not all of the supported stressors are well tested. It maybe retired in later releases. You should always use `Stressors` to define the stressors and use this only when you want more stressors unsupported by `Stressors`. When both `StressngStressors` and `Stressors` are defined, `StressngStressors` wins.",
    },
    {
      field: 'ref',
      label: 'stressors',
      children: [
        {
          field: 'ref',
          label: 'cpu',
          children: [
            {
              field: 'number',
              label: 'load',
              value: 0,
              helperText:
                'Optional. Load specifies P percent loading per CPU worker. 0 is effectively a sleep (no load) and 100 is full loading.',
            },
            {
              field: 'label',
              label: 'options',
              value: [],
              helperText: 'Optional. extend stress-ng options',
            },
            {
              field: 'number',
              label: 'workers',
              value: 0,
              helperText:
                'Workers specifies N workers to apply the stressor. Maximum 8192 workers can run by stress-ng',
            },
          ],
        },
        {
          field: 'ref',
          label: 'memory',
          children: [
            {
              field: 'number',
              label: 'oomScoreAdj',
              value: 0,
              helperText:
                'Optional. OOMScoreAdj sets the oom_score_adj of the stress process. See `man 5 proc` to know more about this option.',
            },
            {
              field: 'label',
              label: 'options',
              value: [],
              helperText: 'Optional. extend stress-ng options',
            },
            {
              field: 'text',
              label: 'size',
              value: '',
              helperText:
                'Optional. Size specifies N bytes consumed per vm worker, default is the total available memory. One can specify the size as % of total available memory or in units of B, KB/KiB, MB/MiB, GB/GiB, TB/TiB.',
            },
            {
              field: 'number',
              label: 'workers',
              value: 0,
              helperText:
                'Workers specifies N workers to apply the stressor. Maximum 8192 workers can run by stress-ng',
            },
          ],
        },
      ],
    },
  ]