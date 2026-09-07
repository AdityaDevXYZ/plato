import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { SystemHealth as Data } from '@/lib/mock-data'
import { Badge } from "@/components/ui/badge"

export function SystemHealth() {
  return (
    <Card className="col-span-1">
      <CardHeader>
        <CardTitle>System Health</CardTitle>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          {Data.map((sys, i) => (
            <div key={i} className="flex items-center justify-between">
              <span className="text-sm font-medium">{sys.service}</span>
              <div className="flex items-center gap-3">
                <span className="text-xs text-muted-foreground">{sys.uptime}</span>
                <Badge variant={sys.status === 'Operational' ? 'success' : 'destructive'}>
                  {sys.status}
                </Badge>
              </div>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  )
}
