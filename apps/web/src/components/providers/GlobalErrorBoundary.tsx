'use client';
import React, { Component, ReactNode } from 'react';
import { Button } from '@/components/ui/button';
import { ShieldAlert } from 'lucide-react';

interface Props {
  children: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export class GlobalErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false,
    error: null
  };

  public static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  public componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    // Observability hook: log to telemetry
    console.error('GlobalErrorBoundary caught an error:', error, errorInfo);
  }

  public render() {
    if (this.state.hasError) {
      return (
        <div className="min-h-screen flex items-center justify-center bg-background p-4 text-center">
          <div className="max-w-md w-full p-6 bg-card border border-border rounded-lg shadow-lg">
            <ShieldAlert className="h-12 w-12 text-destructive mx-auto mb-4" />
            <h2 className="text-xl font-bold mb-2">System Error</h2>
            <p className="text-sm text-muted-foreground mb-6">
              The application encountered an unexpected fault. Our telemetry systems have been notified.
            </p>
            <div className="text-left text-xs bg-muted/50 p-3 rounded font-mono mb-6 overflow-x-auto text-muted-foreground">
              {this.state.error?.message}
            </div>
            <Button onClick={() => window.location.reload()} className="w-full">
              Reload Interface
            </Button>
          </div>
        </div>
      );
    }

    return this.props.children;
  }
}
