import type { ReactNode } from "react";
import { Settings } from "lucide-react";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
  CardAction,
} from "@/components/ui/card";
import { Switch } from "@/components/ui/switch";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";

type IntegrationCardProps = {
  title: string;
  category: string;
  description: string;
  icon: ReactNode;
  iconBgClass?: string;
  enabled?: boolean;
  onToggleEnabled?: (enabled: boolean) => void;
  onManage?: () => void;
  metaText?: ReactNode;
  className?: string;
};

export function IntegrationCard({
  title,
  category,
  description,
  icon,
  iconBgClass = "bg-primary/10 text-primary border-primary/20",
  enabled = true,
  onToggleEnabled,
  onManage,
  metaText,
  className,
}: IntegrationCardProps) {
  return (
    <Card
      className={cn(
        "group relative flex flex-col justify-between border-white/10 bg-surface/80 hover:bg-surface transition-all duration-200 shadow-lg hover:shadow-xl hover:border-white/20",
        className,
      )}
    >
      <CardHeader className="pb-3">
        <div className="flex items-start justify-between gap-3">
          <div className="flex items-center gap-3">
            <div
              className={cn(
                "flex items-center justify-center size-11 rounded-2xl border transition-transform duration-200 group-hover:scale-105 shadow-inner shrink-0",
                iconBgClass,
              )}
            >
              {icon}
            </div>
            <div>
              <div className="flex items-center gap-2">
                <CardTitle className="text-base font-semibold text-foreground">
                  {title}
                </CardTitle>
                <span className="text-[10px] font-medium uppercase tracking-wider px-1.5 py-0.5 rounded-md bg-white/5 border border-white/10 text-text-muted">
                  {category}
                </span>
              </div>
            </div>
          </div>

          {onToggleEnabled && (
            <CardAction>
              <Switch
                checked={enabled}
                onCheckedChange={(checked) => onToggleEnabled(checked)}
                aria-label={`Toggle ${title}`}
              />
            </CardAction>
          )}
        </div>
      </CardHeader>

      <CardContent className="py-2">
        <CardDescription className="text-xs text-text-muted leading-relaxed line-clamp-2">
          {description}
        </CardDescription>
      </CardContent>

      <CardFooter className="pt-3 border-t border-white/5 flex items-center justify-between gap-3 text-xs text-text-muted">
        <span className="truncate">{metaText || "\u00A0"}</span>

        {onManage && (
          <Button
            type="button"
            variant="outline"
            size="sm"
            onClick={onManage}
            className="text-xs h-7 px-2.5 border-white/10 gap-1.5 hover:bg-surface-hover hover:text-foreground cursor-pointer shrink-0"
          >
            <Settings className="w-3 h-3" />
            <span>Manage</span>
          </Button>
        )}
      </CardFooter>
    </Card>
  );
}
