'use client';
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Select } from "@/components/ui/select"
import { MockPhases } from '@/lib/planning-mock-data'
import { Plus, GripVertical, Copy, Trash2, Settings2 } from 'lucide-react'

export function PlanBuilder() {
  return (
    <div className="flex flex-col h-full bg-background border-r">
      <div className="p-6 border-b bg-card">
        <div className="flex justify-between items-start mb-6">
          <div>
            <h2 className="text-2xl font-bold">Plan Builder</h2>
            <p className="text-sm text-muted-foreground mt-1">Design deployment strategies and execution phases.</p>
          </div>
          <div className="flex gap-2">
            <Button variant="outline">Save Draft</Button>
            <Button>Publish Plan</Button>
          </div>
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div className="space-y-2 relative">
            <label className="text-xs font-medium text-muted-foreground">Deployment Strategy</label>
            <Select defaultValue="canary">
              <option value="canary">Canary Rollout (Recommended)</option>
              <option value="sequential">Sequential</option>
              <option value="batch">Batch / Ring</option>
              <option value="regional">Regional</option>
              <option value="custom">Custom</option>
            </Select>
            <div className="pointer-events-none absolute bottom-0 right-0 flex h-10 items-center px-2 text-muted-foreground">
              <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M19 9l-7 7-7-7"></path></svg>
            </div>
          </div>
          <div className="space-y-2 relative">
            <label className="text-xs font-medium text-muted-foreground">Rollback Policy</label>
            <Select defaultValue="auto">
              <option value="auto">Automatic (On Health Check Failure)</option>
              <option value="manual">Manual Only</option>
              <option value="disabled">Disabled</option>
            </Select>
            <div className="pointer-events-none absolute bottom-0 right-0 flex h-10 items-center px-2 text-muted-foreground">
              <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M19 9l-7 7-7-7"></path></svg>
            </div>
          </div>
        </div>
      </div>
      
      <div className="flex-1 overflow-y-auto p-6 bg-muted/20">
        <div className="flex justify-between items-center mb-4">
          <h3 className="font-medium text-sm">Execution Phases</h3>
          <Button variant="outline" size="sm"><Plus className="h-4 w-4 mr-1" /> Add Phase</Button>
        </div>

        <div className="space-y-3">
          {MockPhases.map((phase, i) => (
            <Card key={phase.id} className="shadow-sm border-border">
              <CardContent className="p-4 flex items-center gap-4">
                <GripVertical className="h-5 w-5 text-muted-foreground cursor-grab hover:text-foreground transition-colors" />
                
                <div className="flex-1 grid grid-cols-12 gap-4 items-center">
                  <div className="col-span-1 text-xl font-bold text-muted-foreground opacity-50">{i+1}</div>
                  <div className="col-span-3">
                    <p className="font-medium text-sm">{phase.name}</p>
                    <p className="text-xs text-muted-foreground">{phase.targets} targets ({phase.size})</p>
                  </div>
                  <div className="col-span-4 relative">
                    <Select defaultValue={phase.parallel ? "parallel" : "sequential"} className="h-8 text-xs">
                      <option value="parallel">Parallel Execution</option>
                      <option value="sequential">Sequential Execution</option>
                    </Select>
                    <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-muted-foreground">
                      <svg className="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M19 9l-7 7-7-7"></path></svg>
                    </div>
                  </div>
                  <div className="col-span-4 flex justify-end gap-1">
                    <Button variant="ghost" size="icon" className="h-8 w-8"><Settings2 className="h-4 w-4" /></Button>
                    <Button variant="ghost" size="icon" className="h-8 w-8"><Copy className="h-4 w-4" /></Button>
                    <Button variant="ghost" size="icon" className="h-8 w-8 text-destructive"><Trash2 className="h-4 w-4" /></Button>
                  </div>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    </div>
  )
}
