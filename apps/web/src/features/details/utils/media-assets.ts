import type { components } from "@/api/generated.d";

type MediaAsset = components["schemas"]["Asset"];
type Media = components["schemas"]["Media"];

/**
 * Filter assets by a specific asset type.
 */
export function getAssetsByType(assets: MediaAsset[] = [], type: string): MediaAsset[] {
  return assets.filter((asset) => asset.asset_type === type);
}

/**
 * Find the first asset matching one of the specified asset types.
 */
export function findAssetByType(
  assets: MediaAsset[] = [],
  types: string | string[]
): MediaAsset | undefined {
  const typeList = Array.isArray(types) ? types : [types];
  return assets.find((asset) => typeList.includes(asset.asset_type));
}

/**
 * Get all trailer assets for a media item.
 */
export function getTrailerAssets(assets: MediaAsset[] = []): MediaAsset[] {
  return getAssetsByType(assets, "trailer");
}

/**
 * Get screenshot/visual assets (screenshots, backdrops, or banners).
 */
export function getScreenshotAssets(assets: MediaAsset[] = []): MediaAsset[] {
  return assets.filter(
    (a) =>
      a.asset_type === "screenshot" ||
      a.asset_type === "backdrop" ||
      a.asset_type === "banner"
  );
}

/**
 * Get the hero banner/backdrop asset.
 */
export function getHeroBannerAsset(assets: MediaAsset[] = []): MediaAsset | undefined {
  return findAssetByType(assets, ["banner", "backdrop"]);
}

/**
 * Get the hero poster asset.
 */
export function getHeroPosterAsset(assets: MediaAsset[] = []): MediaAsset | undefined {
  return findAssetByType(assets, "poster");
}

/**
 * Get the main trailer asset.
 */
export function getHeroTrailerAsset(assets: MediaAsset[] = []): MediaAsset | undefined {
  return findAssetByType(assets, "trailer");
}

/**
 * Determine whether user has played the media based on playtime, last played date, or status.
 */
export function hasMediaBeenPlayed(media: Media): boolean {
  const playtimeMinutes = media.details?.playtime_minutes ?? 0;
  const lastPlayedAt = media.details?.last_played_at ?? null;
  return (
    playtimeMinutes > 0 ||
    lastPlayedAt !== null ||
    (Boolean(media.status) && media.status !== "not_started")
  );
}

export { formatPlaytimeHours, formatLastPlayedDate } from "@/utils/datetime";

