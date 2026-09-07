
export const PlanningMissions = [
  { id: 'm-alpha', name: 'Alpha Constellation', targets: 64 },
  { id: 'm-beta', name: 'Beta Network Upgrade', targets: 128 },
  { id: 'm-gamma', name: 'Gamma Observational', targets: 32 },
];

export const ValidationResults = [
  { type: 'warning', message: 'Large rollout warning: Wave 2 exceeds 50 targets.' },
  { type: 'info', message: 'Communication window conflict detected in Wave 1. Engine will automatically resolve.' },
  { type: 'success', message: 'Resource availability verified for all selected targets.' }
];

export const MockPhases = [
  { id: 'p1', name: 'Canary Rollout', size: '5%', targets: 3, parallel: false },
  { id: 'p2', name: 'US Region', size: '20%', targets: 12, parallel: true },
  { id: 'p3', name: 'Global Fleet', size: '75%', targets: 49, parallel: true },
];
