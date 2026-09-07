'use client';
import { Input } from "@/components/ui/input"
import { Search, Filter, ShieldAlert, CheckCircle, AlertTriangle } from 'lucide-react'
import { useState } from 'react'
import { AssessmentHistory } from '@/lib/risk-mock-data'
import { cn } from '@/lib/utils'

export function AssessmentExplorer({ onSelect, selectedId }: { onSelect: (id: string) => void, selectedId: string | null }) {
  const [search, setSearch] = useState('');

  return (
    <div className="flex flex-col h-full bg-card border-r">
      <div className="p-4 border-b space-y-4 flex-shrink-0">
        <h2 className="font-semibold">Assessment Explorer</h2>

        <div className="relative">
          <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input 
            placeholder="Search reports..." 
            className="pl-9 bg-background"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>
        
        <div className="flex gap-2 flex-wrap">
          <button className="text-xs flex items-center bg-accent text-accent-foreground px-2 py-1 rounded"><Filter className="h-3 w-3 mr-1" /> High Risk</button>
          <button className="text-xs flex items-center bg-accent text-accent-foreground px-2 py-1 rounded"><Filter className="h-3 w-3 mr-1" /> Recent</button>
        </div>
      </div>
      
      <div className="flex-1 overflow-y-auto p-2">
        {AssessmentHistory.filter(a => a.mission.toLowerCase().includes(search.toLowerCase()) || a.plan.toLowerCase().includes(search.toLowerCase())).map(assessment => (
          <button
            key={assessment.id}
            onClick={() => onSelect(assessment.id)}
            className={cn(
              "flex flex-col w-full p-3 text-sm rounded-md hover:bg-muted text-left mb-1 transition-colors",
              selectedId === assessment.id && "bg-muted border-l-2 border-l-primary rounded-l-none"
            )}
          >
            <div className="flex justify-between items-start w-full">
              <span className="font-medium truncate pr-2">{assessment.plan}</span>
              {assessment.overallRisk === 'High' && <ShieldAlert className="h-4 w-4 text-destructive flex-shrink-0" />}
              {assessment.overallRisk === 'Medium' && <AlertTriangle className="h-4 w-4 text-yellow-500 flex-shrink-0" />}
              {assessment.overallRisk === 'Low' && <CheckCircle className="h-4 w-4 text-green-500 flex-shrink-0" />}
            </div>
            <span className="text-xs text-muted-foreground mt-1 truncate">{assessment.mission}</span>
            <div className="flex justify-between w-full mt-2 text-xs text-muted-foreground">
              <span>{assessment.date}</span>
              <span>Score: {assessment.score}</span>
            </div>
          </button>
        ))}
      </div>
    </div>
  )
}
