import type { ReactNode } from "react";
import { Label } from "@/components/ui/label";
import { cn } from "@/lib/utils";

type SettingRowProps = {
  label: ReactNode;
  description?: ReactNode;
  badge?: ReactNode;
  children: ReactNode;
  htmlFor?: string;
  className?: string;
  layout?: "horizontal" | "vertical";
};

export function SettingRow({
  label,
  description,
  badge,
  children,
  htmlFor,
  className,
  layout = "horizontal",
}: SettingRowProps) {
  const isVertical = layout === "vertical";

  return (
    <div
      className={cn(
        isVertical
          ? "flex flex-col gap-2.5 py-3.5 first:pt-0 last:pb-0"
          : "flex flex-col sm:flex-row sm:items-center justify-between gap-4 py-3.5 first:pt-0 last:pb-0",
        className,
      )}
    >
      <div className={cn("space-y-1", !isVertical && "max-w-lg")}>
        <div className="flex items-center gap-2">
          {typeof label === "string" ? (
            <Label
              htmlFor={htmlFor}
              className="text-sm font-medium text-foreground cursor-pointer"
            >
              {label}
            </Label>
          ) : (
            label
          )}
          {badge}
        </div>
        {description && (
          <p className="text-xs text-text-muted leading-relaxed">
            {description}
          </p>
        )}
      </div>

      <div
        className={cn(
          isVertical ? "w-full" : "flex items-center gap-3 shrink-0",
        )}
      >
        {children}
      </div>
    </div>
  );
}
