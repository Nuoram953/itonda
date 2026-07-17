import {
  createRootRoute,
  createRouter,
  createRoute,
  Outlet,
} from "@tanstack/react-router";
import NotFoundRoute from "./routes/not-found";
import { Home } from "@/home";
import { Header } from "@/components/ui/header/Header";
import { Sidebar } from "@/components/ui/sidebar/Siderbar";

const rootRoute = createRootRoute({
  component: () => (
    <div className="flex h-screen w-full flex-col bg-background text-foreground">
      <Header />
      <div className="flex flex-1 overflow-hidden">
        <Sidebar />
        <main className="flex-1 overflow-y-auto p-6">
          <Outlet />
        </main>
      </div>
    </div>
  ),
  notFoundComponent: () => NotFoundRoute(),
});
const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: () => <Home />,
});

const libraryRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "library",
  component: () => <h1>Library</h1>,
});

const routeTree = rootRoute.addChildren([indexRoute, libraryRoute]);

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
