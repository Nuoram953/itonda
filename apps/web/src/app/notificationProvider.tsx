import { useState, useCallback, useMemo, type ReactNode } from "react";
import {
  NotificationContext,
  type Notification,
  type NotificationInput,
  type NotificationSeverity,
} from "./notificationContext"; // Import context and types from the new file

export function NotificationProvider({ children }: { children: ReactNode }) {
  const [notifications, setNotifications] = useState<Notification[]>([]);

  const add = useCallback(
    (severity: NotificationSeverity, input: NotificationInput) => {
      const notification: Notification = {
        id: crypto.randomUUID(),
        severity,
        dismissible: true,
        duration: severity === "loading" ? undefined : 5000,
        ...input,
      };

      setNotifications((current) => [...current, notification]);
      return notification.id;
    },
    [],
  );

  const update = useCallback((id: string, changes: Partial<Notification>) => {
    setNotifications((current) =>
      current.map((notification) =>
        notification.id === id ? { ...notification, ...changes } : notification,
      ),
    );
  }, []);

  const updateBySourceId = useCallback(
    (sourceId: string, changes: Partial<Notification>) => {
      setNotifications((current) =>
        current.map((notification) =>
          notification.sourceId === sourceId
            ? { ...notification, ...changes }
            : notification,
        ),
      );
    },
    [],
  );

  const dismiss = useCallback((id: string) => {
    setNotifications((current) =>
      current.filter((notification) => notification.id !== id),
    );
  }, []);

  const dismissBySourceId = useCallback((sourceId: string) => {
    setNotifications((current) =>
      current.filter((notification) => notification.sourceId !== sourceId),
    );
  }, []);

  const value = useMemo(
    () => ({
      notifications,
      notify: {
        success: (input: NotificationInput) => add("success", input),
        info: (input: NotificationInput) => add("info", input),
        warning: (input: NotificationInput) => add("warning", input),
        error: (input: NotificationInput) => add("error", input),
        loading: (input: NotificationInput) => add("loading", input),
        update,
        updateBySourceId,
        dismiss,
        dismissBySourceId,
      },
    }),
    [notifications, add, update, updateBySourceId, dismiss, dismissBySourceId],
  );

  return (
    <NotificationContext.Provider value={value}>
      {children}
    </NotificationContext.Provider>
  );
}
