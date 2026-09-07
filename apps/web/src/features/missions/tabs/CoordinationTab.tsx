import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Progress } from "@/components/ui/progress"
import { Badge } from "@/components/ui/badge"

export function CoordinationTab() {
  return (
    <Card className="animate-in fade-in duration-500">
      <CardHeader><CardTitle>Coordination & Waves</CardTitle></CardHeader>
      <CardContent>
        <div className="space-y-6">
          <div className="space-y-2">
            <div className="flex justify-between text-sm">
              <span className="font-medium">Wave 1: Canary Deployment</span>
              <Badge variant="success">Completed</Badge>
            </div>
            <Progress value={100} />
          </div>
          <div className="space-y-2">
            <div className="flex justify-between text-sm">
              <span className="font-medium">Wave 2: US Region</span>
              <Badge variant="success">Completed</Badge>
            </div>
            <Progress value={100} />
          </div>
          <div className="space-y-2">
            <div className="flex justify-between text-sm">
              <span className="font-medium">Wave 3: Global Fleet</span>
              <Badge variant="default">Running</Badge>
            </div>
            <Progress value={45} />
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
