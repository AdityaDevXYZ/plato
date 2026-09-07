'use client';
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Progress } from "@/components/ui/progress"
import { getTelemetryHistory, AssetEvents } from '@/lib/twin-mock-data'
import { LineChart, Line, ResponsiveContainer, XAxis, Tooltip } from 'recharts'
import { Rocket, LayoutList, ShieldAlert, GitBranch, Activity, Info, AlertTriangle } from 'lucide-react'

export function AssetDetails({ asset }: { asset: any }) {
  if (!asset) {
    return (
      <div className="w-96 bg-card flex flex-col items-center justify-center text-muted-foreground p-8 text-center h-full">
        <Activity className="h-12 w-12 mb-4 opacity-20" />
        <p>Select an asset from the fleet explorer or visualization to view digital twin details.</p>
      </div>
    )
  }

  const telemetry = getTelemetryHistory(asset.id);

  return (
    <div className="w-96 bg-card flex flex-col h-full overflow-y-auto animate-in slide-in-from-right-8 duration-300">
      <div className="p-6 border-b flex-shrink-0">
        <div className="flex items-start justify-between">
          <div>
            <h2 className="text-xl font-bold">{asset.name}</h2>
            <p className="text-sm text-muted-foreground">{asset.id} • {asset.type}</p>
          </div>
          <Badge variant={asset.status === 'Critical' ? 'destructive' : asset.status === 'Warning' ? 'warning' : 'success'}>
            {asset.status}
          </Badge>
        </div>
      </div>

      <div className="p-6 space-y-6 flex-1 overflow-y-auto">
        
        {asset.type === 'Satellite' && (
          <div className="space-y-4">
            <div>
              <div className="flex justify-between text-sm mb-1"><span className="text-muted-foreground">Battery</span> <span>{asset.battery}%</span></div>
              <Progress value={asset.battery} className={asset.battery < 30 ? 'bg-destructive/20 [&>div]:bg-destructive' : ''} />
            </div>
            <div>
              <div className="flex justify-between text-sm mb-1"><span className="text-muted-foreground">CPU</span> <span>{asset.cpu}%</span></div>
              <Progress value={asset.cpu} className={asset.cpu > 80 ? 'bg-destructive/20 [&>div]:bg-destructive' : ''} />
            </div>
            <div>
              <div className="flex justify-between text-sm mb-1"><span className="text-muted-foreground">Memory</span> <span>{asset.memory}%</span></div>
              <Progress value={asset.memory} />
            </div>
          </div>
        )}

        <div className="grid grid-cols-2 gap-4 text-sm">
          <div>
            <span className="text-muted-foreground block text-xs">Software Release</span>
            <span className="font-medium">{asset.release || 'N/A'}</span>
          </div>
          <div>
            <span className="text-muted-foreground block text-xs">Temperature</span>
            <span className="font-medium">{asset.temp ? `${asset.temp}°C` : 'N/A'}</span>
          </div>
        </div>

        {asset.type === 'Satellite' && (
          <Card className="shadow-none border-border">
            <CardHeader className="p-4 pb-2"><CardTitle className="text-sm">CPU Telemetry (Last 30m)</CardTitle></CardHeader>
            <CardContent className="p-4 pt-0 h-[100px]">
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={telemetry}>
                  <XAxis dataKey="time" hide />
                  <Tooltip contentStyle={{ backgroundColor: 'hsl(var(--card))', borderColor: 'hsl(var(--border))' }} />
                  <Line type="monotone" dataKey="value" stroke="hsl(var(--primary))" strokeWidth={2} dot={false} />
                </LineChart>
              </ResponsiveContainer>
            </CardContent>
          </Card>
        )}

        <div className="space-y-3">
          <h3 className="text-sm font-semibold">Event Stream</h3>
          <div className="space-y-2">
            {AssetEvents.map(ev => (
              <div key={ev.id} className="flex gap-3 text-sm">
                {ev.type === 'info' && <Info className="h-4 w-4 text-primary flex-shrink-0" />}
                {ev.type === 'warning' && <AlertTriangle className="h-4 w-4 text-yellow-500 flex-shrink-0" />}
                {ev.type === 'success' && <Activity className="h-4 w-4 text-green-500 flex-shrink-0" />}
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
        <h3 className="text-sm font-semibold mb-3">Quick Commands</h3>
        <Button variant="outline" className="w-full justify-start" size="sm"><Rocket className="mr-2 h-4 w-4" /> Open Mission Context</Button>
        <Button variant="outline" className="w-full justify-start" size="sm"><LayoutList className="mr-2 h-4 w-4" /> View Deployment Plans</Button>
        <Button variant="outline" className="w-full justify-start" size="sm"><ShieldAlert className="mr-2 h-4 w-4" /> Run Assessment</Button>
        <Button variant="outline" className="w-full justify-start" size="sm"><GitBranch className="mr-2 h-4 w-4" /> Trigger Safe Mode</Button>
      </div>
    </div>
  )
}
