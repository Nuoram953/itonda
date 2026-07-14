import { createContext, useContext } from "react";

export type NotificationSeverity =
  | "success"
  | "info"
  | "warning"
  | "error"
  | "loading";

export interface Notification {
  id: string;
  severity: NotificationSeverity;
  dismissible?: boolean;
  duration?: number;
  sourceId?: string;
  title: string;
  description?: string;
  action?: {
    label: string;
    onClick: () => void;
  };
}

export type NotificationInput = Omit<Notification, "id" | "severity">;

export interface NotificationContextValue {
  notifications: Notification[];
  notify: {
    success: (input: NotificationInput) => string;
    info: (input: NotificationInput) => string;
    warning: (input: NotificationInput) => string;
    error: (input: NotificationInput) => string;
    loading: (input: NotificationInput) => string;
    update: (id: string, changes: Partial<Notification>) => void;
    updateBySourceId: (
      sourceId: string,
      changes: Partial<Notification>,
    ) => void;
    dismiss: (id: string) => void;
    dismissBySourceId: (sourceId: string) => void;
  };
}

export const NotificationContext =
  createContext<NotificationContextValue | null>(null);

export function useNotification() {
  const context = useContext(NotificationContext);
  if (!context) {
    throw new Error(
      "useNotification must be used within a NotificationProvider",
    );
  }
  return context;
}
