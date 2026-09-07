'use client';
import { Sidebar } from './Sidebar';
import { NotificationCenter } from './NotificationCenter';
import { ConnectionBanner } from './ConnectionBanner';
import { useAuthStore } from '@/lib/auth-store';

export function AppShell({ children }: { children: React.ReactNode }) {
  const user = useAuthStore(state => state.user);

  return (
    <div className="flex h-screen overflow-hidden bg-background">
      <Sidebar />
      <div className="flex-1 flex flex-col min-w-0">
        <ConnectionBanner />
        <header className="h-16 flex items-center justify-between px-6 border-b border-border bg-card shrink-0">
          <div className="flex items-center gap-2">
            <span className="text-sm font-semibold">{user?.orgName || 'Global Aerospace'}</span>
            <span className="text-xs px-2 py-0.5 bg-muted rounded text-muted-foreground font-mono">WORKSPACE</span>
          </div>
          
          <div className="flex items-center gap-4">
            <NotificationCenter />
            <div className="flex items-center gap-2 border-l border-border pl-4">
              <div className="h-8 w-8 rounded bg-primary/20 flex items-center justify-center text-primary text-sm font-bold">
                {user?.name?.charAt(0) || 'O'}
              </div>
            </div>
          </div>
        </header>
        <main className="flex-1 overflow-y-auto p-6 relative">
          {children}
        </main>
      </div>
    </div>
  );
}
