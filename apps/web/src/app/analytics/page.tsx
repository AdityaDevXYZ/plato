'use client';
import { useState } from 'react';
import { AnalyticsExplorer } from '@/features/analytics/AnalyticsExplorer'
import { AnalyticsDashboard } from '@/features/analytics/AnalyticsDashboard'
import { InsightDetails } from '@/features/analytics/InsightDetails'

export default function AnalyticsPage() {
  const [selectedReport, setSelectedReport] = useState<string | null>('rep-001');

  return (
    <div className="flex h-[calc(100vh-8rem)] -mx-6 -mb-6 overflow-hidden bg-background border-t animate-in fade-in duration-500">
      <div className="w-80 flex-shrink-0 z-10 hidden md:block">
        <AnalyticsExplorer onSelect={setSelectedReport} selectedId={selectedReport} />
      </div>
      
      <div className="flex-1 min-w-0">
        <AnalyticsDashboard />
      </div>
      
      <div className="w-96 flex-shrink-0 border-l border-border z-10 hidden xl:block">
        <InsightDetails id={selectedReport} />
      </div>
    </div>
  )
}
