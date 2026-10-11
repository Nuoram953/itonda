import { useState } from "react";
import { Film, Gamepad2, Play, Tv, Sparkles, Eye } from "lucide-react";
import type { components } from "@/api/generated.d";
import { CommandItem } from "@/components/ui/command";
import { getHeroPosterAsset } from "@/features/details/utils/media-assets";
import { getAssetUrl } from "@/features/details/utils/asset-url";
import { cn } from "@/lib/utils";

export type MediaItem = components["schemas"]["Media"];
export type SearchAction = "start" | "detail";

type GlobalSearchResultRowProps = {
  item: MediaItem;
  isSelected: boolean;
  activeAction: SearchAction;
  onSelect: (action: SearchAction) => void;
  onHover: () => void;
  onHoverAction: (action: SearchAction) => void;
  canLaunch: boolean;
  disabledReason?: string;
  isCurrentMediaPlaying: boolean;
};

export function GlobalSearchResultRow({
  item,
  isSelected,
  activeAction,
  onSelect,
  onHover,
  onHoverAction,
  canLaunch,
  disabledReason,
  isCurrentMediaPlaying,
}: GlobalSearchResultRowProps) {
  const [imageError, setImageError] = useState(false);
  const posterAsset = getHeroPosterAsset(item.assets);
  const posterUrl = posterAsset ? getAssetUrl(posterAsset.id) : "";
  const isGame = item.media_type === "game";

  function renderMediaTypeIcon() {
    switch (item.media_type) {
      case "game":
        return <Gamepad2 className="w-3 h-3 text-primary" />;
      case "movie":
        return <Film className="w-3 h-3 text-primary" />;
      case "tv_show":
        return <Tv className="w-3 h-3 text-primary" />;
      default:
        return <Sparkles className="w-3 h-3 text-primary" />;
    }
  }

  function getStatusLabel(status: components["schemas"]["Media"]["status"]) {
    switch (status) {
      case "not_started":
        return "Unplayed";
      case "in_progress":
        return "In Progress";
      case "completed":
        return "Completed";
      case "paused":
        return "Paused";
      case "abandoned":
        return "Dropped";
      default:
        return status;
    }
  }

  return (
    <CommandItem
      value={item.id}
      onSelect={() => onSelect(activeAction)}
      onMouseEnter={onHover}
      className={cn(
        "flex items-center justify-between gap-3 p-2 rounded-xl border transition-all duration-150 cursor-pointer select-none",
        isSelected
          ? "border-primary/40 bg-surface-raised shadow-sm"
          : "border-white/5 bg-surface/30 hover:bg-surface-hover/50 hover:border-white/10",
      )}
    >
      {/* Media Poster & Info */}
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
            <div className="text-text-muted">{renderMediaTypeIcon()}</div>
          )}
        </div>

        <div className="min-w-0 flex-1">
          <div className="flex items-center gap-2">
            <span className="text-xs font-semibold text-foreground truncate">
              {item.title}
            </span>
          </div>

          <div className="flex items-center gap-2 mt-1">
            <span className="inline-flex items-center gap-1 text-[10px] text-text-muted font-medium capitalize">
              {renderMediaTypeIcon()}
              <span>
                {item.media_type === "tv_show" ? "TV Series" : item.media_type}
              </span>
            </span>

            <span className="text-white/20 text-[10px]">•</span>

            <span className="text-[10px] text-text-muted font-medium">
              {getStatusLabel(item.status)}
            </span>
          </div>
        </div>
      </div>

      {/* Action buttons */}
      <div className="flex items-center gap-1.5 shrink-0">
        {isGame && (
          <button
            type="button"
            disabled={!canLaunch}
            title={disabledReason}
            onMouseEnter={(e) => {
              e.stopPropagation();
              onHoverAction("start");
            }}
            onClick={(e) => {
              e.stopPropagation();
              if (canLaunch) onSelect("start");
            }}
            className={cn(
              "inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-xs font-semibold transition-all cursor-pointer",
              isSelected && activeAction === "start"
                ? isCurrentMediaPlaying
                  ? "bg-emerald-500 text-black shadow-md ring-2 ring-emerald-400/50"
                  : "bg-primary text-primary-foreground shadow-md ring-2 ring-primary/40"
                : "bg-white/5 hover:bg-white/10 text-text-muted hover:text-foreground border border-white/10",
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
            onHoverAction("detail");
          }}
          onClick={(e) => {
            e.stopPropagation();
            onSelect("detail");
          }}
          className={cn(
            "inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-xs font-semibold transition-all cursor-pointer",
            isSelected && activeAction === "detail"
              ? "bg-primary text-primary-foreground shadow-md ring-2 ring-primary/40"
              : "bg-white/5 hover:bg-white/10 text-text-muted hover:text-foreground border border-white/10",
          )}
        >
          <Eye className="w-3 h-3 shrink-0" />
          <span>Details</span>
        </button>
      </div>
    </CommandItem>
  );
}
