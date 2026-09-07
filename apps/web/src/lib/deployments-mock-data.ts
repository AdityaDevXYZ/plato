
export const DeploymentQueue = [
  { id: 'dep-001', mission: 'Beta Network Upgrade', status: 'Running', priority: 'High', targets: 128, progress: 45 },
  { id: 'dep-002', mission: 'Alpha Constellation', status: 'Queued', priority: 'Normal', targets: 64, progress: 0 },
  { id: 'dep-003', mission: 'Gamma Observational', status: 'Completed', priority: 'Normal', targets: 32, progress: 100 },
  { id: 'dep-004', mission: 'Delta Legacy', status: 'Failed', priority: 'Low', targets: 24, progress: 30 },
];

export const ActiveDeployment = {
  id: 'dep-001',
  mission: 'Beta Network Upgrade',
  planVersion: 'v3.0.0-rc.1',
  strategy: 'Canary Rollout',
  initiatedBy: 'J. Doe',
  startTime: '04:15:00 UTC',
  currentState: 'Executing Wave 2',
  rollbackReadiness: 'Verified',
  stats: {
    completedTargets: 58,
    remainingTargets: 70,
    elapsedTime: '45m 12s',
    estimatedRemaining: '1h 15m'
  },
  waves: [
    { id: 'w1', name: 'Canary Phase (US-East)', targets: 10, status: 'Completed', progress: 100, retries: 0 },
    { id: 'w2', name: 'Global Fleet Partition A', targets: 50, status: 'Running', progress: 96, retries: 1 },
    { id: 'w3', name: 'Global Fleet Partition B', targets: 68, status: 'Pending', progress: 0, retries: 0 },
  ],
  events: [
    { id: 1, type: 'info', message: 'Deployment initialized', time: '45m ago' },
    { id: 2, type: 'info', message: 'Wave 1 (Canary Phase) started', time: '44m ago' },
    { id: 3, type: 'success', message: 'Wave 1 completed successfully', time: '30m ago' },
    { id: 4, type: 'info', message: 'Wave 2 (Global Fleet Partition A) started', time: '29m ago' },
    { id: 5, type: 'warning', message: 'Retry triggered on SAT-B-042 due to timeout', time: '10m ago' },
  ]
};

export const PostDeploymentStats = {
  successRate: 98,
  failedAssets: 1,
  retryCount: 3,
  duration: '2h 45m',
};
