'use client';
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { AnalyticsReports } from '@/lib/analytics-mock-data'
import { FileText, Download, Rocket, PlayCircle, ShieldAlert, FileJson, BarChart3 } from 'lucide-react'

export function InsightDetails({ id }: { id: string | null }) {
  if (!id) {
    return (
      <div className="flex flex-col h-full bg-card items-center justify-center p-6 text-center text-muted-foreground">
        <BarChart3 className="h-12 w-12 mb-4 opacity-20" />
        <p>Select a report to view deep operational insights.</p>
      </div>
    )
  }

  const report = AnalyticsReports.find(r => r.id === id) || AnalyticsReports[0];

  return (
    <div className="flex flex-col h-full bg-card overflow-y-auto animate-in slide-in-from-right-8 duration-300">
      <div className="p-6 border-b flex-shrink-0">
        <h2 className="text-xl font-bold">{report.name}</h2>
        <p className="text-sm text-muted-foreground mt-1">Generated: {report.date} • {report.category}</p>
      </div>

      <div className="p-6 space-y-6 flex-1 overflow-y-auto">
        <div className="space-y-2">
          <h3 className="text-sm font-semibold border-b border-border pb-2">Executive Summary</h3>
          <p className="text-sm text-muted-foreground leading-relaxed pt-2">
            The Alpha Constellation experienced a 1.2% improvement in overall deployment success rates compared to Q1. The adoption of the "Canary Rollout" strategy has effectively eliminated fleet-wide software regressions, isolating failures to sub-5% target groups.
          </p>
        </div>

        <div className="space-y-2">
          <h3 className="text-sm font-semibold border-b border-border pb-2">Key Findings</h3>
          <ul className="text-sm text-muted-foreground space-y-3 pt-2">
            <li className="flex items-start gap-2">
              <span className="text-green-500 mt-0.5">•</span>
              Mean Time to Recovery (MTTR) dropped by 45 minutes across the entire fleet.
            </li>
            <li className="flex items-start gap-2">
              <span className="text-destructive mt-0.5">•</span>
              Timeouts during communication windows remain the leading cause of individual asset deployment failure (45%).
            </li>
            <li className="flex items-start gap-2">
              <span className="text-primary mt-0.5">•</span>
              Satellites in high-inclination orbits demonstrated a higher necessity for retry mechanisms.
            </li>
          </ul>
        </div>

        <div className="space-y-2">
          <h3 className="text-sm font-semibold border-b border-border pb-2">Strategic Recommendations</h3>
          <div className="bg-primary/5 p-4 rounded-lg border border-primary/20 space-y-2 mt-2">
            <p className="text-sm font-medium text-primary">Revise Communication Constraints</p>
            <p className="text-xs text-muted-foreground">Adjust deployment planning heuristics to require a minimum 8-minute uninterrupted ground station window for full payload delivery, reducing timeout-based failures.</p>
          </div>
        </div>
      </div>

      <div className="p-6 border-t bg-muted/30 space-y-2 flex-shrink-0">
        <h3 className="text-sm font-semibold mb-3">Quick Actions</h3>
        <div className="grid grid-cols-2 gap-2 mb-2">
          <Button variant="outline" size="sm" className="w-full justify-start"><Rocket className="mr-2 h-4 w-4" /> View Mission</Button>
          <Button variant="outline" size="sm" className="w-full justify-start"><PlayCircle className="mr-2 h-4 w-4" /> View Deployments</Button>
        </div>
        <Button variant="outline" size="sm" className="w-full justify-start"><ShieldAlert className="mr-2 h-4 w-4" /> Open Risk Assessment</Button>
        
        <div className="pt-4 mt-4 border-t border-border">
          <h3 className="text-xs font-semibold mb-2 text-muted-foreground uppercase">Export Report</h3>
          <div className="flex gap-2">
            <Button variant="secondary" size="sm" className="flex-1"><Download className="mr-2 h-4 w-4" /> PDF</Button>
            <Button variant="secondary" size="sm" className="flex-1"><FileText className="mr-2 h-4 w-4" /> CSV</Button>
            <Button variant="secondary" size="sm" className="flex-1"><FileJson className="mr-2 h-4 w-4" /> JSON</Button>
          </div>
        </div>
      </div>
    </div>
  )
}
