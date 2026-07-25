import { createContext, type ReactNode, useContext } from "react";
import { cn } from "@/lib/utils";
import type { IconType } from "react-icons";

const WorkspaceContext = createContext({});

export function Workspace({ children }: { children: ReactNode }) {
  return (
    <WorkspaceContext.Provider value={{}}>
      <section className="flex h-full flex-col overflow-hidden">
        {children}
      </section>
    </WorkspaceContext.Provider>
  );
}

type HeaderProps = {
  title: ReactNode;
  subtitle?: ReactNode;
  children?: ReactNode;
  className?: string;
};

function Header({ title, subtitle, children, className }: HeaderProps) {
  useContext(WorkspaceContext);

  return (
    <header
      className={cn(
        "flex items-center justify-between border-b border-border-strong px-6 py-4",
        className,
      )}
    >
      <div>
        <h1 className="text-2xl font-semibold">{title}</h1>

        {subtitle && <p className="mt-1 text-sm text-text-muted">{subtitle}</p>}
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
    <div className={cn("flex items-center gap-2", className)}>{children}</div>
  );
}

type ActionProps = {
  icon: IconType;
  children: ReactNode;
  active?: boolean;
  className?: string;
} & React.ButtonHTMLAttributes<HTMLButtonElement>;

function Action({
  icon: Icon,
  children,
  active,
  className,
  ...props
}: ActionProps) {
  useContext(WorkspaceContext);

  return (
    <button
      data-active={active}
      className={cn(
        "flex w-16 flex-col items-center gap-2 rounded-md p-2",
        "text-text-muted transition-colors",
        "hover:bg-surface-hover hover:text-primary-hover",
        "data-[active=true]:text-primary-active",
        className,
      )}
      {...props}
    >
      <Icon className="text-lg" />
      <span className="text-xs">{children}</span>
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
