'use client';
import { Input } from "@/components/ui/input"
import { Badge } from "@/components/ui/badge"
import { AuditLogs } from '@/lib/admin-mock-data'
import { Search, Download } from 'lucide-react'
import { Button } from "@/components/ui/button"

export function AuditTab() {
  return (
    <div className="p-6 space-y-6 max-w-5xl">
      <div className="flex justify-between items-center">
        <div>
          <h2 className="text-2xl font-bold">Audit Logs</h2>
          <p className="text-sm text-muted-foreground mt-1">Immutable security and activity history across the platform.</p>
        </div>
        <Button variant="outline"><Download className="h-4 w-4 mr-2" /> Export CSV</Button>
      </div>

      <div className="flex justify-between items-center bg-card p-2 rounded-md border border-border">
        <div className="relative w-72">
          <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input placeholder="Search logs..." className="pl-9 h-9" />
        </div>
      </div>

      <div className="border border-border rounded-md bg-card overflow-hidden">
        <table className="w-full text-sm text-left">
          <thead className="text-xs text-muted-foreground uppercase bg-muted/50 border-b border-border">
            <tr>
              <th className="px-4 py-3">Timestamp (UTC)</th>
              <th className="px-4 py-3">Action</th>
              <th className="px-4 py-3">Actor</th>
              <th className="px-4 py-3">Target / Resource</th>
              <th className="px-4 py-3">Status</th>
            </tr>
          </thead>
          <tbody>
            {AuditLogs.map(log => (
              <tr key={log.id} className="border-b border-border last:border-0 hover:bg-muted/20">
                <td className="px-4 py-3 text-muted-foreground whitespace-nowrap">{log.time}</td>
                <td className="px-4 py-3 font-medium">{log.action}</td>
                <td className="px-4 py-3">{log.actor}</td>
                <td className="px-4 py-3 font-mono text-xs text-muted-foreground">{log.target}</td>
                <td className="px-4 py-3">
                  <Badge variant={log.status === 'Success' ? 'outline' : 'destructive'} className={log.status === 'Success' ? 'text-green-500 border-green-500/20 bg-green-500/10' : ''}>
                    {log.status}
                  </Badge>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  )
}
