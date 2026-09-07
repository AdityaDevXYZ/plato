'use client';
import { cn } from '@/lib/utils'
import { Building, Users, UsersRound, KeyRound, Key, ShieldCheck, Link, Webhook, Settings2, Lock, CreditCard, FileSignature } from 'lucide-react'

const navItems = [
  { id: 'org', name: 'Organization', icon: Building },
  { id: 'users', name: 'Users', icon: Users },
  { id: 'teams', name: 'Teams', icon: UsersRound },
  { id: 'roles', name: 'Roles & Permissions', icon: KeyRound },
  { id: 'apikeys', name: 'API Keys', icon: Key },
  { id: 'audit', name: 'Audit Logs', icon: ShieldCheck },
  { id: 'integrations', name: 'Integrations', icon: Link },
  { id: 'webhooks', name: 'Webhooks', icon: Webhook },
  { id: 'settings', name: 'Platform Settings', icon: Settings2 },
  { id: 'security', name: 'Security', icon: Lock },
  { id: 'billing', name: 'Billing', icon: CreditCard },
  { id: 'licensing', name: 'Licensing', icon: FileSignature },
];

export function AdminSidebar({ activeTab, setActiveTab }: { activeTab: string, setActiveTab: (id: string) => void }) {
  return (
    <div className="flex flex-col h-full w-64 bg-card border-r border-border overflow-y-auto">
      <div className="p-6 border-b border-border">
        <h2 className="text-xl font-bold tracking-tight">Administration</h2>
        <p className="text-xs text-muted-foreground mt-1">Enterprise Platform Config</p>
      </div>
      
      <div className="flex-1 py-4">
        <nav className="space-y-1 px-3">
          {navItems.map((item) => (
            <button
              key={item.id}
              onClick={() => setActiveTab(item.id)}
              className={cn(
                "group flex w-full items-center px-3 py-2 text-sm font-medium rounded-md hover:bg-accent hover:text-accent-foreground transition-colors",
                activeTab === item.id ? "bg-accent text-accent-foreground" : "text-muted-foreground"
              )}
            >
              <item.icon className="mr-3 h-4 w-4 flex-shrink-0" />
              {item.name}
            </button>
          ))}
        </nav>
      </div>
    </div>
  )
}
