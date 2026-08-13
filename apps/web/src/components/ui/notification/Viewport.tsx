import { Toast } from "radix-ui";

import { useNotification } from "@/hooks/use-notification";
import { NotificationItem } from "./Item";

export function NotificationViewport() {
  const { notifications } = useNotification();

  return (
    <Toast.Provider>
      {notifications.map((notification) => (
        <NotificationItem key={notification.id} notification={notification} />
      ))}

      <Toast.Viewport className="fixed bottom-5 right-5 z-50 flex w-96 max-w-[calc(100vw-2.5rem)] flex-col gap-3 outline-none pointer-events-none *:pointer-events-auto" />
    </Toast.Provider>
  );
}

