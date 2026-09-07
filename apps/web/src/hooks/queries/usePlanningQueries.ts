
import { useQuery, useMutation } from '@tanstack/react-query';
import { apiClient } from '@/lib/api/client';

export const useMissions = () => {
  return useQuery({
    queryKey: ['planning', 'missions'],
    queryFn: () => apiClient<any[]>('/planning/missions'),
  });
};

export const useDeploymentPlans = (missionId: string | null) => {
  return useQuery({
    queryKey: ['planning', 'plans', missionId],
    queryFn: () => apiClient<any[]>(`/planning/missions/${missionId}/plans`),
    enabled: !!missionId,
  });
};

export const useRiskReport = (planId: string | null) => {
  return useQuery({
    queryKey: ['risk', 'report', planId],
    queryFn: () => apiClient<any>(`/risk/assessments/${planId}`),
    enabled: !!planId,
  });
};
