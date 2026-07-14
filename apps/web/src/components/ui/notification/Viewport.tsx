import { Toast } from "radix-ui";

import { useNotification } from "../../../hooks/use-notification";
import { NotificationItem } from "./Item.tsx";

export function NotificationViewport() {
  const { notifications } = useNotification();

  return (
    <Toast.Provider>
      {notifications.map((notification) => (
        <NotificationItem key={notification.id} notification={notification} />
      ))}

      <Toast.Viewport
        className="
          fixed
          bottom-4
          right-4
          z-50
          flex
          w-96
          flex-col
          gap-2
          outline-none
        "
      />
    </Toast.Provider>
  );
}
