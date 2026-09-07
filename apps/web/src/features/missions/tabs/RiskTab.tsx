import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"

export function RiskTab() {
  return (
    <Card className="animate-in fade-in duration-500">
      <CardHeader><CardTitle>Risk Profile</CardTitle></CardHeader>
      <CardContent>
        <div className="space-y-4">
          <div className="flex items-center justify-between border-b pb-4">
            <span className="font-medium">Overall Assessment</span>
            <Badge variant="success">Low Risk (95/100)</Badge>
          </div>
          <div className="space-y-2">
            <h4 className="text-sm font-semibold">Categories</h4>
            <div className="grid grid-cols-2 gap-4 text-sm">
              <div className="flex justify-between p-2 border rounded"><span>Operational</span> <Badge variant="success">Nominal</Badge></div>
              <div className="flex justify-between p-2 border rounded"><span>Orbital</span> <Badge variant="success">Clear</Badge></div>
              <div className="flex justify-between p-2 border rounded"><span>Resource</span> <Badge variant="success">Sufficient</Badge></div>
              <div className="flex justify-between p-2 border rounded"><span>Communications</span> <Badge variant="warning">Marginal</Badge></div>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
