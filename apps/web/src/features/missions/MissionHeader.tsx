import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { PlayCircle, Settings, Download } from "lucide-react"

export function MissionHeader({ mission }: { mission: any }) {
  return (
    <div className="flex flex-col md:flex-row items-start md:items-center justify-between gap-4 pb-6">
      <div>
        <div className="flex items-center gap-3">
          <h1 className="text-3xl font-bold tracking-tight">{mission.name}</h1>
          <Badge variant={mission.status === 'Active' ? 'success' : 'secondary'}>{mission.status}</Badge>
        </div>
        <p className="text-muted-foreground mt-1">ID: {mission.id} • Owner: {mission.owner}</p>
      </div>
      <div className="flex items-center gap-2">
        <Button variant="outline"><Download className="h-4 w-4 mr-2" /> Export</Button>
        <Button variant="outline"><Settings className="h-4 w-4 mr-2" /> Configure</Button>
        <Button><PlayCircle className="h-4 w-4 mr-2" /> New Deployment</Button>
      </div>
    </div>
  )
}
