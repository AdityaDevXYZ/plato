'use client';
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { getCommWindows, OrbitEvents } from '@/lib/orbit-mock-data'
import { Rocket, LayoutList, Globe, Info, Activity, RadioTower } from 'lucide-react'

export function OrbitalDetails({ asset }: { asset: any }) {
  if (!asset) {
    return (
      <div className="w-96 bg-card flex flex-col items-center justify-center text-muted-foreground p-8 text-center h-full">
        <Globe className="h-12 w-12 mb-4 opacity-20" />
        <p>Select a satellite or ground station to view real-time orbital awareness.</p>
      </div>
    )
  }

  const windows = getCommWindows(asset.id);

  return (
    <div className="w-96 bg-card flex flex-col h-full overflow-y-auto animate-in slide-in-from-right-8 duration-300">
      <div className="p-6 border-b flex-shrink-0">
        <div className="flex items-start justify-between">
          <div>
            <h2 className="text-xl font-bold">{asset.name}</h2>
            <p className="text-sm text-muted-foreground">{asset.id} • {asset.type}</p>
          </div>
          <Badge variant={asset.status === 'Nominal' ? 'success' : 'warning'}>
            {asset.status}
          </Badge>
        </div>
      </div>

      <div className="p-6 space-y-6 flex-1 overflow-y-auto">
        <div className="space-y-4">
          <h3 className="text-sm font-semibold border-b pb-2">Kinematics</h3>
          <div className="grid grid-cols-2 gap-y-4 text-sm">
            <div><span className="text-muted-foreground block text-xs">Altitude</span><span className="font-medium">{asset.alt}</span></div>
            <div><span className="text-muted-foreground block text-xs">Velocity</span><span className="font-medium">{asset.vel}</span></div>
            <div><span className="text-muted-foreground block text-xs">Period</span><span className="font-medium">{asset.period}</span></div>
            <div><span className="text-muted-foreground block text-xs">Inclination</span><span className="font-medium">{asset.inc}</span></div>
            <div className="col-span-2"><span className="text-muted-foreground block text-xs">Coordinates (Lat, Lng)</span><span className="font-medium font-mono">{asset.lat}, {asset.lng}</span></div>
            <div className="col-span-2 flex justify-between items-center bg-muted/50 p-2 rounded border border-border">
              <span className="text-muted-foreground text-xs">Prediction Confidence</span>
              <span className="font-medium text-green-500">{asset.confidence}%</span>
            </div>
          </div>
        </div>

        {asset.type === 'Satellite' && (
          <div className="space-y-3">
            <h3 className="text-sm font-semibold border-b pb-2">Next Comm Windows</h3>
            <div className="space-y-2">
              {windows.map((w, i) => (
                <div key={i} className="flex justify-between items-center text-sm p-2 border rounded-md border-border bg-card">
                  <div className="flex items-center gap-2">
                    <RadioTower className="h-4 w-4 text-primary" />
                    <div>
                      <p className="font-medium">{w.station}</p>
                      <p className="text-xs text-muted-foreground">{w.start} - {w.end} ({w.duration})</p>
                    </div>
                  </div>
                  <Badge variant="outline">{w.signal} Qual</Badge>
                </div>
              ))}
            </div>
          </div>
        )}

        <div className="space-y-3">
          <h3 className="text-sm font-semibold border-b pb-2">Event Log</h3>
          <div className="space-y-3">
            {OrbitEvents.map(ev => (
              <div key={ev.id} className="flex gap-3 text-sm">
                <Info className="h-4 w-4 text-primary flex-shrink-0 mt-0.5" />
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
        <h3 className="text-sm font-semibold mb-3">Quick Actions</h3>
        <Button variant="outline" className="w-full justify-start" size="sm"><Globe className="mr-2 h-4 w-4" /> Open Digital Twin</Button>
        <Button variant="outline" className="w-full justify-start" size="sm"><Rocket className="mr-2 h-4 w-4" /> View Mission</Button>
        <Button variant="outline" className="w-full justify-start" size="sm"><Activity className="mr-2 h-4 w-4" /> Resource Manager</Button>
        <Button variant="outline" className="w-full justify-start" size="sm"><LayoutList className="mr-2 h-4 w-4" /> Generate Deployment Plan</Button>
      </div>
    </div>
  )
}
