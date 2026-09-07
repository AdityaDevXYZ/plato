import { TopKpiCards } from "@/components/dashboard/TopKpiCards"
import { FleetHealthChart } from "@/components/dashboard/FleetHealthChart"
import { MissionStatusTable } from "@/components/dashboard/MissionStatusTable"
import { RecentActivityTimeline } from "@/components/dashboard/RecentActivityTimeline"
import { AlertCenter } from "@/components/dashboard/AlertCenter"
import { SystemHealth } from "@/components/dashboard/SystemHealth"
import { QuickActions } from "@/components/dashboard/QuickActions"

export default function Dashboard() {
  return (
    <div className="space-y-6 pb-8 animate-in fade-in duration-500">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Executive Dashboard</h1>
        <p className="text-muted-foreground mt-1">Global view of constellation telemetry, orchestration, and fleet health.</p>
      </div>
      
      <TopKpiCards />
      
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
        <FleetHealthChart />
        <MissionStatusTable />
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
        <RecentActivityTimeline />
        <AlertCenter />
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
        <SystemHealth />
        <QuickActions />
        <div className="col-span-1 lg:col-span-1 xl:col-span-2 hidden xl:block" />
      </div>
    </div>
  );
}
