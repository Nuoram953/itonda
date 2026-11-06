import { useState } from "react";
import type { components } from "@/api/generated.d";
import { getAssetUrl } from "../../utils/asset-url";
import {
  getHeroBannerAsset,
  getHeroPosterAsset,
} from "../../utils/media-assets";
import { formatLastPlayedDate, formatPlaytime } from "@/utils/datetime";
import { cn } from "@/lib/utils";

type HeroHeaderProps = {
  media: components["schemas"]["Media"];
};

//TODO: Should come from the api
const DEFAULT_TAGS = [
  "Souls-like",
  "Dark Fantasy",
  "Singleplayer",
  "Masterpiece",
];

export function HeroHeader({ media }: HeroHeaderProps) {
  const bannerAsset = getHeroBannerAsset(media.assets);
  const posterAsset = getHeroPosterAsset(media.assets);
  const [imageLoaded, setImageLoaded] = useState(false);

  const playtimeHours = formatPlaytime(media.details?.playtime_minutes, {
    mode: "compact",
  });
  const lastPlayedDate = formatLastPlayedDate(media.details?.last_played_at);

  return (
    <div className="relative w-full min-h-120 md:min-h-136 overflow-hidden bg-background select-none flex flex-col justify-end">
      <div className="absolute inset-0 w-full h-full">
        {bannerAsset ? (
          <img
            src={getAssetUrl(bannerAsset.id)}
            alt={media.title}
            onLoad={() => setImageLoaded(true)}
            className={cn(
              "absolute inset-0 w-full h-full object-cover transition-opacity duration-700",
              imageLoaded ? "opacity-100" : "opacity-0",
            )}
          />
        ) : (
          <div className="w-full h-full bg-linear-to-br from-slate-900 via-surface to-slate-950" />
        )}
      </div>

      <div className="absolute inset-x-0 top-0 h-20 bg-linear-to-b from-black/50 to-transparent pointer-events-none" />
      <div className="absolute inset-x-0 bottom-0 h-48 bg-linear-to-t from-background via-background/60 to-transparent pointer-events-none" />
      <div className="absolute inset-0 bg-linear-to-r from-background/80 via-background/40 to-transparent pointer-events-none md:w-3/4" />
      <div className="relative z-10 w-full max-w-7xl mx-auto px-6 flex flex-col justify-end pt-20 pb-8">
        <div className="flex flex-col md:flex-row md:items-start gap-6">
          <div className="shrink-0 w-36 h-48 md:w-44 md:h-66 rounded-xl overflow-hidden shadow-2xl border border-white/15 bg-surface transition-transform duration-300 hover:scale-105">
            {posterAsset ? (
              <img
                src={getAssetUrl(posterAsset.id)}
                alt={media.title}
                className="w-full h-full object-cover"
              />
            ) : (
              <div className="w-full h-full bg-linear-to-br from-slate-800 to-slate-950 flex flex-col items-center justify-center p-4 text-center">
                <span className="text-xs font-bold text-text-muted">
                  {media.title}
                </span>
              </div>
            )}
          </div>

          <div className="flex-1 flex flex-col justify-between md:min-h-66">
            <div className="space-y-2.5">
              <h1 className="text-4xl font-extrabold tracking-tight text-foreground drop-shadow-md leading-tight">
                {media.title}
              </h1>

              <div className="flex flex-wrap items-center gap-2 text-xs sm:text-sm font-medium text-text-muted">
                <span>
                  Action RPG · Open World · Medieval · FromSoftware · 2022
                </span>
              </div>

              <div className="flex flex-wrap items-center gap-1.5 pt-0.5">
                {DEFAULT_TAGS.map((tag) => (
                  <span
                    key={tag}
                    className="px-2.5 py-0.5 rounded-md text-xs font-medium bg-surface-hover border border-white/10 text-text-muted"
                  >
                    #{tag}
                  </span>
                ))}
              </div>

              <div className="flex flex-wrap items-end justify-between gap-4 pt-1">
                {media.media_type == "game" && (
                  <div className="flex flex-wrap items-center gap-6 text-xs">
                    <div>
                      <span className="block text-xs font-semibold uppercase tracking-wider text-text-muted">
                        Playtime
                      </span>
                      <span className="text-md font-bold text-foreground">
                        {playtimeHours}
                      </span>
                    </div>

                    <div className="w-px h-6 bg-border/60" />

                    <div>
                      <span className="block text-xs font-semibold uppercase tracking-wider text-text-muted">
                        Last Played
                      </span>
                      <span className="text-md font-semibold text-foreground">
                        {lastPlayedDate}
                      </span>
                    </div>

                    <div className="w-px h-6 bg-border/60" />
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
