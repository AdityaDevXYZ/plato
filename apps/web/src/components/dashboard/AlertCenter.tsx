import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Alerts } from '@/lib/mock-data'
import { Badge } from "@/components/ui/badge"
import { ShieldAlert, Info, AlertTriangle } from 'lucide-react'

export function AlertCenter() {
  return (
    <Card className="col-span-1 lg:col-span-1 xl:col-span-2">
      <CardHeader>
        <CardTitle>Alert Center</CardTitle>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          {Alerts.map((alert) => (
            <div key={alert.id} className="flex items-center gap-4 rounded-lg border border-border p-3">
              {alert.severity === 'Critical' && <ShieldAlert className="h-5 w-5 text-destructive flex-shrink-0" />}
              {alert.severity === 'Warning' && <AlertTriangle className="h-5 w-5 text-yellow-500 flex-shrink-0" />}
              {alert.severity === 'Information' && <Info className="h-5 w-5 text-primary flex-shrink-0" />}
              
              <div className="flex-1 overflow-hidden">
                <p className="text-sm font-medium truncate">{alert.message}</p>
              </div>
              <div className="flex items-center gap-2 flex-shrink-0">
                <Badge variant={
                  alert.severity === 'Critical' ? 'destructive' :
                  alert.severity === 'Warning' ? 'warning' : 'default'
                }>{alert.severity}</Badge>
                <span className="text-xs text-muted-foreground whitespace-nowrap">{alert.time}</span>
              </div>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  )
}
