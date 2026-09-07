import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Rocket, LayoutList, ShieldAlert, Globe, GitBranch } from 'lucide-react'

export function QuickActions() {
  return (
    <Card className="col-span-1">
      <CardHeader>
        <CardTitle>Quick Actions</CardTitle>
      </CardHeader>
      <CardContent className="grid grid-cols-1 gap-2">
        <Button variant="outline" className="justify-start w-full"><Rocket className="mr-2 h-4 w-4" /> Create Mission</Button>
        <Button variant="outline" className="justify-start w-full"><LayoutList className="mr-2 h-4 w-4" /> Generate Plan</Button>
        <Button variant="outline" className="justify-start w-full"><ShieldAlert className="mr-2 h-4 w-4" /> Run Assessment</Button>
        <Button variant="outline" className="justify-start w-full"><Globe className="mr-2 h-4 w-4" /> Open Digital Twin</Button>
        <Button variant="outline" className="justify-start w-full"><GitBranch className="mr-2 h-4 w-4" /> Start Coordination</Button>
      </CardContent>
    </Card>
  )
}
