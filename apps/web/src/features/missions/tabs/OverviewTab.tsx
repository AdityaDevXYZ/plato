import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { MetricCard } from "@/components/dashboard/MetricCard"
import { Activity, Satellite, ShieldAlert } from "lucide-react"

export function OverviewTab({ mission }: { mission: any }) {
  return (
    <div className="space-y-4 animate-in fade-in duration-500">
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <MetricCard title="Total Assets" value={mission.satellites} icon={<Satellite className="h-4 w-4" />} />
        <MetricCard title="Current Release" value={mission.release} />
        <MetricCard title="Mission Health" value={`${mission.health}%`} icon={<Activity className="h-4 w-4 text-primary" />} />
        <MetricCard title="Risk Profile" value={mission.risk} icon={<ShieldAlert className="h-4 w-4 text-green-500" />} />
      </div>
      <div className="grid gap-4 md:grid-cols-2">
        <Card>
          <CardHeader><CardTitle>Recent Alerts</CardTitle></CardHeader>
          <CardContent><p className="text-sm text-muted-foreground">No recent alerts.</p></CardContent>
        </Card>
        <Card>
          <CardHeader><CardTitle>Quick Actions</CardTitle></CardHeader>
          <CardContent><p className="text-sm text-muted-foreground">Use the Quick Actions menu to orchestrate deployments.</p></CardContent>
        </Card>
      </div>
    </div>
  )
}
