import { ImageOff } from "lucide-react";
import type { components } from "@/api/generated.d";
import { getAssetUrl } from "../../utils/asset-url";
import {
  findAssetByType,
  getHeroTrailerAsset,
} from "../../utils/media-assets";

type FeaturedMediaProps = {
  media: components["schemas"]["Media"];
};

export function FeaturedMedia({ media }: FeaturedMediaProps) {
  const trailer = getHeroTrailerAsset(media.assets);
  // Prefer a real gameplay screenshot; fall back to backdrop/banner art.
  const still =
    findAssetByType(media.assets, "screenshot") ??
    findAssetByType(media.assets, ["backdrop", "banner"]);

  if (!trailer && !still) {
    return (
      <div className="rounded-2xl border border-dashed border-white/15 bg-surface/30 p-10 text-center flex flex-col items-center justify-center space-y-4">
        <div className="p-4 rounded-2xl bg-surface-raised/80 border border-white/10 text-text-muted">
          <ImageOff className="w-8 h-8" />
        </div>
        <div className="space-y-1 max-w-sm">
          <h4 className="text-base font-bold text-foreground">
            No trailer or screenshots yet
          </h4>
          <p className="text-xs sm:text-sm text-text-muted">
            Refresh this game's metadata to pull in videos and screenshots.
          </p>
        </div>
      </div>
    );
  }

  return (
    <div
      data-testid="featured-media"
      className="relative aspect-video w-full rounded-2xl overflow-hidden border border-white/10 bg-black shadow-xl"
    >
      {trailer ? (
        <video
          key={trailer.id}
          src={getAssetUrl(trailer.id)}
          poster={still ? getAssetUrl(still.id) : undefined}
          controls
          playsInline
          preload="metadata"
          className="w-full h-full object-contain"
        />
      ) : (
        still && (
          <img
            src={getAssetUrl(still.id)}
            alt={media.title}
            className="w-full h-full object-cover"
          />
        )
      )}
    </div>
  );
}
