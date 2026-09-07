import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table"
import { Badge } from "@/components/ui/badge"

export function AssetsTab({ mission }: { mission: any }) {
  return (
    <Card className="animate-in fade-in duration-500">
      <CardHeader><CardTitle>Mission Assets</CardTitle></CardHeader>
      <CardContent>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Asset ID</TableHead>
              <TableHead>Type</TableHead>
              <TableHead>Health</TableHead>
              <TableHead>Battery</TableHead>
              <TableHead>Software</TableHead>
              <TableHead>Twin Status</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {[1, 2, 3, 4, 5].map((i) => (
              <TableRow key={i}>
                <TableCell className="font-medium">SAT-{mission.id.split('-')[1]}-00{i}</TableCell>
                <TableCell>Satellite</TableCell>
                <TableCell><Badge variant="success">Nominal</Badge></TableCell>
                <TableCell>98%</TableCell>
                <TableCell>{mission.release}</TableCell>
                <TableCell><span className="flex h-2 w-2 rounded-full bg-primary" title="Synced"></span></TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  )
}
