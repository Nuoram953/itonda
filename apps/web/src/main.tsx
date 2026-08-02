import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";

import { RouterProvider } from "@tanstack/react-router";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { NotificationProvider } from "./app/notificationProvider";
import { NotificationViewport } from "./components/ui/notification/Viewport";
import { ErrorBoundary } from "react-error-boundary";
import { RootErrorFallback } from "./app/routes/error";
import { AxiosInterceptor } from "./lib/AxiosInterceptor";
import { router } from "./app/router";
import { WebSocketProvider } from "./app/websocketProvider";
import { ThemeProvider } from "./components/theme-provider";

const queryClient = new QueryClient();

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <ErrorBoundary
        FallbackComponent={RootErrorFallback}
        onReset={() => window.location.reload()}
      >
        <QueryClientProvider client={queryClient}>
          <NotificationProvider>
            <WebSocketProvider>
              <AxiosInterceptor>
                <RouterProvider router={router} />
              </AxiosInterceptor>
              <NotificationViewport />
            </WebSocketProvider>
          </NotificationProvider>
        </QueryClientProvider>
      </ErrorBoundary>
    </ThemeProvider>
  </StrictMode>,
);
