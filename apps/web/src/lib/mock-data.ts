
export const KPIs = {
  totalSatellites: 248,
  onlineSatellites: 242,
  activeMissions: 12,
  deploymentsToday: 3,
  pendingPlans: 5,
  criticalAlerts: 1,
  overallHealth: 97.5
};

export const FleetHealthData = [
  { name: 'Healthy', value: 210, fill: 'hsl(var(--primary))' },
  { name: 'Warning', value: 25, fill: '#eab308' },
  { name: 'Critical', value: 7, fill: 'hsl(var(--destructive))' },
  { name: 'Offline', value: 6, fill: 'hsl(var(--muted-foreground))' },
];

export const MissionStatus = [
  { id: 'm-1', name: 'Alpha Constellation', status: 'Active', satellites: 64, release: 'v2.4.1', risk: 'Low', lastActivity: '10m ago' },
  { id: 'm-2', name: 'Beta Network Upgrade', status: 'Deploying', satellites: 128, release: 'v3.0.0-rc.1', risk: 'Medium', lastActivity: 'Just now' },
  { id: 'm-3', name: 'Gamma Observational', status: 'Planning', satellites: 32, release: 'v1.9.4', risk: 'Low', lastActivity: '2h ago' },
  { id: 'm-4', name: 'Delta Legacy', status: 'Maintenance', satellites: 24, release: 'v1.2.0', risk: 'High', lastActivity: '1d ago' },
];

export const RecentActivity = [
  { id: 1, type: 'deployment', title: 'Beta Network Wave 1 Started', time: 'Just now', description: 'Coordination Engine triggered Wave 1 for 32 satellites.' },
  { id: 2, type: 'risk', title: 'Risk Assessment Completed', time: '15m ago', description: 'Gamma Observational plan evaluated. Overall risk: Low.' },
  { id: 3, type: 'auth', title: 'Admin Login', time: '1h ago', description: 'User J. Doe authenticated successfully.' },
  { id: 4, type: 'analytics', title: 'Weekly Analytics Generated', time: '3h ago', description: 'Fleet reliability score updated.' },
  { id: 5, type: 'coordination', title: 'Alpha Update Completed', time: '1d ago', description: 'All waves completed successfully.' },
];

export const Alerts = [
  { id: 1, severity: 'Critical', message: 'Loss of telemetry on SAT-84', time: '5m ago' },
  { id: 2, severity: 'Warning', message: 'High CPU utilization on Ground Station Alpha', time: '22m ago' },
  { id: 3, severity: 'Information', message: 'Wave 1 deployment nominal', time: '1h ago' },
  { id: 4, severity: 'Warning', message: 'Orbital drift detected on SAT-12', time: '2h ago' },
];

export const SystemHealth = [
  { service: 'API Gateway', status: 'Operational', uptime: '99.99%' },
  { service: 'PostgreSQL DB', status: 'Operational', uptime: '100%' },
  { service: 'Event Bus', status: 'Operational', uptime: '99.95%' },
  { service: 'Telemetry Ingest', status: 'Degraded', uptime: '98.50%' },
  { service: 'Worker Pool', status: 'Operational', uptime: '100%' },
];
