'use client';
import { useState } from 'react';
import { AssessmentExplorer } from '@/features/risk/AssessmentExplorer'
import { RiskDashboard } from '@/features/risk/RiskDashboard'
import { AssessmentDetails } from '@/features/risk/AssessmentDetails'

export default function RiskAssessmentPage() {
  const [selectedAssessment, setSelectedAssessment] = useState<string | null>(null);

  return (
    <div className="flex h-[calc(100vh-8rem)] -mx-6 -mb-6 overflow-hidden bg-background border-t animate-in fade-in duration-500">
      <div className="w-80 flex-shrink-0 z-10 hidden md:block">
        <AssessmentExplorer onSelect={setSelectedAssessment} selectedId={selectedAssessment} />
      </div>
      
      <div className="flex-1 min-w-0">
        <RiskDashboard />
      </div>
      
      <div className="w-80 flex-shrink-0 border-l border-border z-10 hidden xl:block">
        <AssessmentDetails id={selectedAssessment} />
      </div>
    </div>
  )
}
