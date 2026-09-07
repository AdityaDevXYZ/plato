
export type AppEventSeverity = 'info' | 'warning' | 'critical' | 'success';

export interface BaseEvent {
  id: string;
  type: string;
  timestamp: string;
  channel: string;
}

export interface NotificationEvent extends BaseEvent {
  title: string;
  message: string;
  severity: AppEventSeverity;
  link?: string;
  engine: 'DigitalTwin' | 'Orbital' | 'Planning' | 'Risk' | 'Deployment' | 'System';
}

export interface TelemetryEvent extends BaseEvent {
  assetId: string;
  metrics: {
    cpu?: number;
    memory?: number;
    battery?: number;
  };
}

export interface DeploymentProgressEvent extends BaseEvent {
  deploymentId: string;
  status: string;
  progress: number;
}
