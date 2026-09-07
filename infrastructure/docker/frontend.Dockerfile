
FROM node:20-alpine AS builder
WORKDIR /app
# Install dependencies
COPY package.json package-lock.json ./
RUN npm ci
# Build Next.js app
COPY . .
RUN npm run build --workspace=apps/web

FROM node:20-alpine AS runner
WORKDIR /app
ENV NODE_ENV=production
# Next.js telemetry disable
ENV NEXT_TELEMETRY_DISABLED=1

COPY --from=builder /app/apps/web/public ./public
COPY --from=builder /app/apps/web/.next/standalone ./
COPY --from=builder /app/apps/web/.next/static ./.next/static

EXPOSE 3000
CMD ["node", "server.js"]
