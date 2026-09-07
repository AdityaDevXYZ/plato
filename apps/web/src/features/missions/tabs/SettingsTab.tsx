import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import { Button } from "@/components/ui/button"

export function SettingsTab() {
  return (
    <div className="space-y-4 animate-in fade-in duration-500">
      <Card>
        <CardHeader>
          <CardTitle>Mission Configuration</CardTitle>
          <CardDescription>Update fundamental parameters of this mission.</CardDescription>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-muted-foreground">Configuration options are managed via PLATO IaC (Infrastructure as Code).</p>
        </CardContent>
      </Card>
      
      <Card className="border-destructive">
        <CardHeader>
          <CardTitle className="text-destructive">Danger Zone</CardTitle>
          <CardDescription>Destructive actions affecting this mission.</CardDescription>
        </CardHeader>
        <CardContent className="flex gap-4">
          <Button variant="destructive">Archive Mission</Button>
          <Button variant="outline">Transfer Ownership</Button>
        </CardContent>
      </Card>
    </div>
  )
}
