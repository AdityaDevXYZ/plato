'use client';
import { Button } from "@/components/ui/button"
import { RolesData } from '@/lib/admin-mock-data'
import { Plus, Shield } from 'lucide-react'

export function RolesTab() {
  return (
    <div className="p-6 space-y-6 max-w-5xl">
      <div className="flex justify-between items-center">
        <div>
          <h2 className="text-2xl font-bold">Roles & Permissions</h2>
          <p className="text-sm text-muted-foreground mt-1">Configure RBAC policies for platform access control.</p>
        </div>
        <Button><Plus className="h-4 w-4 mr-2" /> Create Custom Role</Button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {RolesData.map((r, i) => (
          <div key={i} className="bg-card border border-border rounded-lg p-5 flex flex-col">
            <div className="flex items-center gap-2 mb-2">
              <Shield className="h-5 w-5 text-primary" />
              <h3 className="font-semibold">{r.name}</h3>
            </div>
            <p className="text-sm text-muted-foreground flex-1">{r.desc}</p>
            <div className="mt-4 pt-4 border-t border-border flex justify-between items-center">
              <span className="text-xs text-muted-foreground">{r.users} users</span>
              <Button variant="link" size="sm" className="h-auto p-0">View Matrix</Button>
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}
