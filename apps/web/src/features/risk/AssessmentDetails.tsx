'use client';
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { ShieldAlert, Info, PlayCircle, FileText, ArrowRight, Activity } from 'lucide-react'

export function AssessmentDetails({ id }: { id: string | null }) {
  if (!id) {
    return (
      <div className="flex flex-col h-full bg-card items-center justify-center p-6 text-center text-muted-foreground">
        <ShieldAlert className="h-12 w-12 mb-4 opacity-20" />
        <p>Select an assessment to view deep risk details and mitigation strategies.</p>
      </div>
    )
  }

  return (
    <div className="flex flex-col h-full bg-card overflow-y-auto animate-in slide-in-from-right-8 duration-300">
      <div className="p-6 border-b flex-shrink-0">
        <div className="flex justify-between items-start">
          <div>
            <h2 className="text-xl font-bold">Resource Depletion Risk</h2>
            <p className="text-sm text-muted-foreground mt-1">Category: Resource • ID: RSK-992</p>
          </div>
          <Badge variant="destructive">High Severity</Badge>
        </div>
      </div>

      <div className="p-6 space-y-6 flex-1 overflow-y-auto">
        <div className="space-y-2">
          <h3 className="text-sm font-semibold">Description</h3>
          <p className="text-sm text-muted-foreground leading-relaxed">
            The planned parallel execution wave targets 49 satellites simultaneously. Orbital tracking indicates that 12 of these satellites are entering a prolonged eclipse phase. Battery capacity is predicted to drop below the 35% safe-operation threshold during the software payload application.
          </p>
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div className="p-3 border border-border rounded-lg bg-muted/30">
            <span className="text-xs text-muted-foreground block">Likelihood</span>
            <span className="font-medium">Very High (92%)</span>
          </div>
          <div className="p-3 border border-border rounded-lg bg-muted/30">
            <span className="text-xs text-muted-foreground block">Impact</span>
            <span className="font-medium text-destructive">Critical</span>
          </div>
        </div>

        <div className="space-y-2">
          <h3 className="text-sm font-semibold">Affected Assets</h3>
          <div className="text-sm text-muted-foreground font-mono bg-muted/50 p-2 rounded">
            SAT-B-012, SAT-B-014, SAT-B-018... (+9 others)
          </div>
        </div>

        <div className="space-y-2">
          <h3 className="text-sm font-semibold">Recommended Mitigation</h3>
          <div className="flex gap-3 text-sm p-3 bg-primary/10 text-primary rounded-lg border border-primary/20">
            <Info className="h-5 w-5 flex-shrink-0" />
            <p>
              Modify the deployment plan. Change Wave 3 from 'Parallel Execution' to 'Sequential Execution', or partition the wave to exclude satellites currently in eclipse.
            </p>
          </div>
        </div>
      </div>

      <div className="p-6 border-t bg-muted/30 space-y-2 flex-shrink-0">
        <h3 className="text-sm font-semibold mb-3">Quick Actions</h3>
        <Button variant="outline" className="w-full justify-start text-sm"><PlayCircle className="mr-2 h-4 w-4" /> Run New Assessment</Button>
        <Button variant="outline" className="w-full justify-start text-sm"><ArrowRight className="mr-2 h-4 w-4" /> Open Deployment Plan</Button>
        <Button variant="outline" className="w-full justify-start text-sm"><Activity className="mr-2 h-4 w-4" /> Preview Coordination</Button>
        <Button variant="outline" className="w-full justify-start text-sm"><FileText className="mr-2 h-4 w-4" /> Export Report</Button>
      </div>
    </div>
  )
}
