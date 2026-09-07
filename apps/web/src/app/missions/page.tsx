import { MissionTable } from "@/features/missions/MissionTable"

export default function MissionsPage() {
  return (
    <div className="space-y-6 animate-in fade-in duration-500">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Mission Operations</h1>
        <p className="text-muted-foreground mt-1">Manage, monitor, and coordinate satellite missions globally.</p>
      </div>
      <MissionTable />
    </div>
  )
}
