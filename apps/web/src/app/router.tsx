import {
  createRootRoute,
  createRouter,
  createRoute,
  Link,
  Outlet,
} from "@tanstack/react-router";
import NotFoundRoute from "./routes/not-found";
import { Home } from "@/home";

const rootRoute = createRootRoute({
  component: () => (
    <>
      <nav>
        <Link to="/">Home</Link>
        <Link to="/library">Library</Link>
      </nav>

      <Outlet />
    </>
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
