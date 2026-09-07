'use client';
import { Wrench } from 'lucide-react'

export function PlaceholderTab({ title, desc }: { title: string, desc: string }) {
  return (
    <div className="p-6 h-full flex flex-col items-center justify-center text-center max-w-3xl mx-auto">
      <Wrench className="h-12 w-12 text-muted-foreground opacity-20 mb-4" />
      <h2 className="text-2xl font-bold mb-2">{title}</h2>
      <p className="text-muted-foreground">{desc}</p>
      <p className="text-xs text-muted-foreground mt-4 border border-border p-2 rounded bg-muted/20">This section is a placeholder for future Enterprise implementations.</p>
    </div>
  )
}
