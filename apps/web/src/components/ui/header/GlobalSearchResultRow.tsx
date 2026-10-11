import { useState } from "react";
import { Film, Gamepad2, Play, Tv, Sparkles, Eye } from "lucide-react";
import type { components } from "@/api/generated.d";
import { CommandItem } from "@/components/ui/command";
import { getHeroPosterAsset } from "@/features/details/utils/media-assets";
import { getAssetUrl } from "@/features/details/utils/asset-url";
import { cn } from "@/lib/utils";

export type MediaItem = components["schemas"]["Media"];
export type SearchAction = "start" | "detail";

const MEDIA_ICONS: Record<string, typeof Gamepad2> = {
  game: Gamepad2,
  movie: Film,
  tv_show: Tv,
};

const STATUS_LABELS: Record<string, string> = {
  not_started: "Unplayed",
  in_progress: "In Progress",
  completed: "Completed",
  paused: "Paused",
  abandoned: "Dropped",
};

import type { useMediaLauncher } from "@/hooks/use-media-launcher";

type GlobalSearchResultRowProps = {
  item: MediaItem;
  isSelected: boolean;
  activeAction: SearchAction;
  launcher: ReturnType<typeof useMediaLauncher>;
  onSelect: (action: SearchAction) => void;
  onHover: (action?: SearchAction) => void;
};

export function GlobalSearchResultRow({
  item,
  isSelected,
  activeAction,
  launcher,
  onSelect,
  onHover,
}: GlobalSearchResultRowProps) {
  const [imageError, setImageError] = useState(false);
  const posterAsset = getHeroPosterAsset(item.assets);
  const posterUrl = posterAsset ? getAssetUrl(posterAsset.id) : "";
  const Icon = MEDIA_ICONS[item.media_type] ?? Sparkles;
  const isGame = item.media_type === "game";

  const canLaunch = launcher.canLaunchMedia(item);
  const disabledReason = launcher.getDisabledReason(item);
  const isCurrentMediaPlaying = launcher.isMediaPlaying(item.id);

  const btnClass = (isActive: boolean) =>
    cn(
      "inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-xs font-semibold transition-all cursor-pointer",
      isActive
        ? isCurrentMediaPlaying && activeAction === "start"
          ? "bg-emerald-500 text-black shadow-md ring-2 ring-emerald-400/50"
          : "bg-primary text-primary-foreground shadow-md ring-2 ring-primary/40"
        : "bg-white/5 hover:bg-white/10 text-text-muted hover:text-foreground border border-white/10",
    );

  return (
    <CommandItem
      value={item.id}
      onSelect={() => onSelect(activeAction)}
      onMouseEnter={() => onHover()}
      className={cn(
        "flex items-center justify-between gap-3 p-2 rounded-xl border transition-all duration-150 cursor-pointer select-none",
        isSelected
          ? "border-primary/40 bg-surface-raised shadow-sm"
          : "border-white/5 bg-surface/30 hover:bg-surface-hover/50 hover:border-white/10",
      )}
    >
      <div className="flex items-center gap-3 min-w-0 flex-1">
        <div className="relative w-9 h-12 rounded-lg overflow-hidden bg-surface-raised shrink-0 border border-white/10 flex items-center justify-center">
          {posterUrl && !imageError ? (
            <img
              src={posterUrl}
              alt={item.title}
              onError={() => setImageError(true)}
              className="w-full h-full object-cover"
              loading="lazy"
            />
          ) : (
            <Icon className="w-3 h-3 text-primary" />
          )}
        </div>

        <div className="min-w-0 flex-1">
          <span className="text-xs font-semibold text-foreground truncate block">
            {item.title}
          </span>
          <div className="flex items-center gap-2 mt-1">
            <span className="inline-flex items-center gap-1 text-[10px] text-text-muted font-medium capitalize">
              <Icon className="w-3 h-3 text-primary" />
              <span>{item.media_type === "tv_show" ? "TV Series" : item.media_type}</span>
            </span>
            <span className="text-white/20 text-[10px]">•</span>
            <span className="text-[10px] text-text-muted font-medium">
              {STATUS_LABELS[item.status] ?? item.status}
            </span>
          </div>
        </div>
      </div>

      <div className="flex items-center gap-1.5 shrink-0">
        {isGame && (
          <button
            type="button"
            disabled={!canLaunch}
            title={disabledReason}
            onMouseEnter={(e) => {
              e.stopPropagation();
              onHover("start");
            }}
            onClick={(e) => {
              e.stopPropagation();
              if (canLaunch) onSelect("start");
            }}
            className={cn(
              btnClass(isSelected && activeAction === "start"),
              !canLaunch && "opacity-40 cursor-not-allowed",
            )}
          >
            {isCurrentMediaPlaying ? (
              <>
                <span className="w-2 h-2 rounded-full bg-black animate-pulse shrink-0" />
                <span>Playing</span>
              </>
            ) : (
              <>
                <Play className="w-3 h-3 fill-current shrink-0" />
                <span>Start Game</span>
              </>
            )}
          </button>
        )}

        <button
          type="button"
          onMouseEnter={(e) => {
            e.stopPropagation();
            onHover("detail");
          }}
          onClick={(e) => {
            e.stopPropagation();
            onSelect("detail");
          }}
          className={btnClass(isSelected && activeAction === "detail")}
        >
          <Eye className="w-3 h-3 shrink-0" />
          <span>Details</span>
        </button>
      </div>
    </CommandItem>
  );
}
