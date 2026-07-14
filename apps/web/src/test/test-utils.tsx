import { type ReactElement, type ReactNode } from "react";
import {
  render as rtlRender,
  type RenderOptions,
} from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ErrorBoundary } from "react-error-boundary";

import { NotificationProvider } from "@/app/notificationProvider";
import { NotificationViewport } from "@/components/ui/notification/Viewport";
import { AxiosInterceptor } from "@/lib/AxiosInterceptor";
import { RootErrorFallback } from "@/app/routes/error";
import { Toast } from "radix-ui";

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
              <AxiosInterceptor>{children}</AxiosInterceptor>
              <Toast.ToastViewport />
              <NotificationViewport />
            </NotificationProvider>
          </QueryClientProvider>
        </ErrorBoundary>
      </Toast.Provider>
    );
  }

  return rtlRender(ui, { wrapper: Wrapper, ...options });
}

/* eslint-disable react-refresh/only-export-components */
export * from "@testing-library/react";

export { renderWithProviders as render };
