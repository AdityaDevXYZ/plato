'use client';
import { Button } from "@/components/ui/button"
import { IntegrationsData, WebhooksData } from '@/lib/admin-mock-data'
import { Plug, Webhook, Plus } from 'lucide-react'

export function IntegrationsTab() {
  return (
    <div className="p-6 space-y-8 max-w-5xl">
      <section className="space-y-4">
        <div>
          <h2 className="text-2xl font-bold">Ecosystem Integrations</h2>
          <p className="text-sm text-muted-foreground mt-1">Manage connections to external systems and hardware APIs.</p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {IntegrationsData.map(int => (
            <div key={int.id} className="bg-card border border-border rounded-lg p-5 flex items-center justify-between">
              <div className="flex items-start gap-4">
                <div className="h-10 w-10 bg-primary/10 text-primary flex items-center justify-center rounded-lg flex-shrink-0">
                  <Plug className="h-5 w-5" />
                </div>
                <div>
                  <h3 className="font-semibold">{int.name}</h3>
                  <p className="text-xs text-muted-foreground mt-1">{int.desc}</p>
                  <span className={`inline-block mt-2 text-[10px] uppercase font-bold tracking-wider ${int.status === 'Connected' ? 'text-green-500' : int.status === 'Disconnected' ? 'text-destructive' : 'text-yellow-500'}`}>
                    • {int.status}
                  </span>
                </div>
              </div>
              <Button variant="outline" size="sm">Configure</Button>
            </div>
          ))}
        </div>
      </section>

      <section className="space-y-4 border-t border-border pt-8">
        <div className="flex justify-between items-center">
          <div>
            <h2 className="text-2xl font-bold">Webhooks</h2>
            <p className="text-sm text-muted-foreground mt-1">Broadcast PLATO events to external HTTPS endpoints.</p>
          </div>
          <Button><Plus className="h-4 w-4 mr-2" /> Add Webhook</Button>
        </div>

        <div className="grid grid-cols-1 gap-4">
          {WebhooksData.map(wh => (
            <div key={wh.id} className="bg-card border border-border rounded-lg p-4 flex justify-between items-center">
              <div className="flex items-center gap-3">
                <Webhook className="h-5 w-5 text-muted-foreground" />
                <div>
                  <div className="font-mono text-sm">{wh.url}</div>
                  <div className="text-xs text-muted-foreground mt-1">Events: {wh.events.join(', ')}</div>
                </div>
              </div>
              <Button variant="ghost" size="sm">Edit</Button>
            </div>
          ))}
        </div>
      </section>
    </div>
  )
}
