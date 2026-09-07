'use client';
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { RiskCategories, RiskTrendData, CategoryComparisonData } from '@/lib/risk-mock-data'
import { LineChart, Line, ResponsiveContainer, XAxis, Tooltip, RadarChart, PolarGrid, PolarAngleAxis, PolarRadiusAxis, Radar } from 'recharts'
import { ShieldAlert, AlertTriangle, CheckCircle } from 'lucide-react'

export function RiskDashboard() {
  return (
    <div className="flex flex-col h-full bg-background border-r overflow-y-auto">
      <div className="p-6 border-b bg-card flex-shrink-0">
        <h2 className="text-2xl font-bold">Risk Assessment Dashboard</h2>
        <p className="text-sm text-muted-foreground mt-1">Holistic view of constellation deployment risk.</p>
        
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mt-6">
          <Card className="shadow-none border-border">
            <CardHeader className="pb-2"><CardTitle className="text-sm font-medium">Overall Risk Score</CardTitle></CardHeader>
            <CardContent>
              <div className="text-3xl font-bold text-yellow-500 flex items-center gap-2">
                72 <AlertTriangle className="h-6 w-6" />
              </div>
              <p className="text-xs text-muted-foreground mt-1">Medium Risk Profile</p>
            </CardContent>
          </Card>
          <Card className="shadow-none border-border md:col-span-2">
            <CardHeader className="pb-2"><CardTitle className="text-sm font-medium">Risk Trend (Last 7 Days)</CardTitle></CardHeader>
            <CardContent className="h-[80px] p-0 px-4">
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={RiskTrendData}>
                  <XAxis dataKey="day" hide />
                  <Tooltip contentStyle={{ backgroundColor: 'hsl(var(--card))', borderColor: 'hsl(var(--border))' }} />
                  <Line type="monotone" dataKey="risk" stroke="hsl(var(--destructive))" strokeWidth={2} dot={false} />
                </LineChart>
              </ResponsiveContainer>
            </CardContent>
          </Card>
        </div>
      </div>
      
      <div className="p-6 grid grid-cols-1 xl:grid-cols-2 gap-6 bg-muted/20 flex-1">
        <div className="space-y-4">
          <h3 className="font-semibold text-sm">Category Breakdown</h3>
          <div className="grid grid-cols-1 gap-3">
            {RiskCategories.map((cat, i) => (
              <div key={i} className="bg-card border border-border p-3 rounded-lg flex items-start gap-3 shadow-sm hover:border-primary/50 transition-colors cursor-pointer">
                <div className="mt-0.5 flex-shrink-0">
                  {cat.severity === 'High' && <ShieldAlert className="h-5 w-5 text-destructive" />}
                  {cat.severity === 'Medium' && <AlertTriangle className="h-5 w-5 text-yellow-500" />}
                  {cat.severity === 'Low' && <CheckCircle className="h-5 w-5 text-green-500" />}
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex justify-between items-center mb-1">
                    <span className="font-medium text-sm truncate">{cat.name}</span>
                    <Badge variant="outline" className="flex-shrink-0">{cat.score}/100</Badge>
                  </div>
                  <p className="text-xs text-muted-foreground line-clamp-1">{cat.summary}</p>
                </div>
              </div>
            ))}
          </div>
        </div>

        <div className="space-y-4">
          <h3 className="font-semibold text-sm">Historical Comparison</h3>
          <Card className="shadow-none border-border">
            <CardContent className="p-6 h-[300px]">
              <ResponsiveContainer width="100%" height="100%">
                <RadarChart cx="50%" cy="50%" outerRadius="80%" data={CategoryComparisonData}>
                  <PolarGrid stroke="hsl(var(--border))" />
                  <PolarAngleAxis dataKey="subject" tick={{ fill: 'hsl(var(--muted-foreground))', fontSize: 10 }} />
                  <PolarRadiusAxis angle={30} domain={[0, 150]} tick={false} axisLine={false} />
                  <Radar name="Current" dataKey="A" stroke="hsl(var(--primary))" fill="hsl(var(--primary))" fillOpacity={0.4} />
                  <Radar name="Previous" dataKey="B" stroke="hsl(var(--muted-foreground))" fill="hsl(var(--muted-foreground))" fillOpacity={0.2} />
                  <Tooltip contentStyle={{ backgroundColor: 'hsl(var(--card))', borderColor: 'hsl(var(--border))' }} />
                </RadarChart>
              </ResponsiveContainer>
            </CardContent>
          </Card>

          <Card className="shadow-none border-border">
            <CardContent className="p-4 space-y-2 text-sm">
              <div className="flex justify-between items-center text-green-500">
                <span>Improved: Operational</span> <span>+10 pts</span>
              </div>
              <div className="flex justify-between items-center text-destructive">
                <span>Regressed: Resource</span> <span>-44 pts</span>
              </div>
              <div className="flex justify-between items-center text-muted-foreground">
                <span>Unchanged: Communication</span> <span>0 pts</span>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  )
}
