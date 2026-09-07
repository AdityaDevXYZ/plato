'use client';
import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Rocket, Lock } from 'lucide-react';
import { useAuthStore } from '@/lib/auth-store';

export default function LoginPage() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const router = useRouter();
  const login = useAuthStore(s => s.login);

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    
    // MOCK API CALL
    setTimeout(() => {
      login({
        id: 'u-123',
        email,
        name: 'System Operator',
        roles: ['Administrator'],
        orgId: 'org-1',
        orgName: 'Global Aerospace'
      }, 'mock.jwt.token.123');
      router.push('/');
      setLoading(false);
    }, 1000);
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-background p-4 relative overflow-hidden">
      {/* Background Decor */}
      <div className="absolute inset-0 z-0 opacity-20" style={{ backgroundImage: 'radial-gradient(circle at 50% 50%, hsl(var(--primary)/0.2), transparent 40%)' }}></div>
      
      <div className="w-full max-w-md z-10 bg-card border border-border rounded-xl shadow-2xl p-8 animate-in fade-in zoom-in-95 duration-500">
        <div className="flex flex-col items-center mb-8">
          <div className="h-12 w-12 bg-primary/10 rounded-full flex items-center justify-center mb-4">
            <Rocket className="h-6 w-6 text-primary" />
          </div>
          <h1 className="text-2xl font-bold tracking-tight">PLATO Identity</h1>
          <p className="text-sm text-muted-foreground mt-1">Enterprise Constellation Management</p>
        </div>

        <form onSubmit={handleLogin} className="space-y-4">
          <div className="space-y-2">
            <label className="text-sm font-medium">Email Address</label>
            <Input 
              type="email" 
              placeholder="operator@plato.aero" 
              value={email}
              onChange={e => setEmail(e.target.value)}
              required 
            />
          </div>
          <div className="space-y-2">
            <div className="flex justify-between items-center">
              <label className="text-sm font-medium">Password</label>
              <a href="#" className="text-xs text-primary hover:underline">Forgot password?</a>
            </div>
            <Input 
              type="password" 
              placeholder="••••••••" 
              value={password}
              onChange={e => setPassword(e.target.value)}
              required 
            />
          </div>
          
          <div className="flex items-center gap-2 pt-2">
            <input type="checkbox" id="remember" className="rounded bg-background accent-primary" />
            <label htmlFor="remember" className="text-sm text-muted-foreground">Remember this device</label>
          </div>

          <Button type="submit" className="w-full mt-4" disabled={loading}>
            {loading ? 'Authenticating...' : 'Sign In'}
          </Button>
        </form>
        
        <div className="mt-6 border-t border-border pt-4 text-center">
          <p className="text-xs text-muted-foreground flex items-center justify-center">
            <Lock className="h-3 w-3 mr-1" /> SSO enforced for organizational accounts
          </p>
        </div>
      </div>
    </div>
  )
}
