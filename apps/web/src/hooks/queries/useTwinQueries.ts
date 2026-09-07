
import { useQuery } from '@tanstack/react-query';
import { apiClient } from '@/lib/api/client';

export const useFleetExplorer = () => {
  return useQuery({
    queryKey: ['twin', 'fleet'],
    queryFn: () => apiClient<any[]>('/twin/fleet'),
    staleTime: 60000,
  });
};

export const useAssetDetails = (assetId: string | null) => {
  return useQuery({
    queryKey: ['twin', 'asset', assetId],
    queryFn: () => apiClient<any>(`/twin/assets/${assetId}`),
    enabled: !!assetId,
  });
};

export const useOrbitalPositions = () => {
  return useQuery({
    queryKey: ['orbital', 'positions'],
    queryFn: () => apiClient<any[]>('/orbital/positions'),
    refetchInterval: 10000, // Poll every 10s as fallback if WS fails
  });
};
