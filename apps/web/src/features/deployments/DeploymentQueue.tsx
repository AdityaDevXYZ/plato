'use client';
import { Input } from "@/components/ui/input"
import { Search, Filter, PlayCircle, Clock, CheckCircle, XCircle } from 'lucide-react'
import { useState } from 'react'
import { DeploymentQueue as QueueData } from '@/lib/deployments-mock-data'
import { cn } from '@/lib/utils'
import { Badge } from "@/components/ui/badge"

export function DeploymentQueue({ onSelect, selectedId }: { onSelect: (id: string) => void, selectedId: string | null }) {
  const [search, setSearch] = useState('');

  return (
    <div className="flex flex-col h-full bg-card border-r">
      <div className="p-4 border-b space-y-4 flex-shrink-0">
        <h2 className="font-semibold">Deployment Queue</h2>

        <div className="relative">
          <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input 
            placeholder="Search deployments..." 
            className="pl-9 bg-background"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>
        
        <div className="flex gap-2 flex-wrap">
          <button className="text-xs flex items-center bg-accent text-accent-foreground px-2 py-1 rounded"><Filter className="h-3 w-3 mr-1" /> Running</button>
          <button className="text-xs flex items-center bg-accent text-accent-foreground px-2 py-1 rounded"><Filter className="h-3 w-3 mr-1" /> High Priority</button>
        </div>
      </div>
      
      <div className="flex-1 overflow-y-auto p-2">
        {QueueData.filter(d => d.mission.toLowerCase().includes(search.toLowerCase())).map(dep => (
          <button
            key={dep.id}
            onClick={() => onSelect(dep.id)}
            className={cn(
              "flex flex-col w-full p-3 text-sm rounded-md hover:bg-muted text-left mb-1 transition-colors border",
              selectedId === dep.id ? "bg-muted border-l-2 border-l-primary border-y-transparent border-r-transparent rounded-l-none" : "border-transparent"
            )}
          >
            <div className="flex justify-between items-start w-full mb-1">
              <span className="font-medium truncate pr-2">{dep.mission}</span>
              {dep.status === 'Running' && <PlayCircle className="h-4 w-4 text-primary flex-shrink-0" />}
              {dep.status === 'Queued' && <Clock className="h-4 w-4 text-muted-foreground flex-shrink-0" />}
              {dep.status === 'Completed' && <CheckCircle className="h-4 w-4 text-green-500 flex-shrink-0" />}
              {dep.status === 'Failed' && <XCircle className="h-4 w-4 text-destructive flex-shrink-0" />}
            </div>
            
            <div className="flex justify-between items-center w-full mt-1">
              <Badge variant={dep.status === 'Running' ? 'default' : 'outline'} className="text-[10px] px-1.5 py-0">
                {dep.status}
              </Badge>
              <span className="text-xs text-muted-foreground">{dep.progress}%</span>
            </div>
            
            <div className="w-full bg-secondary h-1 rounded-full mt-2 overflow-hidden">
              <div 
                className={cn("h-full", dep.status === 'Failed' ? 'bg-destructive' : dep.status === 'Completed' ? 'bg-green-500' : 'bg-primary')} 
                style={{ width: `${dep.progress}%` }} 
              />
            </div>
          </button>
        ))}
      </div>
    </div>
  )
}
