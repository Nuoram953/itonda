import { Toast } from "radix-ui";
import { cva } from "class-variance-authority";
import type { Notification } from "@/app/notificationContext";
import { useNotification } from "@/hooks/use-notification";
import {
  SlCheck,
  SlClose,
  SlExclamation,
  SlInfo,
  SlOptions,
} from "react-icons/sl";
import type { IconType } from "react-icons";

const notificationVariants = cva(
  [
    "text-foreground bg-surface-raised rounded-none shadow-lg p-4 relative border-l-32",
  ],
  {
    variants: {
      severity: {
        success: ["border-l-success"],
        error: ["border-l-danger"],
        warning: ["border-l-warning"],
        info: ["border-l-info"],
        loading: ["border-l-loading"],
      },
    },
    defaultVariants: {
      severity: "info",
    },
  },
);

interface Props {
  notification: Notification;
}

export function NotificationItem({ notification }: Props) {
  const { notify } = useNotification();

  const getIconForSeverity = (severity: Notification["severity"]) => {
    let Icon: IconType;
    switch (severity) {
      case "success":
        Icon = SlCheck;
        break;
      case "info":
        Icon = SlInfo;
        break;
      case "warning":
        Icon = SlExclamation;
        break;
      case "error":
        Icon = SlClose;
        break;
      case "loading":
        Icon = SlOptions;
        break;
    }

    return <Icon className="size-5" />;
  };

  const Icon = getIconForSeverity(notification.severity);

  return (
    <Toast.Root
      open
      duration={notification.duration}
      onOpenChange={(open) => {
        if (!open) {
          notify.dismiss(notification.id);
        }
      }}
      className={notificationVariants({ severity: notification.severity })}
    >
      <div className="absolute bottom-0 -left-8 top-0 flex w-8 items-center justify-center">
        {Icon}
      </div>

      <div className="flex w-full items-center justify-between gap-4 pr-6">
        <div className="flex flex-1 flex-col gap-1">
          <Toast.Title className="flex items-center gap-2 text-sm font-semibold">
            {notification.title}
          </Toast.Title>

          {notification.description && (
            <Toast.Description className="text-sm opacity-90">
              {notification.description}
            </Toast.Description>
          )}
        </div>

        {notification.action && (
          <Toast.Action asChild altText={notification.action.label}>
            <button
              onClick={notification.action.onClick}
              className="inline-flex h-8 shrink-0 items-center justify-center rounded-md border border-border bg-transparent px-3 text-xs font-medium transition-colors hover:bg-secondary focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
            >
              {notification.action.label}
            </button>
          </Toast.Action>
        )}
      </div>

      <Toast.Close className="absolute right-2 top-2 rounded-md p-1 opacity-50 transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="14"
          height="14"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinecap="round"
          strokeLinejoin="round"
        >
          <path d="M18 6 6 18" />
          <path d="m6 6 12 12" />
        </svg>
        <span className="sr-only">Close</span>
      </Toast.Close>
    </Toast.Root>
  );
}
