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
      const defaultDuration = severity === "loading" ? Infinity : 5000;
      const notification: Notification = {
        id: crypto.randomUUID(),
        severity,
        dismissible: true,
        ...input,
        duration: input.duration ?? defaultDuration,
      };

      setNotifications((current) => [...current, notification]);
      return notification.id;
    },
    [],
  );

  const update = useCallback((id: string, changes: Partial<Notification>) => {
    setNotifications((current) =>
      current.map((notification) => {
        if (notification.id !== id) {
          return notification;
        }

        const updatedSeverity = changes.severity ?? notification.severity;
        let updatedDuration =
          changes.duration !== undefined
            ? changes.duration
            : notification.duration;

        if (
          notification.severity === "loading" &&
          updatedSeverity !== "loading" &&
          changes.duration === undefined
        ) {
          updatedDuration = 5000;
        }

        return {
          ...notification,
          ...changes,
          duration: updatedDuration,
        };
      }),
    );
  }, []);

  const updateBySourceId = useCallback(
    (sourceId: string, changes: Partial<Notification>) => {
      setNotifications((current) => {
        const exists = current.some((n) => n.sourceId === sourceId);
        if (!exists) {
          const severity = changes.severity ?? "loading";
          const defaultDuration = severity === "loading" ? Infinity : 5000;
          const notification: Notification = {
            id: crypto.randomUUID(),
            severity,
            dismissible: true,
            title: changes.title ?? "Syncing Library",
            description: changes.description,
            ...changes,
            duration: changes.duration ?? defaultDuration,
            sourceId,
          };
          return [...current, notification];
        }

        return current.map((notification) => {
          if (notification.sourceId !== sourceId) {
            return notification;
          }

          const updatedSeverity = changes.severity ?? notification.severity;
          let updatedDuration =
            changes.duration !== undefined
              ? changes.duration
              : notification.duration;

          if (
            notification.severity === "loading" &&
            updatedSeverity !== "loading" &&
            changes.duration === undefined
          ) {
            updatedDuration = 5000;
          }

          return {
            ...notification,
            ...changes,
            duration: updatedDuration,
          };
        });
      });
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
