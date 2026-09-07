'use client';
import { useState } from 'react';
import { AdminSidebar } from '@/features/admin/AdminSidebar'
import { UsersTab } from '@/features/admin/tabs/UsersTab'
import { RolesTab } from '@/features/admin/tabs/RolesTab'
import { AuditTab } from '@/features/admin/tabs/AuditTab'
import { IntegrationsTab } from '@/features/admin/tabs/IntegrationsTab'
import { PlaceholderTab } from '@/features/admin/tabs/PlaceholderTab'

export default function AdminPage() {
  const [activeTab, setActiveTab] = useState('users');

  return (
    <div className="flex h-[calc(100vh-8rem)] -mx-6 -mb-6 overflow-hidden bg-background border-t animate-in fade-in duration-500">
      <AdminSidebar activeTab={activeTab} setActiveTab={setActiveTab} />
      
      <div className="flex-1 min-w-0 overflow-y-auto bg-muted/5">
        {activeTab === 'users' && <UsersTab />}
        {activeTab === 'roles' && <RolesTab />}
        {activeTab === 'audit' && <AuditTab />}
        {activeTab === 'integrations' && <IntegrationsTab />}
        {activeTab === 'webhooks' && <IntegrationsTab />}
        
        {['org', 'teams', 'apikeys', 'settings', 'security', 'billing', 'licensing'].includes(activeTab) && (
          <PlaceholderTab 
            title={activeTab.charAt(0).toUpperCase() + activeTab.slice(1)} 
            desc="Enterprise configuration module." 
          />
        )}
      </div>
    </div>
  )
}
