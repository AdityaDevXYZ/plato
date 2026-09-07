import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { PlayCircle, GitBranch, ShieldAlert } from "lucide-react"

export function TimelineTab() {
  return (
    <Card className="animate-in fade-in duration-500">
      <CardHeader><CardTitle>Activity Timeline</CardTitle></CardHeader>
      <CardContent>
        <div className="space-y-4">
          <div className="flex items-start gap-4">
            <div className="rounded-full bg-secondary p-2 mt-1"><PlayCircle className="h-4 w-4 text-primary" /></div>
            <div>
              <p className="text-sm font-medium">Deployment Started</p>
              <p className="text-xs text-muted-foreground">Wave 3 initialized.</p>
              <p className="text-xs text-muted-foreground mt-1">2 hrs ago</p>
            </div>
          </div>
          <div className="flex items-start gap-4">
            <div className="rounded-full bg-secondary p-2 mt-1"><GitBranch className="h-4 w-4 text-primary" /></div>
            <div>
              <p className="text-sm font-medium">Coordination Session Created</p>
              <p className="text-xs text-muted-foreground">Plan PL-002 expanded into 3 waves.</p>
              <p className="text-xs text-muted-foreground mt-1">1 day ago</p>
            </div>
          </div>
          <div className="flex items-start gap-4">
            <div className="rounded-full bg-secondary p-2 mt-1"><ShieldAlert className="h-4 w-4 text-primary" /></div>
            <div>
              <p className="text-sm font-medium">Risk Assessment Completed</p>
              <p className="text-xs text-muted-foreground">Overall risk evaluated as Low.</p>
              <p className="text-xs text-muted-foreground mt-1">1 day ago</p>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
