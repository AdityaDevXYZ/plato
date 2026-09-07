'use client';
import { Input } from "@/components/ui/input"
import { Search, ChevronDown, ChevronRight, Satellite, Radio, Activity } from 'lucide-react'
import { useState } from 'react'
import { FleetData } from '@/lib/twin-mock-data'
import { cn } from '@/lib/utils'

export function FleetExplorer({ onSelectAsset, selectedAssetId }: { onSelectAsset: (asset: any) => void, selectedAssetId: string | null }) {
  const [expanded, setExpanded] = useState<Record<string, boolean>>({'m-alpha': true, 'm-beta': true, 'ground': true});
  const [search, setSearch] = useState('');

  const toggleGroup = (id: string) => {
    setExpanded(prev => ({ ...prev, [id]: !prev[id] }));
  };

  const getStatusColor = (status: string) => {
    if (status === 'Critical') return 'bg-destructive';
    if (status === 'Warning') return 'bg-yellow-500';
    return 'bg-green-500';
  };

  return (
    <div className="flex flex-col h-full bg-card border-r">
      <div className="p-4 border-b">
        <h2 className="font-semibold mb-4">Fleet Explorer</h2>
        <div className="relative">
          <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input 
            placeholder="Search assets..." 
            className="pl-9 bg-background"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>
      </div>
      
      <div className="flex-1 overflow-y-auto p-2">
        {FleetData.map(group => (
          <div key={group.missionId} className="mb-2">
            <button 
              onClick={() => toggleGroup(group.missionId)}
              className="flex items-center w-full p-2 text-sm font-medium rounded-md hover:bg-accent hover:text-accent-foreground text-left"
            >
              {expanded[group.missionId] ? <ChevronDown className="h-4 w-4 mr-1 text-muted-foreground" /> : <ChevronRight className="h-4 w-4 mr-1 text-muted-foreground" />}
              {group.missionName}
            </button>
            
            {expanded[group.missionId] && (
              <div className="ml-4 mt-1 space-y-1">
                {group.assets.filter(a => a.name.toLowerCase().includes(search.toLowerCase())).map(asset => (
                  <button
                    key={asset.id}
                    onClick={() => onSelectAsset(asset)}
                    className={cn(
                      "flex items-center w-full p-2 text-sm rounded-md hover:bg-muted text-left",
                      selectedAssetId === asset.id && "bg-muted font-medium"
                    )}
                  >
                    {asset.type === 'Satellite' ? <Satellite className="h-3 w-3 mr-2 text-muted-foreground" /> : <Radio className="h-3 w-3 mr-2 text-muted-foreground" />}
                    <span className="flex-1 truncate">{asset.name}</span>
                    <span className={cn("h-2 w-2 rounded-full", getStatusColor(asset.status))}></span>
                  </button>
                ))}
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  )
}
