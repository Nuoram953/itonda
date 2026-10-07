import type { ReactNode } from "react";
import { Settings as SettingsIcon, AlertCircle } from "lucide-react";
import type { components } from "@/api/generated.d";
import { Badge } from "@/components/ui/badge";
import { Switch } from "@/components/ui/switch";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";

type MediaType = components["schemas"]["MediaType"];

function getMediaTypeLabel(type: MediaType | string): string {
  switch (type) {
    case "game":
      return "Game";
    case "movie":
      return "Movie";
    case "tv_show":
      return "TV Show";
    default:
      return type;
  }
}

type IntegrationRowProps = {
  title: string;
  mediaTypes?: Array<MediaType | string>;
  category?: string;
  description: string;
  icon: ReactNode;
  iconBgClass?: string;
  enabled?: boolean;
  onToggleEnabled?: (enabled: boolean) => void;
  onOpenSheet?: () => void;
  issueText?: ReactNode;
  metaText?: ReactNode;
  badge?: ReactNode;
  className?: string;
};

export function IntegrationRow({
  title,
  mediaTypes,
  description,
  icon,
  iconBgClass = "bg-primary/10 text-primary border-primary/20",
  enabled = true,
  onToggleEnabled,
  onOpenSheet,
  issueText,
  metaText,
  badge,
  className,
}: IntegrationRowProps) {
  return (
    <div
      className={cn(
        "group relative flex flex-col sm:flex-row sm:items-center justify-between gap-4 p-4 sm:p-5 rounded-2xl border border-white/10 bg-surface/60 hover:bg-surface/80 hover:border-white/20 transition-all duration-200 shadow-sm hover:shadow-md",
        className,
      )}
    >
      <div className="flex items-start sm:items-center gap-3.5 sm:gap-4 min-w-0">
        <div
          className={cn(
            "flex items-center justify-center size-11 rounded-2xl border transition-transform duration-200 group-hover:scale-105 shadow-inner shrink-0 mt-0.5 sm:mt-0",
            iconBgClass,
          )}
        >
          {icon}
        </div>

        <div className="min-w-0 space-y-1 sm:space-y-0.5">
          <div className="flex items-center gap-2 flex-wrap">
            <h4 className="text-sm sm:text-base font-semibold text-foreground tracking-tight">
              {title}
            </h4>
            {mediaTypes?.map((type) => (
              <Badge
                key={type}
                variant="subtle"
                className="text-[10px] h-4.5 px-1.5 font-medium tracking-wide rounded-md"
              >
                {getMediaTypeLabel(type)}
              </Badge>
            ))}
            {badge}
          </div>
          <p className="text-xs sm:text-sm text-text-muted leading-relaxed line-clamp-2 sm:line-clamp-1">
            {description}
          </p>
          {issueText && (
            <div className="flex items-center gap-1.5 text-xs text-amber-400 font-medium pt-0.5">
              <AlertCircle className="w-3.5 h-3.5 shrink-0" />
              <span>{issueText}</span>
            </div>
          )}
          {metaText && (
            <div className="text-xs text-text-muted/80 pt-0.5">
              {metaText}
            </div>
          )}
        </div>
      </div>

      <div className="flex items-center justify-end gap-3 sm:gap-4 shrink-0 border-t border-white/5 pt-3 sm:border-0 sm:pt-0">
        {onToggleEnabled && (
          <Switch
            checked={enabled}
            onCheckedChange={(checked) => onToggleEnabled(checked)}
            aria-label={`Toggle ${title}`}
          />
        )}

        {onOpenSheet && (
          <Button
            type="button"
            variant="ghost"
            size="icon-sm"
            onClick={onOpenSheet}
            className="text-text-muted hover:text-foreground hover:bg-white/10 rounded-xl cursor-pointer"
            title={`Configure ${title}`}
            aria-label={`Configure ${title}`}
          >
            <SettingsIcon className="w-4 h-4" />
          </Button>
        )}
      </div>
    </div>
  );
}
