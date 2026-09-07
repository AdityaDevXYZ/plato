
export const OrbitalData = [
  {
    missionId: 'm-alpha',
    missionName: 'Alpha Constellation',
    assets: [
      { id: 'SAT-A-001', name: 'Alpha-1', type: 'Satellite', status: 'Nominal', lat: 34.05, lng: -118.24, alt: '550km', vel: '7.5 km/s', period: '95m', inc: '53°', nextWindow: '14m', confidence: 99.8 },
      { id: 'SAT-A-002', name: 'Alpha-2', type: 'Satellite', status: 'Warning', lat: 51.50, lng: -0.12, alt: '548km', vel: '7.5 km/s', period: '95m', inc: '53°', nextWindow: '42m', confidence: 98.5 },
      { id: 'SAT-A-003', name: 'Alpha-3', type: 'Satellite', status: 'Nominal', lat: 35.68, lng: 139.69, alt: '551km', vel: '7.5 km/s', period: '95m', inc: '53°', nextWindow: 'Now', confidence: 99.9 },
    ]
  },
  {
    missionId: 'ground',
    missionName: 'Ground Stations',
    assets: [
      { id: 'GS-001', name: 'US-East Station', type: 'GroundStation', status: 'Nominal', lat: 38.90, lng: -77.03, alt: '0km', vel: '0', period: 'N/A', inc: 'N/A', nextWindow: 'N/A', confidence: 100 },
      { id: 'GS-002', name: 'EU-West Station', type: 'GroundStation', status: 'Nominal', lat: 48.85, lng: 2.35, alt: '0km', vel: '0', period: 'N/A', inc: 'N/A', nextWindow: 'N/A', confidence: 100 },
    ]
  }
];

export const getCommWindows = (assetId: string) => {
  return [
    { start: '14:30', end: '14:45', duration: '15m', station: 'US-East Station', signal: 'High' },
    { start: '16:15', end: '16:22', duration: '7m', station: 'EU-West Station', signal: 'Medium' },
    { start: '18:00', end: '18:12', duration: '12m', station: 'US-East Station', signal: 'High' },
  ];
};

export const OrbitEvents = [
  { id: 1, type: 'comm_open', message: 'Communication window opened with EU-West', time: 'Just now' },
  { id: 2, type: 'predict_up', message: 'Orbital prediction updated (Conf: 99.9%)', time: '5m ago' },
  { id: 3, type: 'comm_close', message: 'Communication window closed with US-East', time: '45m ago' },
];
