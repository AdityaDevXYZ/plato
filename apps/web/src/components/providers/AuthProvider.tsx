'use client';
import { useEffect, useState } from 'react';
import { useRouter, usePathname } from 'next/navigation';
import { useAuthStore } from '@/lib/auth-store';

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const { isAuthenticated, logout } = useAuthStore();
  const router = useRouter();
  const pathname = usePathname();
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  useEffect(() => {
    if (mounted && !isAuthenticated && pathname !== '/login') {
      router.push('/login');
    }
  }, [isAuthenticated, pathname, router, mounted]);

  // Listen to the unauthorized event from api client
  useEffect(() => {
    const handleUnauthorized = () => {
      logout();
      router.push('/login');
    };
    window.addEventListener('auth:unauthorized', handleUnauthorized);
    return () => window.removeEventListener('auth:unauthorized', handleUnauthorized);
  }, [logout, router]);

  // Prevent hydration mismatch or flashing by waiting for mount
  if (!mounted) return null;

  if (!isAuthenticated && pathname !== '/login') {
    return null; // Return null to prevent flicker before redirect
  }

  return <>{children}</>;
}
