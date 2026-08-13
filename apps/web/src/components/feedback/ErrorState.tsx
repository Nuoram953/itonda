import * as React from "react";
import { Workspace } from "@/components/workspace/Workspace";
import { cn } from "@/lib/utils";

export type ErrorStateProps = {
  title?: string;
  message?: string;
  icon?: React.ReactNode;
  action?: React.ReactNode;
  className?: string;
  withWorkspace?: boolean;
};

export function ErrorState({
  title = "Something went wrong",
  message = "The requested item could not be loaded or may have been deleted.",
  icon,
  action,
  className,
  withWorkspace = true,
}: ErrorStateProps) {
  const content = (
    <div
      className={cn(
        "flex-1 flex flex-col items-center justify-center p-12 text-center",
        className,
      )}
    >
      {icon && <div className="mb-3 text-text-muted">{icon}</div>}
      <p className="text-lg font-semibold text-foreground">{title}</p>
      {message && (
        <p className="text-sm text-text-muted mt-1 max-w-md">{message}</p>
      )}
      {action && <div className="mt-4">{action}</div>}
    </div>
  );

  if (withWorkspace) {
    return <Workspace>{content}</Workspace>;
  }

  return content;
}
