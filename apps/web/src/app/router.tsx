/* eslint-disable react-refresh/only-export-components */
import {
  createRootRoute,
  createRouter,
  createRoute,
  Outlet,
} from "@tanstack/react-router";

import NotFoundRoute from "./routes/not-found";
import { Home } from "@/home";
import { Header } from "@/components/ui/header/Header";
import { SidebarProvider, SidebarInset } from "@/components/ui/sidebar";
import { Sidebar } from "@/components/ui/sidebar/Sidebar";
import { Libary } from "@/features/library";
import { MediaDetails } from "@/features/details";

function MediaLayout() {
  return (
    <div className="relative h-full overflow-hidden">
      <Outlet />
    </div>
  );
}

const rootRoute = createRootRoute({
  component: () => (
    <SidebarProvider>
      <Sidebar />
      <SidebarInset className="flex h-svh flex-col overflow-hidden">
        <Header />

        <main className="flex-1 overflow-auto">
          <Outlet />
        </main>
      </SidebarInset>
    </SidebarProvider>
  ),
  notFoundComponent: () => NotFoundRoute(),
});

const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: () => <Home />,
});

export type MediaSearch = {
  type?: string;
};

export const mediaRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "media",
  validateSearch: (search: Record<string, unknown>): MediaSearch => {
    return {
      type: typeof search.type === "string" ? search.type : undefined,
    };
  },
  component: () => <MediaLayout />,
});

const mediaIndexRoute = createRoute({
  getParentRoute: () => mediaRoute,
  path: "/",
  component: () => <Libary />,
});

const mediaDetailsRoute = createRoute({
  getParentRoute: () => mediaRoute,
  path: "$mediaId",
  component: () => <MediaDetails />,
});

const routeTree = rootRoute.addChildren([
  indexRoute,
  mediaRoute.addChildren([mediaIndexRoute, mediaDetailsRoute]),
]);

export const router = createRouter({
  routeTree,
  defaultPreload: "intent",
  defaultPreloadStaleTime: 0,
  defaultStaleTime: 5000,
  scrollRestoration: true,
  defaultErrorComponent: ({ error, reset }) => (
    <div className="p-8 text-foreground">
      <h3 className="mb-2 text-lg font-semibold text-danger">
        Failed to load page
      </h3>

      <p className="mb-4 text-sm text-text-muted">{error.message}</p>

      <button
        onClick={() => reset()}
        className="rounded border border-border-strong bg-surface px-3 py-1.5 text-sm hover:bg-surface-hover"
      >
        Try again
      </button>
    </div>
  ),
});
