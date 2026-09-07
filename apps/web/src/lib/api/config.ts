
export const config = {
  env: process.env.NEXT_PUBLIC_ENV || 'development',
  api: {
    baseUrl: process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api/v1',
    timeout: 10000,
  },
  ws: {
    url: process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:8080/ws',
    reconnectInterval: 5000,
    maxRetries: 5,
  }
};
