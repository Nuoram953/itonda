export const ASSET_BASE_URL = "http://localhost:3005/api/v1/assets";

export function getAssetUrl(assetId?: string | null): string {
  if (!assetId) return "";
  return `${ASSET_BASE_URL}/${assetId}`;
}
