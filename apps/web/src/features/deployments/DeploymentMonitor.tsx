'use client';
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Progress } from "@/components/ui/progress"
import { ActiveDeployment } from '@/lib/deployments-mock-data'
import { PlayCircle, Clock, CheckCircle } from 'lucide-react'

export function DeploymentMonitor() {
  return (
    <div className="flex flex-col h-full bg-background border-r overflow-y-auto">
      <div className="p-6 border-b bg-card flex-shrink-0">
        <div className="flex justify-between items-start mb-6">
          <div>
            <h2 className="text-2xl font-bold">Execution Monitor</h2>
            <p className="text-sm text-muted-foreground mt-1">Real-time observability into active deployment waves.</p>
          </div>
          <Badge variant="default" className="text-sm py-1 animate-pulse shadow-[0_0_10px_rgba(59,130,246,0.5)]">
            LIVE
          </Badge>
        </div>
        
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mt-6">
          <Card className="shadow-none border-border bg-muted/20">
            <CardHeader className="p-4 pb-2"><CardTitle className="text-xs font-medium text-muted-foreground uppercase">Phase</CardTitle></CardHeader>
            <CardContent className="p-4 pt-0">
              <div className="text-lg font-bold truncate">{ActiveDeployment.currentState}</div>
            </CardContent>
          </Card>
          <Card className="shadow-none border-border bg-muted/20">
            <CardHeader className="p-4 pb-2"><CardTitle className="text-xs font-medium text-muted-foreground uppercase">Progress</CardTitle></CardHeader>
            <CardContent className="p-4 pt-0">
              <div className="text-lg font-bold text-primary">{ActiveDeployment.stats.completedTargets} / {ActiveDeployment.stats.completedTargets + ActiveDeployment.stats.remainingTargets}</div>
            </CardContent>
          </Card>
          <Card className="shadow-none border-border bg-muted/20">
            <CardHeader className="p-4 pb-2"><CardTitle className="text-xs font-medium text-muted-foreground uppercase">Elapsed</CardTitle></CardHeader>
            <CardContent className="p-4 pt-0">
              <div className="text-lg font-bold">{ActiveDeployment.stats.elapsedTime}</div>
            </CardContent>
          </Card>
          <Card className="shadow-none border-border bg-muted/20">
            <CardHeader className="p-4 pb-2"><CardTitle className="text-xs font-medium text-muted-foreground uppercase">ETA</CardTitle></CardHeader>
            <CardContent className="p-4 pt-0">
              <div className="text-lg font-bold">{ActiveDeployment.stats.estimatedRemaining}</div>
            </CardContent>
          </Card>
        </div>
      </div>
      
      <div className="p-6 flex-1 bg-muted/10">
        <h3 className="font-semibold text-sm mb-4">Execution Waves</h3>
        <div className="space-y-4">
          {ActiveDeployment.waves.map((wave, i) => (
            <Card key={wave.id} className="shadow-sm border-border">
              <CardContent className="p-5">
                <div className="flex justify-between items-center mb-4">
                  <div className="flex items-center gap-3">
                    <div className="h-8 w-8 rounded-full bg-secondary flex items-center justify-center text-sm font-bold flex-shrink-0">
                      {i + 1}
                    </div>
                    <div>
                      <h4 className="font-medium">{wave.name}</h4>
                      <p className="text-xs text-muted-foreground">{wave.targets} Targets • {wave.retries} Retries</p>
                    </div>
                  </div>
                  <div className="flex items-center gap-3">
                    <span className="text-sm font-medium">{wave.progress}%</span>
                    {wave.status === 'Running' && <PlayCircle className="h-5 w-5 text-primary" />}
                    {wave.status === 'Pending' && <Clock className="h-5 w-5 text-muted-foreground" />}
                    {wave.status === 'Completed' && <CheckCircle className="h-5 w-5 text-green-500" />}
                  </div>
                </div>
                <Progress value={wave.progress} className="h-2" />
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    </div>
  )
}
