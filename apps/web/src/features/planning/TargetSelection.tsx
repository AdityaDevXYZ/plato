'use client';
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Select } from "@/components/ui/select"
import { PlanningMissions } from '@/lib/planning-mock-data'
import { Search, Filter, Satellite } from 'lucide-react'

export function TargetSelection() {
  return (
    <div className="flex flex-col h-full bg-card border-r">
      <div className="p-4 border-b space-y-4">
        <h2 className="font-semibold">Target Selection</h2>
        
        <div className="space-y-2">
          <label className="text-xs font-medium text-muted-foreground">Mission Grouping</label>
          <div className="relative">
            <Select defaultValue="m-alpha">
              {PlanningMissions.map(m => (
                <option key={m.id} value={m.id}>{m.name} ({m.targets} assets)</option>
              ))}
            </Select>
            <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-muted-foreground">
              <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M19 9l-7 7-7-7"></path></svg>
            </div>
          </div>
        </div>

        <div className="relative">
          <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input placeholder="Search targets..." className="pl-9 bg-background" />
        </div>
        
        <div className="flex gap-2">
          <button className="text-xs flex items-center bg-accent text-accent-foreground px-2 py-1 rounded"><Filter className="h-3 w-3 mr-1" /> Active only</button>
          <button className="text-xs flex items-center bg-accent text-accent-foreground px-2 py-1 rounded"><Filter className="h-3 w-3 mr-1" /> Needs Update</button>
        </div>
      </div>
      
      <div className="flex-1 overflow-y-auto p-4 space-y-2">
        <div className="flex justify-between items-center mb-2">
          <span className="text-sm font-medium">Selected (64/64)</span>
          <span className="text-xs text-primary cursor-pointer hover:underline">Clear all</span>
        </div>
        
        {Array.from({length: 10}).map((_, i) => (
          <div key={i} className="flex items-center gap-3 p-2 bg-muted/50 rounded-md border border-transparent hover:border-border">
            <input type="checkbox" className="accent-primary" defaultChecked />
            <Satellite className="h-4 w-4 text-muted-foreground" />
            <span className="text-sm">SAT-A-00{i+1}</span>
            <span className="text-xs text-muted-foreground ml-auto">v2.4.0</span>
          </div>
        ))}
      </div>
    </div>
  )
}
