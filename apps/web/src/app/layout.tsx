import type { Metadata } from "next";
import "./globals.css";
import { cn } from "@/lib/utils";
import { AppShell } from "@/components/layout/AppShell";
import { Providers } from "@/components/providers/Providers";
import { GlobalErrorBoundary } from "@/components/providers/GlobalErrorBoundary";
import { AuthProvider } from "@/components/providers/AuthProvider";

export const metadata: Metadata = {
  title: "PLATO | Mission Control",
  description: "Enterprise Constellation Management Platform",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="dark">
      <body className={cn("bg-background text-foreground antialiased selection:bg-primary/30 font-sans")}>
        <GlobalErrorBoundary>
          <Providers>
            <AuthProvider>
              <AppShell>
                {children}
              </AppShell>
            </AuthProvider>
          </Providers>
        </GlobalErrorBoundary>
      </body>
    </html>
  );
}
