'use client';
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { SuccessTrendData, StrategyComparisonData, FailureDistributionData } from '@/lib/analytics-mock-data'
import { LineChart, Line, BarChart, Bar, PieChart, Pie, Cell, ResponsiveContainer, XAxis, YAxis, Tooltip, CartesianGrid } from 'recharts'
import { ArrowUpRight, ArrowDownRight, Target, Clock, ShieldAlert, Activity } from 'lucide-react'

const COLORS = ['hsl(var(--destructive))', 'hsl(var(--warning))', 'hsl(var(--primary))', 'hsl(var(--success))'];

export function AnalyticsDashboard() {
  return (
    <div className="flex flex-col h-full bg-background border-r overflow-y-auto">
      <div className="p-6 border-b bg-card flex-shrink-0">
        <h2 className="text-2xl font-bold">Mission Intelligence Center</h2>
        <p className="text-sm text-muted-foreground mt-1">Operational insights and performance benchmarks.</p>
        
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mt-6">
          <Card className="shadow-none border-border">
            <CardHeader className="p-4 pb-2"><CardTitle className="text-xs font-medium text-muted-foreground flex items-center justify-between">Success Rate <Target className="h-4 w-4" /></CardTitle></CardHeader>
            <CardContent className="p-4 pt-0">
              <div className="text-2xl font-bold text-green-500">98.5%</div>
              <p className="text-xs text-green-500 flex items-center mt-1"><ArrowUpRight className="h-3 w-3 mr-1" /> +1.2% (YTD)</p>
            </CardContent>
          </Card>
          <Card className="shadow-none border-border">
            <CardHeader className="p-4 pb-2"><CardTitle className="text-xs font-medium text-muted-foreground flex items-center justify-between">Avg Rollout <Clock className="h-4 w-4" /></CardTitle></CardHeader>
            <CardContent className="p-4 pt-0">
              <div className="text-2xl font-bold">4h 15m</div>
              <p className="text-xs text-green-500 flex items-center mt-1"><ArrowDownRight className="h-3 w-3 mr-1" /> -12m (YTD)</p>
            </CardContent>
          </Card>
          <Card className="shadow-none border-border">
            <CardHeader className="p-4 pb-2"><CardTitle className="text-xs font-medium text-muted-foreground flex items-center justify-between">Fleet Reliability <ShieldAlert className="h-4 w-4" /></CardTitle></CardHeader>
            <CardContent className="p-4 pt-0">
              <div className="text-2xl font-bold">99.9%</div>
              <p className="text-xs text-muted-foreground flex items-center mt-1">Nominal state</p>
            </CardContent>
          </Card>
          <Card className="shadow-none border-border">
            <CardHeader className="p-4 pb-2"><CardTitle className="text-xs font-medium text-muted-foreground flex items-center justify-between">Adoption <Activity className="h-4 w-4" /></CardTitle></CardHeader>
            <CardContent className="p-4 pt-0">
              <div className="text-2xl font-bold">84%</div>
              <p className="text-xs text-muted-foreground flex items-center mt-1">Latest v3 Payload</p>
            </CardContent>
          </Card>
        </div>
      </div>
      
      <div className="p-6 grid grid-cols-1 xl:grid-cols-2 gap-6 bg-muted/20 flex-1">
        
        <Card className="shadow-none border-border">
          <CardHeader>
            <CardTitle className="text-sm font-semibold">Deployment Success Trend</CardTitle>
          </CardHeader>
          <CardContent className="h-[250px]">
            <ResponsiveContainer width="100%" height="100%">
              <LineChart data={SuccessTrendData}>
                <CartesianGrid strokeDasharray="3 3" stroke="hsl(var(--border))" vertical={false} />
                <XAxis dataKey="month" stroke="hsl(var(--muted-foreground))" fontSize={12} tickLine={false} axisLine={false} />
                <YAxis domain={[90, 100]} stroke="hsl(var(--muted-foreground))" fontSize={12} tickLine={false} axisLine={false} tickFormatter={(val) => `${val}%`} />
                <Tooltip contentStyle={{ backgroundColor: 'hsl(var(--card))', borderColor: 'hsl(var(--border))' }} />
                <Line type="monotone" dataKey="rate" stroke="hsl(var(--primary))" strokeWidth={3} dot={{ r: 4, fill: 'hsl(var(--primary))' }} />
              </LineChart>
            </ResponsiveContainer>
          </CardContent>
        </Card>

        <Card className="shadow-none border-border">
          <CardHeader>
            <CardTitle className="text-sm font-semibold">Failure Distribution</CardTitle>
          </CardHeader>
          <CardContent className="h-[250px] flex items-center">
            <ResponsiveContainer width="100%" height="100%">
              <PieChart>
                <Pie data={FailureDistributionData} cx="50%" cy="50%" innerRadius={60} outerRadius={80} paddingAngle={5} dataKey="value">
                  {FailureDistributionData.map((entry, index) => (
                    <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                  ))}
                </Pie>
                <Tooltip contentStyle={{ backgroundColor: 'hsl(var(--card))', borderColor: 'hsl(var(--border))' }} />
              </PieChart>
            </ResponsiveContainer>
            <div className="w-1/3 flex flex-col gap-2 justify-center">
              {FailureDistributionData.map((d, i) => (
                <div key={i} className="flex items-center gap-2 text-xs">
                  <span className="w-3 h-3 rounded-full flex-shrink-0" style={{ backgroundColor: COLORS[i % COLORS.length] }}></span>
                  <span className="text-muted-foreground truncate">{d.name} ({d.value}%)</span>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>

        <Card className="shadow-none border-border xl:col-span-2">
          <CardHeader>
            <CardTitle className="text-sm font-semibold">Strategy Performance Comparison</CardTitle>
          </CardHeader>
          <CardContent className="h-[250px]">
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={StrategyComparisonData}>
                <CartesianGrid strokeDasharray="3 3" stroke="hsl(var(--border))" vertical={false} />
                <XAxis dataKey="name" stroke="hsl(var(--muted-foreground))" fontSize={12} tickLine={false} axisLine={false} />
                <YAxis yAxisId="left" stroke="hsl(var(--muted-foreground))" fontSize={12} tickLine={false} axisLine={false} />
                <YAxis yAxisId="right" orientation="right" domain={[0, 100]} stroke="hsl(var(--muted-foreground))" fontSize={12} tickLine={false} axisLine={false} />
                <Tooltip contentStyle={{ backgroundColor: 'hsl(var(--card))', borderColor: 'hsl(var(--border))' }} />
                <Bar yAxisId="left" dataKey="duration" name="Avg Duration (min)" fill="hsl(var(--primary))" radius={[4, 4, 0, 0]} />
                <Bar yAxisId="right" dataKey="success" name="Success Rate (%)" fill="hsl(var(--success))" radius={[4, 4, 0, 0]} />
              </BarChart>
            </ResponsiveContainer>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
