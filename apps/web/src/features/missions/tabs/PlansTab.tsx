import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"

export function PlansTab() {
  return (
    <Card className="animate-in fade-in duration-500">
      <CardHeader><CardTitle>Deployment Plans</CardTitle></CardHeader>
      <CardContent>
        <div className="space-y-4">
          {[1, 2].map((i) => (
            <div key={i} className="border rounded-lg p-4 flex justify-between items-center">
              <div>
                <h4 className="font-medium">Plan PL-00{i}</h4>
                <p className="text-sm text-muted-foreground mt-1">Canary Rollout Strategy • 3 Phases</p>
              </div>
              <Badge variant={i === 1 ? 'success' : 'secondary'}>{i === 1 ? 'Completed' : 'Archived'}</Badge>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  )
}
