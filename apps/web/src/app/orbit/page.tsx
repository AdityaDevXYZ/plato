'use client';
import { useState } from 'react';
import dynamic from 'next/dynamic';
import { ConstellationExplorer } from '@/features/orbit/ConstellationExplorer';
import { OrbitalDetails } from '@/features/orbit/OrbitalDetails';

const EarthVisualization = dynamic(
  () => import('@/features/orbit/EarthVisualization').then(m => m.EarthVisualization),
  {
    ssr: false,
    loading: () => (
      <div className="flex-1 flex items-center justify-center bg-[#0a0a0a] text-muted-foreground">
        Loading Orbital Tracking Map...
      </div>
    )
  }
);

export default function OrbitalAwarenessPage() {
  const [selectedAsset, setSelectedAsset] = useState<any | null>(null);

  return (
    <div className="flex h-[calc(100vh-8rem)] -mx-6 -mb-6 overflow-hidden bg-background border-t">
      <div className="w-72 flex-shrink-0 hidden md:block z-10">
        <ConstellationExplorer onSelectAsset={setSelectedAsset} selectedAssetId={selectedAsset?.id || null} />
      </div>
      
      <div className="flex-1 min-w-0 relative">
        <EarthVisualization onSelectAsset={setSelectedAsset} selectedAssetId={selectedAsset?.id || null} />
      </div>
      
      <div className="flex-shrink-0 hidden lg:block border-l border-border h-full z-10">
        <OrbitalDetails asset={selectedAsset} />
      </div>
    </div>
  );
}
