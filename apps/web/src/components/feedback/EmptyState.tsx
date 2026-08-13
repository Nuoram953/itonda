import * as React from "react";
import { Workspace } from "@/components/workspace/Workspace";
import { cn } from "@/lib/utils";

export type EmptyStateProps = {
  title?: string;
  message?: string;
  icon?: React.ReactNode;
  action?: React.ReactNode;
  className?: string;
  withWorkspace?: boolean;
};

export function EmptyState({
  title = "No items found",
  message = "There are no items to display at this time.",
  icon,
  action,
  className,
  withWorkspace = false,
}: EmptyStateProps) {
  const content = (
    <div
      className={cn(
        "flex-1 flex flex-col items-center justify-center py-16 text-center max-w-md mx-auto",
        className,
      )}
    >
      {icon && (
        <div className="p-4 rounded-2xl bg-surface/60 border border-white/10 text-text-muted mb-4 shadow-lg">
          {icon}
        </div>
      )}
      <h3 className="text-lg font-semibold text-foreground">{title}</h3>
      {message && (
        <p className="text-xs text-text-muted mt-1 max-w-xs">{message}</p>
      )}
      {action && <div className="mt-4">{action}</div>}
    </div>
  );

  if (withWorkspace) {
    return <Workspace>{content}</Workspace>;
  }

  return content;
}
