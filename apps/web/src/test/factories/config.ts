import type { components } from "@/api/generated.d";

export type CombinedConfig = components["schemas"]["CombinedConfig"];

export type DeepPartial<T> = T extends (...args: unknown[]) => unknown
  ? T
  : T extends Array<infer U>
    ? _DeepPartialArray<U>
    : T extends object
      ? _DeepPartialObject<T>
      : T;

type _DeepPartialArray<T> = Array<DeepPartial<T>>;
type _DeepPartialObject<T> = { [P in keyof T]?: DeepPartial<T[P]> };

function isPlainObject(item: unknown): item is Record<string, unknown> {
  return typeof item === "object" && item !== null && !Array.isArray(item);
}

export function deepMerge<T extends Record<string, unknown>>(
  target: T,
  source?: Record<string, unknown>,
): T {
  if (!source) return target;
  const output = { ...target };

  for (const key of Object.keys(source)) {
    const sourceValue = source[key];
    const targetValue = output[key];

    if (sourceValue === undefined) {
      continue;
    }

    if (isPlainObject(targetValue) && isPlainObject(sourceValue)) {
      output[key as keyof T] = deepMerge(
        targetValue,
        sourceValue,
      ) as unknown as T[keyof T];
    } else {
      output[key as keyof T] = sourceValue as unknown as T[keyof T];
    }
  }

  return output;
}

export function createCombinedConfig(
  overrides?: DeepPartial<CombinedConfig>,
): CombinedConfig {
  const baseConfig: CombinedConfig = {
    settings: {
      metadata: {
        steam: {
          enabled: true,
          fetch_achievements: true,
          fetch_playtime: true,
        },
        igdb: {
          enabled: true,
        },
      },
      assets: {
        steam_grid_db: {
          enabled: true,
        },
        tmdb: {
          enabled: true,
        },
      },
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
    },
    app: {
      server: {
        host: "0.0.0.0",
        port: 3005,
      },
    },
  };

  return deepMerge(baseConfig, overrides as Record<string, unknown>);
}

export function createSteamConfig(
  overrides?: DeepPartial<CombinedConfig>,
): CombinedConfig {
  return createCombinedConfig(
    deepMerge(
      {
        secrets: {
          storefronts: {
            steam: {
              api_key: "test-steam-api-key",
              steam_id: "76561198000000000",
            },
          },
        },
      },
      overrides as Record<string, unknown>,
    ),
  );
}

export function createSteamGridDbConfig(
  overrides?: DeepPartial<CombinedConfig>,
): CombinedConfig {
  return createCombinedConfig(
    deepMerge(
      {
        secrets: {
          asset_store: {
            steam_grid_db: {
              api_key: "test-sgdb-api-key",
            },
          },
        },
      },
      overrides as Record<string, unknown>,
    ),
  );
}

export function createTmdbConfig(
  overrides?: DeepPartial<CombinedConfig>,
): CombinedConfig {
  return createCombinedConfig(
    deepMerge(
      {
        secrets: {
          asset_store: {
            tmdb: {
              api_key: "test-tmdb-api-key",
            },
          },
        },
      },
      overrides as Record<string, unknown>,
    ),
  );
}

export function createIgdbConfig(
  overrides?: DeepPartial<CombinedConfig>,
): CombinedConfig {
  return createCombinedConfig(
    deepMerge(
      {
        secrets: {
          metadata_store: {
            igdb: {
              client_id: "test-twitch-client-id",
              client_secret: "test-twitch-client-secret",
            },
          },
        },
      },
      overrides as Record<string, unknown>,
    ),
  );
}
