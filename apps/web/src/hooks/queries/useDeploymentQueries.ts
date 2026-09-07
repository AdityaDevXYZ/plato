
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { apiClient } from '@/lib/api/client';

export const useDeploymentQueue = () => {
  return useQuery({
    queryKey: ['deployment', 'queue'],
    queryFn: () => apiClient<any[]>('/deployments/queue'),
  });
};

export const useDeploymentDetails = (deploymentId: string | null) => {
  return useQuery({
    queryKey: ['deployment', 'details', deploymentId],
    queryFn: () => apiClient<any>(`/deployments/${deploymentId}`),
    enabled: !!deploymentId,
  });
};

export const useOperatorControls = () => {
  const queryClient = useQueryClient();
  
  const pause = useMutation({
    mutationFn: (id: string) => apiClient(`/deployments/${id}/pause`, { method: 'POST' }),
    onSuccess: (_, id) => queryClient.invalidateQueries({ queryKey: ['deployment', 'details', id] })
  });

  const cancel = useMutation({
    mutationFn: (id: string) => apiClient(`/deployments/${id}/cancel`, { method: 'POST' }),
    onSuccess: (_, id) => queryClient.invalidateQueries({ queryKey: ['deployment', 'details', id] })
  });
  
  const rollback = useMutation({
    mutationFn: (id: string) => apiClient(`/deployments/${id}/rollback`, { method: 'POST' }),
    onSuccess: (_, id) => queryClient.invalidateQueries({ queryKey: ['deployment', 'details', id] })
  });

  return { pause, cancel, rollback };
};
