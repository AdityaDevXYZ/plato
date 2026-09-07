'use client';
import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table"
import { Badge } from "@/components/ui/badge"
import { Input } from "@/components/ui/input"
import { Search } from 'lucide-react'
import { MissionsListMock } from '@/lib/mission-mock-data'

export function MissionTable() {
  const router = useRouter();
  const [search, setSearch] = useState('');
  
  const filtered = MissionsListMock.filter(m => m.name.toLowerCase().includes(search.toLowerCase()));

  return (
    <div className="space-y-4">
      <div className="flex items-center w-full max-w-sm relative">
        <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
        <Input 
          placeholder="Search missions..." 
          className="pl-9"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
      </div>
      
      <div className="rounded-md border bg-card">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Mission</TableHead>
              <TableHead>Status</TableHead>
              <TableHead>Owner</TableHead>
              <TableHead>Satellites</TableHead>
              <TableHead>Release</TableHead>
              <TableHead>Health</TableHead>
              <TableHead>Risk</TableHead>
              <TableHead className="text-right">Last Activity</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {filtered.map((mission) => (
              <TableRow 
                key={mission.id} 
                className="cursor-pointer hover:bg-muted/50"
                onClick={() => router.push(`/missions/${mission.id}`)}
              >
                <TableCell className="font-medium">{mission.name}</TableCell>
                <TableCell>
                  <Badge variant={mission.status === 'Active' ? 'success' : 'secondary'}>{mission.status}</Badge>
                </TableCell>
                <TableCell className="text-muted-foreground">{mission.owner}</TableCell>
                <TableCell>{mission.satellites}</TableCell>
                <TableCell className="text-muted-foreground">{mission.release}</TableCell>
                <TableCell>
                  <Badge variant={mission.health >= 90 ? 'success' : 'warning'}>{mission.health}%</Badge>
                </TableCell>
                <TableCell>
                  <Badge variant={mission.risk === 'Low' ? 'success' : 'warning'}>{mission.risk}</Badge>
                </TableCell>
                <TableCell className="text-right text-muted-foreground">{mission.lastActivity}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>
    </div>
  )
}
