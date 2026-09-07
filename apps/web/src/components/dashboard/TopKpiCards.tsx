import { Satellite, Rocket, PlayCircle, ShieldAlert, Activity, GitBranch } from 'lucide-react'
import { MetricCard } from './MetricCard'
import { KPIs } from '@/lib/mock-data'

export function TopKpiCards() {
  return (
    <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4 xl:grid-cols-7">
      <MetricCard title="Total Assets" value={KPIs.totalSatellites} icon={<Satellite className="h-4 w-4" />} />
      <MetricCard title="Online" value={KPIs.onlineSatellites} icon={<Activity className="h-4 w-4" />} trend="+2" trendUp={true} description="since yesterday" />
      <MetricCard title="Active Missions" value={KPIs.activeMissions} icon={<Rocket className="h-4 w-4" />} />
      <MetricCard title="Deployments" value={KPIs.deploymentsToday} icon={<PlayCircle className="h-4 w-4" />} description="today" />
      <MetricCard title="Pending Plans" value={KPIs.pendingPlans} icon={<GitBranch className="h-4 w-4" />} />
      <MetricCard title="Critical Alerts" value={KPIs.criticalAlerts} icon={<ShieldAlert className="h-4 w-4 text-destructive" />} />
      <MetricCard title="Fleet Health" value={`${KPIs.overallHealth}%`} icon={<Activity className="h-4 w-4 text-primary" />} />
    </div>
  )
}
