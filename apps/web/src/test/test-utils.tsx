import { type ReactElement, type ReactNode } from "react";
import {
  render as rtlRender,
  type RenderOptions,
} from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ErrorBoundary } from "react-error-boundary";

import { NotificationProvider } from "@/app/notificationProvider";
import { ActiveMediaProvider } from "@/app/activeMediaProvider";
import { NotificationViewport } from "@/components/ui/notification/Viewport";
import { AxiosInterceptor } from "@/lib/AxiosInterceptor";
import { RootErrorFallback } from "@/app/routes/error";
import { WebSocketContext } from "@/app/websocketProvider";
import type { AppWebSocket } from "@/lib/websocket/client";
import { Toast } from "radix-ui";
import {
  createMemoryHistory,
  createRootRoute,
  createRouter,
  RouterProvider,
} from "@tanstack/react-router";

import { SidebarProvider } from "@/components/ui/sidebar";

const mockWebSocket = {
  getStatus: () => "connected" as const,
  onStatusChange: (handler: (status: "connected") => void) => {
    handler("connected");
    return () => {};
  },
  connect: () => {},
  disconnect: () => {},
  send: () => {},
  on: () => () => {},
  off: () => {},
  registerHandler: () => () => {},
} as unknown as AppWebSocket;

function renderWithProviders(
  ui: ReactElement,
  options?: Omit<RenderOptions, "wrapper">,
) {
  const testQueryClient = new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
      },
    },
  });

  function Wrapper({ children }: { children: ReactNode }) {
    return (
      <Toast.Provider>
        <ErrorBoundary FallbackComponent={RootErrorFallback} onReset={() => {}}>
          <QueryClientProvider client={testQueryClient}>
            <NotificationProvider>
              <ActiveMediaProvider>
                <WebSocketContext.Provider value={mockWebSocket}>
                  <SidebarProvider>
                    <AxiosInterceptor>{children}</AxiosInterceptor>
                    <Toast.ToastViewport />
                    <NotificationViewport />
                  </SidebarProvider>
                </WebSocketContext.Provider>
              </ActiveMediaProvider>
            </NotificationProvider>
          </QueryClientProvider>
        </ErrorBoundary>
      </Toast.Provider>
    );
  }

  return rtlRender(ui, { wrapper: Wrapper, ...options });
}

export const renderWithRouter = (component: React.ReactNode) => {
  const rootRoute = createRootRoute({
    component: () => <SidebarProvider>{component}</SidebarProvider>,
  });

  const router = createRouter({
    routeTree: rootRoute,
    history: createMemoryHistory(),
  });

  return renderWithProviders(<RouterProvider router={router} />);
};

/* eslint-disable react-refresh/only-export-components */
export * from "@testing-library/react";

export { renderWithProviders as render };
