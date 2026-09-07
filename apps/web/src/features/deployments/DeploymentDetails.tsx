'use client';
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { ActiveDeployment, PostDeploymentStats } from '@/lib/deployments-mock-data'
import { Info, PlayCircle, AlertTriangle, CheckCircle, Pause, XCircle, RotateCcw } from 'lucide-react'

export function DeploymentDetails({ id }: { id: string | null }) {
  if (!id) {
    return (
      <div className="flex flex-col h-full bg-card items-center justify-center p-6 text-center text-muted-foreground">
        <PlayCircle className="h-12 w-12 mb-4 opacity-20" />
        <p>Select a deployment to view telemetry and command controls.</p>
      </div>
    )
  }

  const isRunning = id === 'dep-001';
  const isCompleted = id === 'dep-003';

  return (
    <div className="flex flex-col h-full bg-card overflow-y-auto animate-in slide-in-from-right-8 duration-300">
      <div className="p-6 border-b flex-shrink-0">
        <h2 className="text-xl font-bold truncate pr-2">{ActiveDeployment.mission}</h2>
        <p className="text-sm text-muted-foreground mt-1">ID: {id} • {ActiveDeployment.strategy}</p>
      </div>

      <div className="p-6 space-y-6 flex-1 overflow-y-auto">
        <div className="space-y-4">
          <div className="flex justify-between items-center border-b border-border pb-2">
            <span className="text-sm text-muted-foreground">Plan Version</span>
            <span className="text-sm font-medium">{ActiveDeployment.planVersion}</span>
          </div>
          <div className="flex justify-between items-center border-b border-border pb-2">
            <span className="text-sm text-muted-foreground">Initiated By</span>
            <span className="text-sm font-medium">{ActiveDeployment.initiatedBy}</span>
          </div>
          <div className="flex justify-between items-center border-b border-border pb-2">
            <span className="text-sm text-muted-foreground">Rollback</span>
            <Badge variant="success" className="bg-green-500/20 text-green-500 hover:bg-green-500/30">{ActiveDeployment.rollbackReadiness}</Badge>
          </div>
        </div>

        {isCompleted && (
          <div className="space-y-3 bg-muted/30 p-4 rounded-lg border border-border">
            <h3 className="text-sm font-semibold">Post-Deployment Summary</h3>
            <div className="grid grid-cols-2 gap-2 text-sm">
              <div className="text-muted-foreground">Success Rate</div><div className="text-right font-medium text-green-500">{PostDeploymentStats.successRate}%</div>
              <div className="text-muted-foreground">Failed Assets</div><div className="text-right font-medium">{PostDeploymentStats.failedAssets}</div>
              <div className="text-muted-foreground">Retries</div><div className="text-right font-medium">{PostDeploymentStats.retryCount}</div>
              <div className="text-muted-foreground">Duration</div><div className="text-right font-medium">{PostDeploymentStats.duration}</div>
            </div>
          </div>
        )}

        <div className="space-y-3">
          <h3 className="text-sm font-semibold">Event Stream</h3>
          <div className="space-y-3">
            {ActiveDeployment.events.map(ev => (
              <div key={ev.id} className="flex gap-3 text-sm">
                {ev.type === 'info' && <Info className="h-4 w-4 text-primary flex-shrink-0 mt-0.5" />}
                {ev.type === 'warning' && <AlertTriangle className="h-4 w-4 text-yellow-500 flex-shrink-0 mt-0.5" />}
                {ev.type === 'success' && <CheckCircle className="h-4 w-4 text-green-500 flex-shrink-0 mt-0.5" />}
                <div>
                  <p>{ev.message}</p>
                  <p className="text-xs text-muted-foreground">{ev.time}</p>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>

      <div className="p-6 border-t bg-muted/30 space-y-2 flex-shrink-0">
        <h3 className="text-sm font-semibold mb-3">Operator Controls</h3>
        <div className="grid grid-cols-2 gap-2">
          <Button variant="outline" size="sm" disabled={!isRunning} className="w-full"><Pause className="mr-2 h-4 w-4" /> Pause</Button>
          <Button variant="destructive" size="sm" disabled={!isRunning} className="w-full"><XCircle className="mr-2 h-4 w-4" /> Cancel</Button>
        </div>
        <Button variant="outline" size="sm" disabled={!isRunning} className="w-full border-destructive text-destructive hover:bg-destructive hover:text-destructive-foreground"><RotateCcw className="mr-2 h-4 w-4" /> Initiate Rollback</Button>
      </div>
    </div>
  )
}
