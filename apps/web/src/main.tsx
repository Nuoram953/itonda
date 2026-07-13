import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";

import {
  createRootRoute,
  createRouter,
  createRoute,
  RouterProvider,
  Link,
  Outlet,
} from "@tanstack/react-router";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Home } from "./home";

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

  notFoundComponent: () => (
    <div>
      <p>Page not found</p>
      <Link to="/">Start Over</Link>
    </div>
  ),
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

const router = createRouter({
  routeTree,
  defaultPreload: "intent",
  defaultPreloadStaleTime: 0,
  defaultStaleTime: 5000,
  scrollRestoration: true,
});

const queryClient = new QueryClient();

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  </StrictMode>,
);
