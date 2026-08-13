import { Toast } from "radix-ui";
import { cva } from "class-variance-authority";
import type { Notification } from "@/app/notificationContext";
import { useNotification } from "@/hooks/use-notification";
import {
  Check,
  CircleAlert,
  Ellipsis,
  Info,
  TriangleAlert,
  X,
} from "lucide-react";
import { cn } from "@/lib/utils";

const SEVERITY_CONFIG: Record<
  Notification["severity"],
  {
    variant: string;
    badge: string;
    accent: string;
    icon: React.ComponentType<{ className?: string }>;
    iconClass?: string;
  }
> = {
  success: {
    variant: "shadow-emerald-950/20 border-emerald-500/30",
    badge: "bg-emerald-500/15 text-emerald-400 border-emerald-500/20",
    accent: "bg-gradient-to-r from-emerald-500 to-emerald-400",
    icon: Check,
  },
  error: {
    variant: "shadow-rose-950/20 border-rose-500/30",
    badge: "bg-rose-500/15 text-rose-400 border-rose-500/20",
    accent: "bg-gradient-to-r from-rose-500 to-rose-400",
    icon: TriangleAlert,
  },
  warning: {
    variant: "shadow-amber-950/20 border-amber-500/30",
    badge: "bg-amber-500/15 text-amber-400 border-amber-500/20",
    accent: "bg-gradient-to-r from-amber-500 to-amber-400",
    icon: CircleAlert,
  },
  info: {
    variant: "shadow-blue-950/20 border-blue-500/30",
    badge: "bg-blue-500/15 text-blue-400 border-blue-500/20",
    accent: "bg-gradient-to-r from-blue-500 to-blue-400",
    icon: Info,
  },
  loading: {
    variant: "shadow-slate-950/20 border-slate-500/30",
    badge: "bg-slate-500/15 text-slate-300 border-slate-500/20",
    accent: "bg-gradient-to-r from-slate-500 to-slate-400",
    icon: Ellipsis,
    iconClass: "animate-pulse",
  },
};

const notificationVariants = cva(
  "group relative flex w-full items-start gap-3.5 rounded-2xl bg-surface/90 border border-white/10 backdrop-blur-xl p-4 shadow-2xl transition-all duration-300 text-foreground overflow-hidden",
  {
    variants: {
      severity: {
        success: SEVERITY_CONFIG.success.variant,
        error: SEVERITY_CONFIG.error.variant,
        warning: SEVERITY_CONFIG.warning.variant,
        info: SEVERITY_CONFIG.info.variant,
        loading: SEVERITY_CONFIG.loading.variant,
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
  const config = SEVERITY_CONFIG[notification.severity];
  const IconComponent = config.icon;

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
      <div className={cn("absolute top-0 inset-x-0 h-0.5", config.accent)} />

      <div
        className={cn(
          "flex h-9 w-9 shrink-0 items-center justify-center rounded-xl border shadow-inner mt-0.5",
          config.badge,
        )}
      >
        <IconComponent className={cn("w-4 h-4", config.iconClass)} />
      </div>

      <div className="flex flex-1 flex-col gap-1 pr-6 min-w-0">
        <Toast.Title className="text-sm font-bold tracking-tight text-foreground leading-snug">
          {notification.title}
        </Toast.Title>

        {notification.description && (
          <Toast.Description className="text-xs text-text-muted leading-relaxed">
            {notification.description}
          </Toast.Description>
        )}

        {notification.action && (
          <div className="pt-2">
            <Toast.Action asChild altText={notification.action.label}>
              <button
                type="button"
                onClick={notification.action.onClick}
                className="inline-flex items-center justify-center rounded-xl border border-white/10 bg-surface-raised/80 px-3 py-1.5 text-xs font-semibold text-foreground transition-all hover:bg-surface-hover hover:border-white/20 focus:outline-none focus:ring-1 focus:ring-primary shadow-sm cursor-pointer"
              >
                {notification.action.label}
              </button>
            </Toast.Action>
          </div>
        )}
      </div>

      <Toast.Close className="absolute right-3 top-3.5 rounded-lg p-1 text-text-muted/60 opacity-70 transition-all hover:opacity-100 hover:text-foreground hover:bg-surface-hover focus:outline-none cursor-pointer">
        <X className="w-3.5 h-3.5" />
        <span className="sr-only">Close</span>
      </Toast.Close>
    </Toast.Root>
  );
}
