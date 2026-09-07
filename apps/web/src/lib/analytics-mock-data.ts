
export const AnalyticsReports = [
  { id: 'rep-001', name: 'Operational Summary (Q2)', category: 'Executive', date: '2026-07-01' },
  { id: 'rep-002', name: 'Alpha Mission Performance', category: 'Mission', date: '2026-07-15' },
  { id: 'rep-003', name: 'Deployment Strategy Review', category: 'Deployment', date: '2026-07-18' },
  { id: 'rep-004', name: 'Fleet Reliability Report', category: 'Reliability', date: '2026-07-20' },
];

export const SuccessTrendData = [
  { month: 'Jan', rate: 94 },
  { month: 'Feb', rate: 95 },
  { month: 'Mar', rate: 93 },
  { month: 'Apr', rate: 97 },
  { month: 'May', rate: 98 },
  { month: 'Jun', rate: 99 },
];

export const StrategyComparisonData = [
  { name: 'Canary', duration: 180, success: 99, rollback: 1 },
  { name: 'Sequential', duration: 240, success: 95, rollback: 5 },
  { name: 'Batch', duration: 120, success: 92, rollback: 8 },
  { name: 'Regional', duration: 160, success: 97, rollback: 3 },
];

export const FailureDistributionData = [
  { name: 'Timeouts', value: 45 },
  { name: 'Payload Errors', value: 25 },
  { name: 'Comm Windows', value: 20 },
  { name: 'Hardware Faults', value: 10 },
];
