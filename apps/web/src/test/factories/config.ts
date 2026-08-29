import type { components } from "@/api/generated.d";

export type CombinedConfig = components["schemas"]["CombinedConfig"];

export function createCombinedConfig(
  overrides?: Partial<CombinedConfig>,
): CombinedConfig {
  return {
    settings: {
      metadata: {
        steam: {
          enabled: true,
          fetch_achievements: true,
          fetch_playtime: true,
        },
      },
      ...overrides?.settings,
    },
    secrets: {
      storefronts: {
        steam: {
          api_key: "steam-api-key-xyz",
          steam_id: "76561198000000000",
          account_name: null,
          avatar_url: null,
        },
      },
      asset_store: {
        steam_grid_db: {
          api_key: "sgdb-api-key-123",
        },
        tmdb: {
          api_key: "tmdb-api-key-456",
        },
      },
      metadata_store: {
        igdb: {
          client_id: "",
          client_secret: "",
        },
      },
      ...overrides?.secrets,
    },
    app: {
      server: {
        host: "0.0.0.0",
        port: 3005,
      },
      ...overrides?.app,
    },
    ...overrides,
  };
}
