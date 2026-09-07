
import { config } from './config';

type MessageHandler = (payload: any) => void;

export class RealtimeClient {
  private ws: WebSocket | null = null;
  private url: string;
  private subscriptions = new Map<string, Set<MessageHandler>>();
  private reconnectAttempts = 0;
  private isConnecting = false;
  private heartbeatInterval: NodeJS.Timeout | null = null;

  constructor(url: string = config.ws.url) {
    this.url = url;
  }

  private notifyConnectionState(isConnected: boolean) {
    const handlers = this.subscriptions.get('system:connection');
    if (handlers) {
      handlers.forEach(handler => handler(isConnected));
    }
  }

  connect() {
    if (this.ws?.readyState === WebSocket.OPEN || this.isConnecting) return;
    this.isConnecting = true;
    
    try {
      this.ws = new WebSocket(this.url);
      
      this.ws.onopen = () => {
        this.isConnecting = false;
        this.reconnectAttempts = 0;
        this.notifyConnectionState(true);
        this.startHeartbeat();
        this.resubscribeAll();
        console.log('[WS] Connected');
      };

      this.ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          if (data.type === 'pong') return; 
          
          const handlers = this.subscriptions.get(data.channel);
          if (handlers) {
            handlers.forEach(handler => handler(data.payload));
          }
        } catch (e) {
          console.error('[WS] Parse error:', e);
        }
      };

      this.ws.onclose = () => {
        this.cleanup();
        this.notifyConnectionState(false);
        this.scheduleReconnect();
      };

      this.ws.onerror = () => {
        // Will trigger onclose
      };
    } catch (e) {
      this.isConnecting = false;
      this.notifyConnectionState(false);
      this.scheduleReconnect();
    }
  }

  subscribe(channel: string, handler: MessageHandler) {
    if (!this.subscriptions.has(channel)) {
      this.subscriptions.set(channel, new Set());
      if (channel !== 'system:connection') {
        this.sendSubscription(channel, 'subscribe');
      }
    }
    this.subscriptions.get(channel)!.add(handler);
    
    // Immediate state return for connection channel
    if (channel === 'system:connection' && this.ws?.readyState === WebSocket.OPEN) {
      handler(true);
    }
    
    return () => this.unsubscribe(channel, handler);
  }

  private unsubscribe(channel: string, handler: MessageHandler) {
    const handlers = this.subscriptions.get(channel);
    if (handlers) {
      handlers.delete(handler);
      if (handlers.size === 0) {
        this.subscriptions.delete(channel);
        if (channel !== 'system:connection') {
          this.sendSubscription(channel, 'unsubscribe');
        }
      }
    }
  }

  private sendSubscription(channel: string, action: 'subscribe' | 'unsubscribe') {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify({ action, channel }));
    }
  }

  private resubscribeAll() {
    for (const channel of this.subscriptions.keys()) {
      if (channel !== 'system:connection') {
        this.sendSubscription(channel, 'subscribe');
      }
    }
  }

  private startHeartbeat() {
    this.heartbeatInterval = setInterval(() => {
      if (this.ws?.readyState === WebSocket.OPEN) {
        this.ws.send(JSON.stringify({ type: 'ping' }));
      }
    }, 30000);
  }

  private cleanup() {
    this.isConnecting = false;
    if (this.heartbeatInterval) clearInterval(this.heartbeatInterval);
  }

  private scheduleReconnect() {
    if (this.reconnectAttempts >= config.ws.maxRetries) {
      console.error('[WS] Max reconnect attempts reached');
      return;
    }
    
    const backoff = Math.min(1000 * Math.pow(2, this.reconnectAttempts), 30000);
    this.reconnectAttempts++;
    console.log(`[WS] Reconnecting in ${backoff}ms...`);
    setTimeout(() => this.connect(), backoff);
  }

  disconnect() {
    this.cleanup();
    this.notifyConnectionState(false);
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }
}

export const realtime = new RealtimeClient();
