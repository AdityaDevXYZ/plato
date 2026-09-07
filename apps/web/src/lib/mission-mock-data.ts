
export const MissionsListMock = [
  { id: 'm-alpha', name: 'Alpha Constellation', status: 'Active', owner: 'J. Doe', satellites: 64, release: 'v2.4.1', health: 98, risk: 'Low', lastActivity: '10m ago' },
  { id: 'm-beta', name: 'Beta Network Upgrade', status: 'Deploying', owner: 'S. Smith', satellites: 128, release: 'v3.0.0-rc.1', health: 100, risk: 'Medium', lastActivity: 'Just now' },
  { id: 'm-gamma', name: 'Gamma Observational', status: 'Planning', owner: 'A. Patel', satellites: 32, release: 'v1.9.4', health: 92, risk: 'Low', lastActivity: '2h ago' },
  { id: 'm-delta', name: 'Delta Legacy', status: 'Maintenance', owner: 'R. Jones', satellites: 24, release: 'v1.2.0', health: 85, risk: 'High', lastActivity: '1d ago' },
];

export const getMissionDetails = (id: string) => {
  return MissionsListMock.find(m => m.id === id) || MissionsListMock[0];
};
