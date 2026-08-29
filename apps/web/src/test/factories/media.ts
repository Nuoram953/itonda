import type { components } from "@/api/generated.d";

export type Media = components["schemas"]["Media"];
export type Asset = components["schemas"]["Asset"];
export type Launch = components["schemas"]["Launch"];
export type MediaInstallation = components["schemas"]["MediaInstallation"];
export type MediaStorefront = components["schemas"]["MediaStorefront"];
export type MediaDetails = components["schemas"]["MediaDetails"];
export type MediaGameDetails = components["schemas"]["MediaGameDetails"];
export type MediaResponse = components["schemas"]["MediaResponse"];

export function createAsset(overrides?: Partial<Asset>): Asset {
  return {
    id: "asset-1",
    asset_type: "poster",
    ...overrides,
  };
}

export function createLaunch(overrides?: Partial<Launch>): Launch {
  return {
    id: "launch-1",
    name: "Default Profile",
    agent_id: null,
    ...overrides,
  };
}

export function createMediaInstallation(
  overrides?: Partial<MediaInstallation>,
): MediaInstallation {
  return {
    id: "installation-1",
    agent_id: "agent-1",
    external_id: null,
    path: null,
    storefront_id: null,
    ...overrides,
  };
}

export function createMediaStorefront(
  overrides?: Partial<MediaStorefront>,
): MediaStorefront {
  return {
    external_id: "steam-1",
    storefront_id: "Steam",
    last_played_at: null,
    playtime_minutes: null,
    ...overrides,
  };
}

export type GameplayPillar = components["schemas"]["GameplayPillar"];

export function createGameplayPillar(
  overrides?: Partial<GameplayPillar>,
): GameplayPillar {
  return {
    id: "pillar-1",
    title: "Realistic Combat",
    description: "Master challenging melee and ranged battles.",
    icon: "combat",
    asset_id: null,
    ...overrides,
  };
}

export function createMediaDetails(
  overrides?: Partial<MediaGameDetails>,
): MediaGameDetails {
  return {
    developers: [],
    publishers: [],
    last_played_at: null,
    playtime_minutes: null,
    series: null,
    pillars: overrides?.pillars ?? [],
    ...overrides,
  };
}



export function createMedia(overrides?: Partial<Media>): Media {
  return {
    id: "media-1",
    title: "Test Media",
    media_type: "game",
    status: "not_started",
    assets: [],
    genres: [],
    tags: [],
    launches: [],
    installations: [],
    storefronts: [],
    details: null,
    description: null,
    summary: null,
    release_date: null,
    ...overrides,
  };
}

export function createMediaResponse(
  overrides?: Partial<MediaResponse>,
): MediaResponse {
  const items = overrides?.items ?? [];
  return {
    items,
    total: items.length,
    page: 1,
    limit: 24,
    total_pages: 1,
    has_next: false,
    ...overrides,
  };
}
