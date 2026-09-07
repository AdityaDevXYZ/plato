import { TargetSelection } from '@/features/planning/TargetSelection'
import { PlanBuilder } from '@/features/planning/PlanBuilder'
import { PlanSummary } from '@/features/planning/PlanSummary'

export default function MissionPlanningPage() {
  return (
    <div className="flex h-[calc(100vh-8rem)] -mx-6 -mb-6 overflow-hidden bg-background border-t animate-in fade-in duration-500">
      <div className="w-80 flex-shrink-0 z-10 hidden lg:block">
        <TargetSelection />
      </div>
      
      <div className="flex-1 min-w-0">
        <PlanBuilder />
      </div>
      
      <div className="w-80 flex-shrink-0 border-l border-border z-10 hidden xl:block">
        <PlanSummary />
      </div>
    </div>
  )
}
