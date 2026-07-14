import { NotificationContext } from "@/app/notificationContext";
import { useContext } from "react";

export function useNotification() {
  const context = useContext(NotificationContext);

  if (!context) {
    throw new Error("useNotification must be used inside NotificationProvider");
  }

  return context;
}
