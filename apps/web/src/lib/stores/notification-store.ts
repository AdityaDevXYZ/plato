
import { create } from 'zustand';
import { NotificationEvent } from '../api/events';

interface NotificationState {
  notifications: NotificationEvent[];
  unreadCount: number;
  addNotification: (notification: NotificationEvent) => void;
  markAsRead: (id: string) => void;
  markAllAsRead: () => void;
  clearAll: () => void;
}

export const useNotificationStore = create<NotificationState>((set) => ({
  notifications: [],
  unreadCount: 0,
  addNotification: (notification) => set((state) => {
    // Deduplicate
    if (state.notifications.some(n => n.id === notification.id)) return state;
    
    const maxNotifications = 100;
    const newQueue = [notification, ...state.notifications].slice(0, maxNotifications);
    
    return {
      notifications: newQueue,
      unreadCount: state.unreadCount + 1
    };
  }),
  markAsRead: (id) => set((state) => ({
    unreadCount: Math.max(0, state.unreadCount - 1)
  })),
  markAllAsRead: () => set({ unreadCount: 0 }),
  clearAll: () => set({ notifications: [], unreadCount: 0 })
}));
