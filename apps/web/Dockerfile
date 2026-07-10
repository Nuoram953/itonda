FROM node:22

RUN corepack enable

WORKDIR /app

COPY pnpm-workspace.yaml pnpm-lock.yaml package.json ./

COPY apps/web/package.json ./apps/web/package.json

RUN cat pnpm-workspace.yaml
RUN pnpm --version
RUN pnpm install --frozen-lockfile --filter ./apps/web

COPY apps/web ./apps/web

WORKDIR /app/apps/web

EXPOSE 5173

CMD ["pnpm", "dev", "--host", "0.0.0.0"]
