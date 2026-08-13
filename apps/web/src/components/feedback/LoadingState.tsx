import { Workspace } from "@/components/workspace/Workspace";
import { cn } from "@/lib/utils";

export type LoadingStateProps = {
  message?: string;
  className?: string;
  withWorkspace?: boolean;
};

export function LoadingState({
  message = "Loading...",
  className,
  withWorkspace = true,
}: LoadingStateProps) {
  const content = (
    <div
      className={cn("flex-1 flex items-center justify-center p-12", className)}
    >
      <div className="flex items-center gap-3 text-text-muted">
        <div className="w-5 h-5 border-2 border-primary border-t-transparent rounded-full animate-spin" />
        <span className="text-sm font-medium">{message}</span>
      </div>
    </div>
  );

  if (withWorkspace) {
    return <Workspace>{content}</Workspace>;
  }

  return content;
}
