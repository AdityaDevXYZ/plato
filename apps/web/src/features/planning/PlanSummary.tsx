'use client';
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { ValidationResults } from '@/lib/planning-mock-data'
import { AlertTriangle, CheckCircle, Info, ShieldAlert, GitBranch, LayoutList } from 'lucide-react'

export function PlanSummary() {
  return (
    <div className="flex flex-col h-full bg-card overflow-y-auto">
      <div className="p-6 border-b">
        <h2 className="font-semibold mb-4">Plan Summary</h2>
        <div className="space-y-4 text-sm">
          <div className="flex justify-between border-b border-border pb-2"><span className="text-muted-foreground">Target Assets</span><span className="font-medium">64 Satellites</span></div>
          <div className="flex justify-between border-b border-border pb-2"><span className="text-muted-foreground">Execution Waves</span><span className="font-medium">3 Phases</span></div>
          <div className="flex justify-between border-b border-border pb-2"><span className="text-muted-foreground">Est. Duration</span><span className="font-medium">4h 30m</span></div>
          <div className="flex justify-between border-b border-border pb-2"><span className="text-muted-foreground">Rollback</span><span className="font-medium text-green-500">Available</span></div>
          <div className="flex justify-between border-b border-border pb-2"><span className="text-muted-foreground">Status</span><Badge variant="outline">Draft</Badge></div>
        </div>
      </div>

      <div className="p-6 border-b border-border">
        <h3 className="font-semibold text-sm mb-4">Validation</h3>
        <div className="space-y-3">
          {ValidationResults.map((v, i) => (
            <div key={i} className="flex gap-3 text-sm p-3 bg-muted/50 rounded-lg border border-border">
              {v.type === 'warning' && <AlertTriangle className="h-4 w-4 text-yellow-500 flex-shrink-0" />}
              {v.type === 'info' && <Info className="h-4 w-4 text-primary flex-shrink-0" />}
              {v.type === 'success' && <CheckCircle className="h-4 w-4 text-green-500 flex-shrink-0" />}
              <p className="text-xs leading-relaxed">{v.message}</p>
            </div>
          ))}
        </div>
      </div>

      <div className="p-6 space-y-2">
        <h3 className="text-sm font-semibold mb-3">Quick Actions</h3>
        <Button variant="outline" className="w-full justify-start text-sm"><ShieldAlert className="mr-2 h-4 w-4" /> Run Risk Assessment</Button>
        <Button variant="outline" className="w-full justify-start text-sm"><GitBranch className="mr-2 h-4 w-4" /> Preview Coordination Waves</Button>
        <Button variant="outline" className="w-full justify-start text-sm"><LayoutList className="mr-2 h-4 w-4" /> Export Plan Template</Button>
      </div>
    </div>
  )
}
