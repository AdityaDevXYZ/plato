
export const UsersData = [
  { id: 'u-1', name: 'Alice Systems', email: 'alice@plato.aero', role: 'Administrator', status: 'Active', mfa: true, lastLogin: '2 mins ago' },
  { id: 'u-2', name: 'Bob Mission', email: 'bob@plato.aero', role: 'Mission Manager', status: 'Active', mfa: true, lastLogin: '1 hour ago' },
  { id: 'u-3', name: 'Charlie Ops', email: 'charlie@plato.aero', role: 'Operator', status: 'Active', mfa: false, lastLogin: '1 day ago' },
  { id: 'u-4', name: 'Diana Eng', email: 'diana@plato.aero', role: 'Engineer', status: 'Inactive', mfa: false, lastLogin: '2 weeks ago' },
];

export const RolesData = [
  { name: 'Administrator', desc: 'Full access to all systems, billing, and security settings.', users: 3 },
  { name: 'Mission Manager', desc: 'Can plan and approve deployments. Cannot manage users.', users: 12 },
  { name: 'Operator', desc: 'Can execute and pause deployments. Cannot approve plans.', users: 45 },
  { name: 'Engineer', desc: 'Can view telemetry and logs. Cannot modify state.', users: 80 },
  { name: 'Read Only', desc: 'Can view dashboards. No action permissions.', users: 15 },
];

export const AuditLogs = [
  { id: 'log-001', action: 'Deployment approved', actor: 'Bob Mission', target: 'dep-001', time: '10:05:00 UTC', status: 'Success' },
  { id: 'log-002', action: 'API key created', actor: 'Alice Systems', target: 'key-ext-1', time: '09:30:15 UTC', status: 'Success' },
  { id: 'log-003', action: 'Login', actor: 'Charlie Ops', target: 'auth', time: '08:45:00 UTC', status: 'Success' },
  { id: 'log-004', action: 'Failed Login', actor: 'Unknown', target: 'auth', time: '08:42:10 UTC', status: 'Failed' },
  { id: 'log-005', action: 'Role changed', actor: 'Alice Systems', target: 'Diana Eng (Role: Engineer)', time: 'Yesterday', status: 'Success' },
];

export const ApiKeys = [
  { id: 'key-1', name: 'Telemetry Ingest Agent', created: '2026-01-10', lastUsed: '5 mins ago', scopes: ['telemetry:write'], expires: 'Never' },
  { id: 'key-2', name: 'LITHOS Integration', created: '2026-05-15', lastUsed: '1 hour ago', scopes: ['mission:read', 'orbit:read'], expires: '2026-11-15' },
];

export const IntegrationsData = [
  { id: 'int-1', name: 'LITHOS', desc: 'Spacecraft Manufacturing & Testing', status: 'Connected' },
  { id: 'int-2', name: 'MERIDIAN', desc: 'Global Ground Station Network', status: 'Connected' },
  { id: 'int-3', name: 'CRATON', desc: 'Secure Payload Storage', status: 'Disconnected' },
  { id: 'int-4', name: 'AURORA', desc: 'Advanced Orbital Modeling', status: 'Pending' },
];

export const WebhooksData = [
  { id: 'wh-1', url: 'https://api.external.com/v1/plato-hook', status: 'Active', events: ['deployment.completed', 'risk.high'] },
  { id: 'wh-2', url: 'https://hooks.slack.com/services/T00...', status: 'Active', events: ['*'] },
];
