import type { ReactNode } from "react";
import { cn } from "@/lib/utils";

type SettingsSectionProps = {
  title: string;
  description?: string;
  count?: number;
  headerAction?: ReactNode;
  children: ReactNode;
  className?: string;
};

export function SettingsSection({
  title,
  description,
  count,
  headerAction,
  children,
  className,
}: SettingsSectionProps) {
  return (
    <section className={cn("space-y-4", className)}>
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-white/10 pb-4">
        <div>
          <h3 className="text-xl font-bold text-foreground tracking-tight flex items-center gap-2.5">
            <span>{title}</span>
            {count !== undefined && (
              <span className="text-xs font-semibold px-2.5 py-0.5 rounded-full bg-surface/80 border border-white/10 text-text-muted">
                {count}
              </span>
            )}
          </h3>
          {description && (
            <p className="text-xs sm:text-sm text-text-muted mt-1">
              {description}
            </p>
          )}
        </div>

        {headerAction}
      </div>

      <div className="space-y-3">{children}</div>
    </section>
  );
}
