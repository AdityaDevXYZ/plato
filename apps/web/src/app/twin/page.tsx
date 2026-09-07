'use client';
import { useState } from 'react';
import { FleetExplorer } from '@/features/twin/FleetExplorer'
import { FleetVisualization } from '@/features/twin/FleetVisualization'
import { AssetDetails } from '@/features/twin/AssetDetails'

export default function DigitalTwinPage() {
  const [selectedAsset, setSelectedAsset] = useState<any | null>(null);

  return (
    <div className="flex h-[calc(100vh-8rem)] -mx-6 -mb-6 overflow-hidden bg-background border-t">
      <div className="w-72 flex-shrink-0 hidden md:block">
        <FleetExplorer onSelectAsset={setSelectedAsset} selectedAssetId={selectedAsset?.id || null} />
      </div>
      
      <div className="flex-1 min-w-0">
        <FleetVisualization onSelectAsset={setSelectedAsset} selectedAssetId={selectedAsset?.id || null} />
      </div>
      
      <div className="flex-shrink-0 hidden lg:block border-l border-border h-full">
        <AssetDetails asset={selectedAsset} />
      </div>
    </div>
  )
}
