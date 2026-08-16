import { useState } from "react";
import { Link } from "@tanstack/react-router";
import { Gamepad2 } from "lucide-react";
import { useActiveMedia } from "@/hooks/use-active-media";
import { getHeroPosterAsset } from "@/features/details/utils/media-assets";
import { getAssetUrl } from "@/features/details/utils/asset-url";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { cn } from "@/lib/utils";

export interface NowPlayingProps {
  className?: string;
}

function NowPlayingBeacon({ pulse = true }: { pulse?: boolean }) {
  return (
    <span className="relative flex h-2 w-2 shrink-0">
      {pulse && (
        <span className="absolute inline-flex h-full w-full animate-ping rounded-full bg-emerald-400 opacity-75" />
      )}
      <span className="relative inline-flex h-2 w-2 rounded-full bg-emerald-500" />
    </span>
  );
}

function NowPlayingCover({
  posterUrl,
  title,
  size = "md",
}: {
  posterUrl?: string;
  title: string;
  size?: "md" | "sm";
}) {
  const [imageError, setImageError] = useState(false);
  const showImage = Boolean(posterUrl && !imageError);

  if (size === "sm") {
    return showImage ? (
      <img
        src={posterUrl}
        alt={title}
        onError={() => setImageError(true)}
        className="h-full w-full rounded-md object-cover"
      />
    ) : (
      <Gamepad2 className="h-4 w-4 text-text-muted" />
    );
  }

  return (
    <div className="relative h-11 w-11 shrink-0 overflow-hidden rounded-lg border border-white/10 bg-surface-raised flex items-center justify-center">
      {showImage ? (
        <img
          src={posterUrl}
          alt={title}
          onError={() => setImageError(true)}
          className="h-full w-full object-cover"
          loading="lazy"
        />
      ) : (
        <Gamepad2 className="h-5 w-5 text-text-muted" />
      )}
    </div>
  );
}

function NowPlayingTimer({ formattedElapsed }: { formattedElapsed: string }) {
  return (
    <div className="flex items-center gap-1.5 text-[11px] font-mono text-text-muted">
      <Gamepad2 className="h-3 w-3 text-text-muted/80 shrink-0" />
      <span>{formattedElapsed}</span>
    </div>
  );
}

export function NowPlaying({ className }: NowPlayingProps) {
  const { session, media, isPlaying, formattedElapsed } = useActiveMedia();

  if (!isPlaying || !session || !media) {
    return null;
  }

  const posterAsset = media?.assets
    ? getHeroPosterAsset(media.assets)
    : undefined;
  const posterUrl = posterAsset ? getAssetUrl(posterAsset.id) : "";
  const title = media.title;

  return (
    <div className={cn("w-full select-none", className)}>
      <Link
        to="/media/$mediaId"
        params={{ mediaId: session.mediaId }}
        className={cn(
          "group flex items-center gap-2.5 rounded-xl  bg-surface/80 p-2",
          "hover:border-white/20 hover:bg-surface-hover transition-all shadow-xs",
          "group-data-[collapsible=icon]:hidden",
        )}
        title={`Now Playing: ${title}`}
      >
        <NowPlayingCover posterUrl={posterUrl} title={title} size="md" />

        <div className="min-w-0 flex-1 space-y-0.5">
          <p className="truncate text-xs font-semibold text-foreground group-hover:text-primary transition-colors">
            {title}
          </p>

          <NowPlayingTimer formattedElapsed={formattedElapsed} />
        </div>
      </Link>

      <div className="hidden group-data-[collapsible=icon]:flex group-data-[collapsible=icon]:justify-center">
        <Tooltip>
          <TooltipTrigger
            render={
              <Link
                to="/media/$mediaId"
                params={{ mediaId: session.mediaId }}
                className={cn(
                  "relative flex h-8 w-8 items-center justify-center rounded-lg border border-white/10 bg-surface/80",
                  "hover:border-white/20 hover:bg-surface-hover transition-all",
                )}
                aria-label={`Now Playing: ${title}`}
              />
            }
          >
            <NowPlayingCover posterUrl={posterUrl} title={title} size="sm" />
            <span className="absolute -top-1 -right-1 flex h-2 w-2">
              <NowPlayingBeacon />
            </span>
          </TooltipTrigger>

          <TooltipContent
            side="right"
            align="center"
            className="flex flex-col gap-1 p-2.5 bg-surface/95 border border-white/10 shadow-xl rounded-lg text-xs"
          >
            <div className="flex items-center gap-1.5 text-[10px] font-bold uppercase tracking-wider text-text-muted font-mono">
              <span className="h-1.5 w-1.5 rounded-full bg-emerald-400 animate-pulse" />
              Now Playing
            </div>
            <span className="font-semibold text-foreground max-w-48 truncate">
              {title}
            </span>
            <NowPlayingTimer formattedElapsed={formattedElapsed} />
          </TooltipContent>
        </Tooltip>
      </div>
    </div>
  );
}
