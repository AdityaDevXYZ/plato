
import { useEffect, useState, useRef } from 'react';
import { realtime } from '@/lib/api/realtime';

export function useRealtimeSubscription<T>(channel: string, onMessage: (payload: T) => void, dependencies: any[] = []) {
  const handlerRef = useRef(onMessage);
  
  useEffect(() => {
    handlerRef.current = onMessage;
  }, [onMessage]);

  useEffect(() => {
    const handler = (payload: T) => {
      handlerRef.current(payload);
    };

    const unsubscribe = realtime.subscribe(channel, handler);
    return () => unsubscribe();
  }, [channel, ...dependencies]);
}

export function useConnectionState() {
  const [isConnected, setIsConnected] = useState(false);

  useEffect(() => {
    const handleStateChange = (state: boolean) => setIsConnected(state);
    
    // In a real implementation, realtime client would emit connection events.
    // Assuming realtime client is augmented to support event listeners for state.
    const sub = realtime.subscribe('system:connection', handleStateChange);
    
    return () => sub();
  }, []);

  return isConnected;
}
