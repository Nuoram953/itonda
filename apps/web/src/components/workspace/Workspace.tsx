import { createContext, type ReactNode, useContext } from "react";
import { cn } from "@/lib/utils";
import { ArrowLeft, type LucideIcon } from "lucide-react";

const WorkspaceContext = createContext({});

export function Workspace({ children }: { children: ReactNode }) {
  return (
    <WorkspaceContext.Provider value={{}}>
      <section className="flex h-full flex-col overflow-hidden bg-background text-foreground">
        {children}
      </section>
    </WorkspaceContext.Provider>
  );
}

type HeaderProps = {
  title: ReactNode;
  subtitle?: ReactNode;
  showBackBtn?: boolean;
  children?: ReactNode;
  className?: string;
};

function Header({
  title,
  subtitle,
  showBackBtn,
  children,
  className,
}: HeaderProps) {
  useContext(WorkspaceContext);

  return (
    <header
      className={cn(
        "flex items-center justify-between border-b border-white/10 bg-surface/50 px-6 py-4 z-10 shrink-0",
        className,
      )}
    >
      <div className="flex items-center gap-3 min-w-0">
        {showBackBtn && (
          <button
            type="button"
            onClick={() => {
              history.back();
            }}
            className={cn(
              "flex items-center justify-center p-2 rounded-xl border border-white/10 bg-surface/70 text-text-muted transition-colors cursor-pointer",
              "hover:bg-surface-hover hover:text-foreground hover:border-white/20",
              "disabled:cursor-not-allowed disabled:opacity-50 disabled:hover:bg-transparent disabled:hover:text-text-muted",
            )}
            title="Go back"
            aria-label="Back"
          >
            <ArrowLeft className="w-4 h-4" />
          </button>
        )}

        <div className="flex items-center gap-3 flex-wrap">
          {typeof title === "string" ? (
            <h1 className="text-xl font-bold tracking-tight text-foreground">
              {title}
            </h1>
          ) : (
            title
          )}

          {subtitle && (
            <span className="inline-flex items-center rounded-full bg-surface-raised/80 border border-white/10 px-2.5 py-0.5 text-xs font-semibold text-text-muted">
              {subtitle}
            </span>
          )}
        </div>
      </div>

      {children}
    </header>
  );
}

type ActionsProps = {
  children: ReactNode;
  className?: string;
};

function Actions({ children, className }: ActionsProps) {
  useContext(WorkspaceContext);

  return (
    <div className={cn("flex items-center gap-2.5", className)}>{children}</div>
  );
}

type ActionProps = {
  icon: LucideIcon;
  children: ReactNode;
  active?: boolean;
  className?: string;
} & React.ButtonHTMLAttributes<HTMLButtonElement>;

function Action({
  icon: Icon,
  children,
  className,
  active,
  ...props
}: ActionProps) {
  useContext(WorkspaceContext);

  return (
    <button
      className={cn(
        "flex items-center gap-2 px-3 py-1.5 rounded-xl bg-surface/70 border border-white/10 text-xs font-medium text-text-muted shadow-sm transition-all duration-200 cursor-pointer",
        "hover:bg-surface-hover hover:text-foreground hover:border-white/20",
        "disabled:cursor-not-allowed disabled:opacity-50 disabled:hover:bg-transparent disabled:hover:text-text-muted",
        active && "bg-primary/10 text-primary border-primary/40 font-semibold",
        className,
      )}
      {...props}
    >
      <Icon className="w-4 h-4 shrink-0" />
      <span className="truncate">{children}</span>
    </button>
  );
}

type ContentProps = {
  children: ReactNode;
  className?: string;
};

function Content({ children, className }: ContentProps) {
  useContext(WorkspaceContext);

  return (
    <main className={cn("flex-1 overflow-auto p-6", className)}>
      {children}
    </main>
  );
}

Workspace.Header = Header;
Workspace.Actions = Actions;
Workspace.Action = Action;
Workspace.Content = Content;
