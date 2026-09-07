import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { RecentActivity } from '@/lib/mock-data'
import { GitBranch, ShieldAlert, PlayCircle, Key, BarChart } from 'lucide-react'

const icons: Record<string, any> = {
  deployment: PlayCircle,
  risk: ShieldAlert,
  coordination: GitBranch,
  auth: Key,
  analytics: BarChart,
}

export function RecentActivityTimeline() {
  return (
    <Card className="col-span-1 lg:col-span-1 xl:col-span-2">
      <CardHeader>
        <CardTitle>Recent Activity</CardTitle>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          {RecentActivity.map((activity) => {
            const Icon = icons[activity.type] || PlayCircle;
            return (
              <div key={activity.id} className="flex items-start gap-4">
                <div className="rounded-full bg-secondary p-2 mt-1">
                  <Icon className="h-4 w-4 text-primary" />
                </div>
                <div className="flex-1 space-y-1">
                  <p className="text-sm font-medium leading-none">{activity.title}</p>
                  <p className="text-xs text-muted-foreground">{activity.description}</p>
                </div>
                <div className="text-xs text-muted-foreground whitespace-nowrap">
                  {activity.time}
                </div>
              </div>
            )
          })}
        </div>
      </CardContent>
    </Card>
  )
}
