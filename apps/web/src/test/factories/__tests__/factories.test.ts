import { describe, it, expect } from "vitest";
import {
  createMedia,
  createAsset,
  createLaunch,
  createMediaInstallation,
  createMediaStorefront,
  createMediaDetails,
  createMediaResponse,
  createAgent,
  createActiveMediaSession,
  createCombinedConfig,
  createSteamConfig,
  createSteamGridDbConfig,
  createTmdbConfig,
  createIgdbConfig,
} from "../index";

describe("Test Factories", () => {
  describe("createMedia", () => {
    it("creates a media entity with valid default values", () => {
      const media = createMedia();

      expect(media).toEqual({
        id: "media-1",
        title: "Test Media",
        media_type: "game",
        status: "not_started",
        external_ids: [],
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
      });
    });

    it("allows overriding specific media fields", () => {
      const media = createMedia({
        id: "custom-media-99",
        title: "Elden Ring",
        status: "in_progress",
        release_date: 1645747200,
        assets: [createAsset({ id: "poster-1", asset_type: "poster" })],
      });

      expect(media.id).toBe("custom-media-99");
      expect(media.title).toBe("Elden Ring");
      expect(media.status).toBe("in_progress");
      expect(media.release_date).toBe(1645747200);
      expect(media.assets).toHaveLength(1);
      expect(media.assets[0]).toEqual({ id: "poster-1", asset_type: "poster" });
      // Untouched fields maintain defaults
      expect(media.media_type).toBe("game");
      expect(media.genres).toEqual([]);
    });
  });

  describe("createAsset", () => {
    it("creates an asset with valid default values", () => {
      const asset = createAsset();
      expect(asset).toEqual({
        id: "asset-1",
        asset_type: "poster",
      });
    });

    it("allows overriding asset fields", () => {
      const asset = createAsset({ id: "banner-123", asset_type: "banner" });
      expect(asset).toEqual({
        id: "banner-123",
        asset_type: "banner",
      });
    });
  });

  describe("createLaunch", () => {
    it("creates a launch profile with default values", () => {
      const launch = createLaunch();
      expect(launch).toEqual({
        id: "launch-1",
        name: "Default Profile",
        agent_id: null,
      });
    });

    it("allows overriding launch profile fields", () => {
      const launch = createLaunch({
        id: "dx11",
        name: "DirectX 11",
        agent_id: "agent-1",
      });
      expect(launch).toEqual({
        id: "dx11",
        name: "DirectX 11",
        agent_id: "agent-1",
      });
    });
  });

  describe("createMediaInstallation", () => {
    it("creates a media installation with default values", () => {
      const installation = createMediaInstallation();
      expect(installation).toEqual({
        id: "installation-1",
        agent_id: "agent-1",
        external_id: null,
        path: null,
        storefront_id: null,
      });
    });

    it("allows overriding installation fields", () => {
      const installation = createMediaInstallation({
        path: "/opt/games/elden-ring",
        storefront_id: "Steam",
      });
      expect(installation.path).toBe("/opt/games/elden-ring");
      expect(installation.storefront_id).toBe("Steam");
    });
  });

  describe("createMediaStorefront", () => {
    it("creates a storefront with default values", () => {
      const storefront = createMediaStorefront();
      expect(storefront).toEqual({
        external_id: "steam-1",
        storefront_id: "Steam",
        last_played_at: null,
        playtime_minutes: null,
      });
    });

    it("allows overriding storefront fields", () => {
      const storefront = createMediaStorefront({
        external_id: "steam-730",
        playtime_minutes: 360,
        last_played_at: 1700000000,
      });
      expect(storefront.external_id).toBe("steam-730");
      expect(storefront.playtime_minutes).toBe(360);
      expect(storefront.last_played_at).toBe(1700000000);
    });
  });

  describe("createMediaDetails", () => {
    it("creates media game details with default values", () => {
      const details = createMediaDetails();
      expect(details).toEqual({
        developers: [],
        publishers: [],
        last_played_at: null,
        playtime_minutes: null,
        series: null,
      });
    });

    it("allows overriding details fields", () => {
      const details = createMediaDetails({
        developers: ["FromSoftware"],
        playtime_minutes: 180,
        series: "Dark Souls",
      });
      expect(details.developers).toEqual(["FromSoftware"]);
      expect(details.playtime_minutes).toBe(180);
      expect(details.series).toBe("Dark Souls");
    });
  });

  describe("createMediaResponse", () => {
    it("creates paginated media response with default values", () => {
      const response = createMediaResponse();
      expect(response).toEqual({
        items: [],
        total: 0,
        page: 1,
        limit: 24,
        total_pages: 1,
        has_next: false,
      });
    });

    it("automatically computes total from items if not explicitly provided", () => {
      const items = [createMedia({ id: "m1" }), createMedia({ id: "m2" })];
      const response = createMediaResponse({ items });
      expect(response.items).toHaveLength(2);
      expect(response.total).toBe(2);
    });
  });

  describe("createAgent", () => {
    it("creates agent entity with default values", () => {
      const agent = createAgent();
      expect(agent).toEqual({
        id: "agent-1",
        name: "Desktop-Agent",
        hostname: "desktop-pc",
        platform: "linux",
        is_connected: true,
        created_at: 1000,
        agent_version: null,
        connected_at: null,
        ip_address: null,
        last_seen_at: null,
      });
    });

    it("allows overriding agent properties", () => {
      const agent = createAgent({
        id: "agent-custom",
        name: "Custom Node",
        is_connected: false,
      });
      expect(agent.id).toBe("agent-custom");
      expect(agent.name).toBe("Custom Node");
      expect(agent.is_connected).toBe(false);
    });
  });

  describe("createActiveMediaSession", () => {
    it("creates active media session with default values", () => {
      const session = createActiveMediaSession();
      expect(session).toEqual({
        mediaId: "media-1",
        launchId: "launch-1",
        agentId: "agent-1",
        startedAt: 1700000000000,
      });
    });

    it("allows overriding session properties", () => {
      const session = createActiveMediaSession({
        mediaId: "game-99",
        startedAt: 1600000000,
      });
      expect(session.mediaId).toBe("game-99");
      expect(session.startedAt).toBe(1600000000);
    });
  });

  describe("createCombinedConfig", () => {
    it("creates combined config with defaults", () => {
      const config = createCombinedConfig();
      expect(config.app.server.host).toBe("0.0.0.0");
      expect(config.app.server.port).toBe(3005);
      expect(config.settings.metadata.steam.enabled).toBe(true);
      expect(config.secrets.storefronts.steam.api_key).toBe(
        "steam-api-key-xyz",
      );
    });

    it("allows deeply overriding nested config properties without losing defaults", () => {
      const config = createCombinedConfig({
        secrets: {
          storefronts: {
            steam: {
              api_key: "custom-steam-key",
            },
          },
        },
      });
      expect(config.secrets.storefronts.steam.api_key).toBe("custom-steam-key");
      expect(config.secrets.storefronts.steam.steam_id).toBe("76561198000000000");
      expect(config.secrets.asset_store.steam_grid_db.api_key).toBe("sgdb-api-key-123");
      expect(config.settings.metadata.steam.enabled).toBe(true);
      expect(config.app.server.host).toBe("0.0.0.0");
    });
  });

  describe("drawer config preset factories", () => {
    it("creates Steam config with test defaults and allows overrides", () => {
      const config = createSteamConfig();
      expect(config.secrets.storefronts.steam.api_key).toBe("test-steam-api-key");
      expect(config.secrets.storefronts.steam.steam_id).toBe("76561198000000000");

      const overridden = createSteamConfig({
        secrets: {
          storefronts: {
            steam: {
              api_key: "another-key",
            },
          },
        },
      });
      expect(overridden.secrets.storefronts.steam.api_key).toBe("another-key");
    });

    it("creates SteamGridDB config with test defaults", () => {
      const config = createSteamGridDbConfig();
      expect(config.secrets.asset_store.steam_grid_db.api_key).toBe("test-sgdb-api-key");
      expect(config.settings.assets.steam_grid_db.enabled).toBe(true);
    });

    it("creates TMDB config with test defaults", () => {
      const config = createTmdbConfig();
      expect(config.secrets.asset_store.tmdb.api_key).toBe("test-tmdb-api-key");
      expect(config.settings.assets.tmdb?.enabled).toBe(true);
    });

    it("creates IGDB config with test defaults", () => {
      const config = createIgdbConfig();
      expect(config.secrets.metadata_store.igdb.client_id).toBe("test-twitch-client-id");
      expect(config.secrets.metadata_store.igdb.client_secret).toBe("test-twitch-client-secret");
      expect(config.settings.metadata.igdb?.enabled).toBe(true);
    });
  });
});
