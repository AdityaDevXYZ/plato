
// RFC 9457 Problem Details
export interface ProblemDetails {
  type: string;
  title: string;
  status: number;
  detail?: string;
  instance?: string;
  [key: string]: any;
}

export interface PaginatedResponse<T> {
  data: T[];
  meta: {
    total: number;
    cursor?: string;
    hasMore: boolean;
  };
}

export type EntityId = string;

export interface Mission {
  id: EntityId;
  name: string;
  status: 'Draft' | 'Active' | 'Completed' | 'Failed';
  ownerId: EntityId;
  createdAt: string;
  updatedAt: string;
}

export interface Satellite {
  id: EntityId;
  missionId: EntityId;
  name: string;
  status: 'Nominal' | 'Warning' | 'Critical' | 'Offline';
  telemetry: {
    lat: number;
    lng: number;
    alt: number;
    cpu: number;
    memory: number;
    battery: number;
  };
}

export interface GroundStation {
  id: EntityId;
  name: string;
  lat: number;
  lng: number;
  status: 'Nominal' | 'Maintenance' | 'Offline';
}

export interface DeploymentPlan {
  id: EntityId;
  missionId: EntityId;
  version: string;
  strategy: string;
  status: 'Draft' | 'Approved' | 'Executed';
}

export interface RiskAssessment {
  id: EntityId;
  planId: EntityId;
  score: number;
  severity: 'Low' | 'Medium' | 'High' | 'Critical';
  summary: string;
}

export interface AnalyticsReport {
  id: EntityId;
  name: string;
  category: string;
  createdAt: string;
}

export interface User {
  id: EntityId;
  email: string;
  name: string;
  roles: string[];
}
