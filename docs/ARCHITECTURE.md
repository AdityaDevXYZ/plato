
# PLATO Production Architecture

## Overview
PLATO operates on a highly available, distributed cloud-native architecture deployed via Kubernetes. It utilizes a stateless Rust monolith handling intense orbital math and telemetry routing, alongside a React/Next.js frontend.

## Infrastructure Components
1. **Frontend (Next.js)**
   - Deployed as stateless containers.
   - Handles SSR/SSG and serves static assets.
   - Scaled horizontally based on CPU utilization via HPA.

2. **API Gateway / Backend Monolith (Rust / Axum)**
   - High-performance API handling REST and WebSockets.
   - Stateless design allowing massive horizontal scaling (3 to 15 pods based on load).
   - Handles real-time telemetry streaming and heavy computational loads via async workers.

3. **Database (PostgreSQL)**
   - Relational data for Identity, RBAC, Mission Plans, and Analytics.
   - Hosted via managed Cloud SQL (e.g. AWS RDS) in production for automated backups, read-replicas, and connection pooling (PgBouncer).

4. **Cache & Rate Limiting (Redis)**
   - In-memory data store for ephemeral states, JWT blocklists, and aggressive API rate limiting.
   
5. **Event Bus (NATS)**
   - High-throughput messaging system decoupling the asynchronous engines (Digital Twin, Orbital Kinematics, Deployment execution).
   - Core nervous system for inter-service communication and WebSocket multiplexing.

6. **Ingress & TLS (Nginx + Cert-Manager)**
   - Terminates HTTPS (Let's Encrypt).
   - Routes `/api/*` and WebSocket traffic to the Rust backend, and `/` traffic to the Next.js frontend.

## Deployment Strategy (CI/CD)
- **CI**: GitHub Actions runs rigorous Rust and TypeScript tests on every Pull Request.
- **CD**: Tagging a release (`v1.x.x`) triggers Docker multi-stage builds. Images are pushed to the GHCR registry.
- **GitOps**: ArgoCD monitors the `infrastructure/k8s/base` manifests. When image tags are updated, ArgoCD performs a rolling update of the pods, ensuring zero downtime.

## Observability & Reliability
- **Prometheus**: Scrapes `/health/liveness` and `/health/readiness` endpoints on the Rust backend.
- **Graceful Shutdown**: SIGTERM handlers ensure active deployments and telemetry batches are safely flushed to NATS before a pod terminates.
