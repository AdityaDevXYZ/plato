
export const AssessmentHistory = [
  { id: 'cra-001', mission: 'Alpha Constellation', plan: 'PL-001', date: '2026-07-20', status: 'Completed', overallRisk: 'Low', score: 95 },
  { id: 'cra-002', mission: 'Beta Network Upgrade', plan: 'PL-002', date: '2026-07-19', status: 'Completed', overallRisk: 'Medium', score: 72 },
  { id: 'cra-003', mission: 'Gamma Observational', plan: 'PL-003', date: '2026-07-18', status: 'Completed', overallRisk: 'High', score: 45 },
  { id: 'cra-004', mission: 'Delta Legacy', plan: 'PL-004', date: '2026-07-17', status: 'Completed', overallRisk: 'Low', score: 91 },
];

export const RiskCategories = [
  { name: 'Operational', score: 98, severity: 'Low', confidence: 'High', summary: 'Nominal operational state across all target assets.' },
  { name: 'Orbital', score: 85, severity: 'Medium', confidence: 'Medium', summary: 'Minor debris proximity warning on SAT-B-002. Maneuver unnecessary.' },
  { name: 'Resource', score: 60, severity: 'High', confidence: 'High', summary: 'Insufficient battery capacity for concurrent deployment wave.' },
  { name: 'Communication', score: 92, severity: 'Low', confidence: 'High', summary: 'Sufficient ground station windows available.' },
  { name: 'Mission', score: 100, severity: 'Low', confidence: 'High', summary: 'Mission objectives align with plan constraints.' },
  { name: 'Infrastructure', score: 95, severity: 'Low', confidence: 'High', summary: 'Backend orchestration engines operating nominally.' },
  { name: 'Software', score: 99, severity: 'Low', confidence: 'High', summary: 'Release payload cryptographically verified.' },
];

export const RiskTrendData = [
  { day: 'Mon', risk: 15 },
  { day: 'Tue', risk: 20 },
  { day: 'Wed', risk: 10 },
  { day: 'Thu', risk: 35 },
  { day: 'Fri', risk: 12 },
  { day: 'Sat', risk: 8 },
  { day: 'Sun', risk: 5 },
];

export const CategoryComparisonData = [
  { subject: 'Operational', A: 120, B: 110, fullMark: 150 },
  { subject: 'Orbital', A: 98, B: 130, fullMark: 150 },
  { subject: 'Resource', A: 86, B: 130, fullMark: 150 },
  { subject: 'Communication', A: 99, B: 100, fullMark: 150 },
  { subject: 'Software', A: 85, B: 90, fullMark: 150 },
  { subject: 'Infrastructure', A: 65, B: 85, fullMark: 150 },
];
