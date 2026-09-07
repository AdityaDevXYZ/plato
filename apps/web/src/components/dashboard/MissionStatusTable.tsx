import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table"
import { Badge } from "@/components/ui/badge"
import { MissionStatus } from '@/lib/mock-data'

export function MissionStatusTable() {
  return (
    <Card className="col-span-1 lg:col-span-2 xl:col-span-3">
      <CardHeader>
        <CardTitle>Active Missions</CardTitle>
      </CardHeader>
      <CardContent>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Mission</TableHead>
              <TableHead>Status</TableHead>
              <TableHead>Satellites</TableHead>
              <TableHead>Release</TableHead>
              <TableHead>Risk</TableHead>
              <TableHead className="text-right">Last Activity</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {MissionStatus.map((mission) => (
              <TableRow key={mission.id}>
                <TableCell className="font-medium">{mission.name}</TableCell>
                <TableCell>
                  <Badge variant={
                    mission.status === 'Active' ? 'success' :
                    mission.status === 'Deploying' ? 'default' :
                    mission.status === 'Planning' ? 'secondary' : 'warning'
                  }>
                    {mission.status}
                  </Badge>
                </TableCell>
                <TableCell>{mission.satellites}</TableCell>
                <TableCell className="text-muted-foreground">{mission.release}</TableCell>
                <TableCell>
                  <Badge variant={
                    mission.risk === 'Low' ? 'success' :
                    mission.risk === 'Medium' ? 'warning' : 'destructive'
                  }>
                    {mission.risk}
                  </Badge>
                </TableCell>
                <TableCell className="text-right text-muted-foreground">{mission.lastActivity}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  )
}
