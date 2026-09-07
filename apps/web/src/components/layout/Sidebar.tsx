'use client';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { Home, Rocket, Globe, Activity, LayoutList, ShieldAlert, GitBranch, PlayCircle, BarChart3, Settings, Server } from 'lucide-react';
import { cn } from '@/lib/utils';

const navigation = [
  { name: 'Dashboard', href: '/', icon: Home },
  { name: 'Missions', href: '/missions', icon: Rocket },
  { name: 'Digital Twin', href: '/twin', icon: Globe },
  { name: 'Orbital Awareness', href: '/orbit', icon: Activity },
  { name: 'Mission Planning', href: '/planning', icon: LayoutList },
  { name: 'Risk Assessment', href: '/risk', icon: ShieldAlert },
  { name: 'Coordination', href: '/coordination', icon: GitBranch },
  { name: 'Deployments', href: '/deployments', icon: PlayCircle },
  { name: 'Analytics', href: '/analytics', icon: BarChart3 },
];

export function Sidebar() {
  const pathname = usePathname();

  return (
    <div className="flex h-full w-64 flex-col bg-card border-r border-border">
      <div className="flex h-16 items-center px-6 border-b border-border">
        <Rocket className="h-6 w-6 text-primary mr-2" />
        <span className="text-lg font-bold tracking-tight">PLATO</span>
      </div>
      
      <div className="flex-1 overflow-y-auto py-4">
        <nav className="space-y-1 px-3">
          {navigation.map((item) => {
            const isActive = pathname === item.href || pathname.startsWith(item.href + '/');
            return (
              <Link
                key={item.name}
                href={item.href}
                className={cn(
                  "group flex items-center px-3 py-2 text-sm font-medium rounded-md hover:bg-accent hover:text-accent-foreground",
                  isActive ? "bg-accent text-accent-foreground" : "text-muted-foreground"
                )}
              >
                <item.icon className="mr-3 h-5 w-5 flex-shrink-0" aria-hidden="true" />
                {item.name}
              </Link>
            )
          })}
        </nav>
      </div>
      
      <div className="border-t border-border p-4">
        <nav className="space-y-1">
          <Link href="/settings" className="group flex items-center px-3 py-2 text-sm font-medium rounded-md text-muted-foreground hover:bg-accent hover:text-accent-foreground">
            <Settings className="mr-3 h-5 w-5 flex-shrink-0" />
            Settings
          </Link>
          <Link href="/admin" className="group flex items-center px-3 py-2 text-sm font-medium rounded-md text-muted-foreground hover:bg-accent hover:text-accent-foreground">
            <Server className="mr-3 h-5 w-5 flex-shrink-0" />
            Administration
          </Link>
        </nav>
      </div>
    </div>
  );
}
