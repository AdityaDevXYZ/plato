'use client';
import { useState } from 'react';
import { DeploymentQueue } from '@/features/deployments/DeploymentQueue'
import { DeploymentMonitor } from '@/features/deployments/DeploymentMonitor'
import { DeploymentDetails } from '@/features/deployments/DeploymentDetails'

export default function DeploymentsPage() {
  const [selectedDeployment, setSelectedDeployment] = useState<string | null>('dep-001');

  return (
    <div className="flex h-[calc(100vh-8rem)] -mx-6 -mb-6 overflow-hidden bg-background border-t animate-in fade-in duration-500">
      <div className="w-80 flex-shrink-0 z-10 hidden md:block">
        <DeploymentQueue onSelect={setSelectedDeployment} selectedId={selectedDeployment} />
      </div>
      
      <div className="flex-1 min-w-0">
        <DeploymentMonitor />
      </div>
      
      <div className="w-80 flex-shrink-0 border-l border-border z-10 hidden xl:block">
        <DeploymentDetails id={selectedDeployment} />
      </div>
    </div>
  )
}
