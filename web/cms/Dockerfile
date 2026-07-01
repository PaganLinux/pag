# ─── Build stage ────────────────────────────────────────
FROM node:22-alpine AS builder

WORKDIR /app
COPY package.json package-lock.json* ./
RUN npm ci

COPY . .
RUN npm run build

# ─── Runtime stage ──────────────────────────────────────
FROM node:22-alpine

WORKDIR /app
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/package.json .

ENV HOST=0.0.0.0
ENV PORT=3006

EXPOSE 3006

CMD ["node", "dist/server/entry.mjs"]
