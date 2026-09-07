
export const FleetData = [
  {
    missionId: 'm-alpha',
    missionName: 'Alpha Constellation',
    assets: [
      { id: 'SAT-A-001', name: 'Alpha-1', type: 'Satellite', status: 'Nominal', health: 98, battery: 95, cpu: 42, memory: 56, temp: 22, release: 'v2.4.1', orbit: 'LEO', x: 20, y: 30 },
      { id: 'SAT-A-002', name: 'Alpha-2', type: 'Satellite', status: 'Warning', health: 85, battery: 72, cpu: 85, memory: 60, temp: 34, release: 'v2.4.1', orbit: 'LEO', x: 40, y: 60 },
      { id: 'SAT-A-003', name: 'Alpha-3', type: 'Satellite', status: 'Nominal', health: 100, battery: 100, cpu: 20, memory: 40, temp: 18, release: 'v2.4.1', orbit: 'LEO', x: 70, y: 20 },
    ]
  },
  {
    missionId: 'm-beta',
    missionName: 'Beta Network Upgrade',
    assets: [
      { id: 'SAT-B-001', name: 'Beta-1', type: 'Satellite', status: 'Nominal', health: 95, battery: 88, cpu: 55, memory: 65, temp: 25, release: 'v3.0.0-rc.1', orbit: 'MEO', x: 80, y: 70 },
      { id: 'SAT-B-002', name: 'Beta-2', type: 'Satellite', status: 'Critical', health: 40, battery: 20, cpu: 95, memory: 90, temp: 45, release: 'v2.9.0', orbit: 'MEO', x: 30, y: 80 },
    ]
  },
  {
    missionId: 'ground',
    missionName: 'Ground Stations',
    assets: [
      { id: 'GS-001', name: 'US-East Station', type: 'GroundStation', status: 'Nominal', health: 100, connection: 'Active', x: 50, y: 50 },
      { id: 'GS-002', name: 'EU-West Station', type: 'GroundStation', status: 'Warning', health: 88, connection: 'Intermittent', x: 60, y: 50 },
    ]
  }
];

export const getTelemetryHistory = (assetId: string) => {
  return [
    { time: '10:00', value: Math.floor(Math.random() * 20) + 40 },
    { time: '10:05', value: Math.floor(Math.random() * 20) + 40 },
    { time: '10:10', value: Math.floor(Math.random() * 20) + 40 },
    { time: '10:15', value: Math.floor(Math.random() * 20) + 40 },
    { time: '10:20', value: Math.floor(Math.random() * 20) + 40 },
  ];
};

export const AssetEvents = [
  { id: 1, type: 'info', message: 'Telemetry sync completed', time: '1m ago' },
  { id: 2, type: 'warning', message: 'Battery temperature elevated', time: '5m ago' },
  { id: 3, type: 'success', message: 'Software payload deployed', time: '1h ago' },
];
