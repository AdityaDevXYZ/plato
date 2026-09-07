'use client';
import { useParams } from 'next/navigation'
import { getMissionDetails } from '@/lib/mission-mock-data'
import { MissionHeader } from '@/features/missions/MissionHeader'
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { OverviewTab } from '@/features/missions/tabs/OverviewTab'
import { AssetsTab } from '@/features/missions/tabs/AssetsTab'
import { PlansTab } from '@/features/missions/tabs/PlansTab'
import { RiskTab } from '@/features/missions/tabs/RiskTab'
import { CoordinationTab } from '@/features/missions/tabs/CoordinationTab'
import { TimelineTab } from '@/features/missions/tabs/TimelineTab'
import { SettingsTab } from '@/features/missions/tabs/SettingsTab'

export default function MissionDetailPage() {
  const { id } = useParams()
  const mission = getMissionDetails(id as string)

  return (
    <div className="space-y-4 pb-8">
      <MissionHeader mission={mission} />
      
      <Tabs defaultValue="overview" className="w-full">
        <TabsList className="mb-4 flex justify-start w-full bg-transparent border-b rounded-none p-0 overflow-x-auto h-12">
          <TabsTrigger value="overview" className="data-[state=active]:border-b-2 data-[state=active]:border-primary rounded-none shadow-none h-full bg-transparent px-4">Overview</TabsTrigger>
          <TabsTrigger value="assets" className="data-[state=active]:border-b-2 data-[state=active]:border-primary rounded-none shadow-none h-full bg-transparent px-4">Assets</TabsTrigger>
          <TabsTrigger value="plans" className="data-[state=active]:border-b-2 data-[state=active]:border-primary rounded-none shadow-none h-full bg-transparent px-4">Deployment Plans</TabsTrigger>
          <TabsTrigger value="risk" className="data-[state=active]:border-b-2 data-[state=active]:border-primary rounded-none shadow-none h-full bg-transparent px-4">Risk</TabsTrigger>
          <TabsTrigger value="coordination" className="data-[state=active]:border-b-2 data-[state=active]:border-primary rounded-none shadow-none h-full bg-transparent px-4">Coordination</TabsTrigger>
          <TabsTrigger value="timeline" className="data-[state=active]:border-b-2 data-[state=active]:border-primary rounded-none shadow-none h-full bg-transparent px-4">Timeline</TabsTrigger>
          <TabsTrigger value="settings" className="data-[state=active]:border-b-2 data-[state=active]:border-primary rounded-none shadow-none h-full bg-transparent px-4">Settings</TabsTrigger>
        </TabsList>
        
        <TabsContent value="overview"><OverviewTab mission={mission} /></TabsContent>
        <TabsContent value="assets"><AssetsTab mission={mission} /></TabsContent>
        <TabsContent value="plans"><PlansTab /></TabsContent>
        <TabsContent value="risk"><RiskTab /></TabsContent>
        <TabsContent value="coordination"><CoordinationTab /></TabsContent>
        <TabsContent value="timeline"><TimelineTab /></TabsContent>
        <TabsContent value="settings"><SettingsTab /></TabsContent>
      </Tabs>
    </div>
  )
}
