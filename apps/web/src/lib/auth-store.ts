
import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { setAccessToken } from './api/client';

export type Role = 'Administrator' | 'Mission Manager' | 'Operator' | 'Engineer' | 'Read Only';

export interface UserSession {
  id: string;
  email: string;
  name: string;
  roles: Role[];
  orgId: string;
  orgName: string;
}

interface AuthState {
  user: UserSession | null;
  isAuthenticated: boolean;
  login: (user: UserSession, token: string) => void;
  logout: () => void;
  hasRole: (role: Role) => boolean;
  switchOrg: (orgId: string, orgName: string) => void;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set, get) => ({
      user: null,
      isAuthenticated: false,
      login: (user, token) => {
        setAccessToken(token);
        set({ user, isAuthenticated: true });
      },
      logout: () => {
        setAccessToken(null);
        set({ user: null, isAuthenticated: false });
        // Clean up WS or other connections if needed
      },
      hasRole: (role) => {
        const { user } = get();
        if (!user) return false;
        if (user.roles.includes('Administrator')) return true; // Admin has all rights
        return user.roles.includes(role);
      },
      switchOrg: (orgId, orgName) => {
        set((state) => ({
          user: state.user ? { ...state.user, orgId, orgName } : null
        }));
      }
    }),
    {
      name: 'plato-auth-storage',
      partialize: (state) => ({ user: state.user, isAuthenticated: state.isAuthenticated }),
      onRehydrateStorage: () => (state) => {
        // Re-sync access token with api client if needed, though secure cookies are preferred
        // We assume token might be refreshed on next API call if expired
      }
    }
  )
);
