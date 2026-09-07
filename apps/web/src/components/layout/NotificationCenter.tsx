'use client';
import { useState } from 'react';
import { Bell, Info, ShieldAlert, CheckCircle, AlertTriangle } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { useNotificationStore } from '@/lib/stores/notification-store';
import { useRealtimeSubscription } from '@/hooks/useRealtime';
import { NotificationEvent } from '@/lib/api/events';

export function NotificationCenter() {
  const [isOpen, setIsOpen] = useState(false);
  const { notifications, unreadCount, addNotification, markAllAsRead, clearAll } = useNotificationStore();

  // Subscribe to global notification channel
  useRealtimeSubscription<NotificationEvent>('global:notifications', (payload) => {
    addNotification(payload);
  });

  return (
    <div className="relative">
      <Button variant="ghost" size="icon" className="relative h-9 w-9 text-muted-foreground hover:text-foreground" onClick={() => setIsOpen(!isOpen)}>
        <Bell className="h-5 w-5" />
        {unreadCount > 0 && (
          <span className="absolute top-1 right-1 flex h-3 w-3 items-center justify-center rounded-full bg-destructive text-[9px] font-bold text-white">
            {unreadCount > 9 ? '9+' : unreadCount}
          </span>
        )}
      </Button>

      {isOpen && (
        <>
          <div className="fixed inset-0 z-40 bg-transparent" onClick={() => setIsOpen(false)} />
          <div className="absolute right-0 mt-2 w-80 z-50 bg-card border border-border rounded-lg shadow-xl overflow-hidden animate-in fade-in slide-in-from-top-4 duration-200">
            <div className="flex justify-between items-center p-4 border-b border-border bg-muted/30">
              <h3 className="font-semibold text-sm">Notifications</h3>
              <div className="flex gap-2">
                <button onClick={markAllAsRead} className="text-xs text-muted-foreground hover:text-primary transition-colors">Mark read</button>
                <button onClick={clearAll} className="text-xs text-muted-foreground hover:text-destructive transition-colors">Clear</button>
              </div>
            </div>
            
            <div className="max-h-[400px] overflow-y-auto">
              {notifications.length === 0 ? (
                <div className="p-8 text-center text-muted-foreground text-sm flex flex-col items-center">
                  <Bell className="h-8 w-8 opacity-20 mb-2" />
                  <p>No new notifications</p>
                </div>
              ) : (
                <div className="flex flex-col">
                  {notifications.map((notif, i) => (
                    <div key={notif.id || i} className="p-4 border-b border-border last:border-0 hover:bg-muted/30 transition-colors flex gap-3">
                      <div className="mt-0.5 flex-shrink-0">
                        {notif.severity === 'critical' && <ShieldAlert className="h-4 w-4 text-destructive" />}
                        {notif.severity === 'warning' && <AlertTriangle className="h-4 w-4 text-yellow-500" />}
                        {notif.severity === 'success' && <CheckCircle className="h-4 w-4 text-green-500" />}
                        {notif.severity === 'info' && <Info className="h-4 w-4 text-primary" />}
                      </div>
                      <div>
                        <div className="flex justify-between items-start mb-1">
                          <h4 className="text-xs font-semibold">{notif.title}</h4>
                          <span className="text-[10px] text-muted-foreground whitespace-nowrap ml-2">{notif.timestamp}</span>
                        </div>
                        <p className="text-xs text-muted-foreground line-clamp-2">{notif.message}</p>
                        <Badge variant="outline" className="mt-2 text-[9px] px-1.5 py-0">{notif.engine}</Badge>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        </>
      )}
    </div>
  );
}
