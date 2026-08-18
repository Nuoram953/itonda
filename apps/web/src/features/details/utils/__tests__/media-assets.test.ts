import { describe, it, expect } from "vitest";
import type { components } from "@/api/generated.d";
import {
  getAssetsByType,
  findAssetByType,
  getTrailerAssets,
  getScreenshotAssets,
  getHeroBannerAsset,
  getHeroPosterAsset,
  getHeroTrailerAsset,
  hasMediaBeenPlayed,
  formatPlaytimeHours,
  formatLastPlayedDate,
} from "../media-assets";

type MediaAsset = components["schemas"]["Asset"];
type Media = components["schemas"]["Media"];

describe("media-assets utilities", () => {
  const mockAssets: MediaAsset[] = [
    { id: "asset-1", asset_type: "poster" },
    { id: "asset-2", asset_type: "banner" },
    { id: "asset-3", asset_type: "trailer" },
    { id: "asset-4", asset_type: "screenshot" },
    { id: "asset-5", asset_type: "backdrop" },
  ];

  it("filters assets by type correctly", () => {
    const posters = getAssetsByType(mockAssets, "poster");
    expect(posters).toHaveLength(1);
    expect(posters[0].id).toBe("asset-1");
  });

  it("finds first asset matching single or array of types", () => {
    expect(findAssetByType(mockAssets, "trailer")?.id).toBe("asset-3");
    expect(findAssetByType(mockAssets, ["banner", "backdrop"])?.id).toBe(
      "asset-2",
    );
    expect(findAssetByType(mockAssets, "nonexistent")).toBeUndefined();
  });

  it("extracts trailer and screenshot assets", () => {
    expect(getTrailerAssets(mockAssets)).toHaveLength(1);
    expect(getScreenshotAssets(mockAssets)).toHaveLength(3); // banner, screenshot, backdrop
  });

  it("extracts hero assets (banner, poster, trailer)", () => {
    expect(getHeroPosterAsset(mockAssets)?.id).toBe("asset-1");
    expect(getHeroBannerAsset(mockAssets)?.id).toBe("asset-2");
    expect(getHeroTrailerAsset(mockAssets)?.id).toBe("asset-3");
  });

  it("evaluates hasMediaBeenPlayed correctly", () => {
    const baseMedia: Media = {
      id: "m1",
      title: "Test Game",
      media_type: "game",
      status: "not_started",
      assets: [],
      launches: [],
      storefronts: [],
      installations: [],
    };

    expect(hasMediaBeenPlayed(baseMedia)).toBe(false);

    expect(
      hasMediaBeenPlayed({
        ...baseMedia,
        details: { playtime_minutes: 120 },
      }),
    ).toBe(true);

    expect(
      hasMediaBeenPlayed({
        ...baseMedia,
        details: { last_played_at: 1600000000 },
      }),
    ).toBe(true);

    expect(
      hasMediaBeenPlayed({
        ...baseMedia,
        status: "in_progress",
      }),
    ).toBe(true);
  });

  it("formats playtime hours correctly", () => {
    expect(formatPlaytimeHours(0)).toBe(0);
    expect(formatPlaytimeHours(125)).toBe(2);
    expect(formatPlaytimeHours(null)).toBe(0);
  });

  it("formats last played date correctly", () => {
    expect(formatLastPlayedDate(null)).toBe("Never played");
    expect(formatLastPlayedDate(undefined)).toBe("Never played");
    expect(formatLastPlayedDate(1700000000)).toContain("2023");
  });
});
