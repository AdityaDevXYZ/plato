
import { QueryClient } from '@tanstack/react-query';
import { ApiError } from './api/client';

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: (failureCount, error) => {
        // Do not retry 401, 403, 404
        if (error instanceof ApiError) {
          if ([401, 403, 404].includes(error.problem.status)) return false;
        }
        return failureCount < 3;
      },
      staleTime: 5 * 60 * 1000, // 5 mins
      refetchOnWindowFocus: false,
    },
  },
});
