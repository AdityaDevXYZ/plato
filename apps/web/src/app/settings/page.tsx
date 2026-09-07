'use client';
import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { useAuthStore } from '@/lib/auth-store';
import { ShieldCheck, User, Laptop, Smartphone, Globe, LogOut } from 'lucide-react';

export default function SettingsPage() {
  const { user, logout } = useAuthStore();
  const [activeTab, setActiveTab] = useState<'profile' | 'sessions'>('profile');

  if (!user) return <div className="p-8">Not authenticated</div>;

  return (
    <div className="flex h-[calc(100vh-8rem)] -mx-6 -mb-6 overflow-hidden bg-background border-t animate-in fade-in duration-500">
      
      {/* Settings Sidebar */}
      <div className="w-64 bg-card border-r border-border p-4">
        <h2 className="text-lg font-bold mb-4">Account Settings</h2>
        <nav className="space-y-1">
          <button onClick={() => setActiveTab('profile')} className={`w-full flex items-center px-3 py-2 text-sm rounded-md ${activeTab === 'profile' ? 'bg-accent text-accent-foreground' : 'text-muted-foreground hover:bg-accent hover:text-accent-foreground'}`}>
            <User className="h-4 w-4 mr-2" /> Profile
          </button>
          <button onClick={() => setActiveTab('sessions')} className={`w-full flex items-center px-3 py-2 text-sm rounded-md ${activeTab === 'sessions' ? 'bg-accent text-accent-foreground' : 'text-muted-foreground hover:bg-accent hover:text-accent-foreground'}`}>
            <ShieldCheck className="h-4 w-4 mr-2" /> Security & Sessions
          </button>
        </nav>
      </div>

      {/* Main Content */}
      <div className="flex-1 p-8 overflow-y-auto">
        {activeTab === 'profile' && (
          <div className="max-w-2xl space-y-8">
            <div>
              <h3 className="text-2xl font-bold">Profile</h3>
              <p className="text-sm text-muted-foreground">Manage your personal information.</p>
            </div>
            
            <div className="flex items-center gap-6">
              <div className="h-24 w-24 rounded-full bg-primary/20 text-primary flex items-center justify-center text-3xl font-bold">
                {user.name.charAt(0)}
              </div>
              <Button variant="outline">Change Avatar</Button>
            </div>

            <div className="grid grid-cols-2 gap-6">
              <div className="space-y-2">
                <label className="text-sm font-medium">Full Name</label>
                <div className="p-2 bg-muted/50 border border-border rounded text-sm">{user.name}</div>
              </div>
              <div className="space-y-2">
                <label className="text-sm font-medium">Email Address</label>
                <div className="p-2 bg-muted/50 border border-border rounded text-sm">{user.email}</div>
              </div>
              <div className="space-y-2">
                <label className="text-sm font-medium">Organization</label>
                <div className="p-2 bg-muted/50 border border-border rounded text-sm">{user.orgName}</div>
              </div>
              <div className="space-y-2">
                <label className="text-sm font-medium">Roles</label>
                <div className="p-2 bg-muted/50 border border-border rounded text-sm">{user.roles.join(', ')}</div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'sessions' && (
          <div className="max-w-2xl space-y-8">
            <div>
              <h3 className="text-2xl font-bold">Security & Sessions</h3>
              <p className="text-sm text-muted-foreground">Manage multi-factor authentication and active devices.</p>
            </div>

            <div className="bg-card border border-border p-5 rounded-lg flex justify-between items-center">
              <div>
                <h4 className="font-semibold text-sm">Multi-Factor Authentication</h4>
                <p className="text-xs text-muted-foreground mt-1">Protect your account with TOTP (Authenticator App).</p>
              </div>
              <Button variant="outline" className="border-green-500 text-green-500 hover:bg-green-500 hover:text-white">Enabled</Button>
            </div>

            <div>
              <h4 className="font-semibold text-sm mb-4">Active Sessions</h4>
              <div className="space-y-3">
                <div className="bg-muted/30 border border-border p-4 rounded-lg flex justify-between items-center">
                  <div className="flex items-center gap-4">
                    <Laptop className="h-6 w-6 text-primary" />
                    <div>
                      <p className="text-sm font-medium">MacBook Pro • Chrome</p>
                      <p className="text-xs text-muted-foreground flex items-center mt-1">
                        <Globe className="h-3 w-3 mr-1" /> 192.168.1.100 • Current Session
                      </p>
                    </div>
                  </div>
                  <span className="text-xs font-bold text-green-500">Active Now</span>
                </div>

                <div className="bg-card border border-border p-4 rounded-lg flex justify-between items-center">
                  <div className="flex items-center gap-4">
                    <Smartphone className="h-6 w-6 text-muted-foreground" />
                    <div>
                      <p className="text-sm font-medium">iPhone 14 Pro • Safari</p>
                      <p className="text-xs text-muted-foreground flex items-center mt-1">
                        <Globe className="h-3 w-3 mr-1" /> 10.0.0.52 • Last active 2 hours ago
                      </p>
                    </div>
                  </div>
                  <Button variant="ghost" size="sm" className="text-destructive hover:bg-destructive/10"><LogOut className="h-4 w-4 mr-2" /> Revoke</Button>
                </div>
              </div>
            </div>
            
            <div className="pt-8 border-t border-border">
              <Button variant="destructive" onClick={() => { logout(); window.location.href = '/login'; }}>
                Sign out of all devices
              </Button>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}
