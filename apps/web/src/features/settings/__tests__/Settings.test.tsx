import { describe, it, expect, vi, beforeEach } from "vitest";
import {
  render,
  screen,
  fireEvent,
  waitFor,
  createCombinedConfig,
} from "@/test/test-utils";
import { Settings } from "../index";

const mockConfigData = createCombinedConfig({
  app: {
    server: {
      host: "127.0.0.1",
      port: 3005,
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
        api_key: "sgdb-key",
      },
      tmdb: {
        api_key: "tmdb-key",
      },
    },
    metadata_store: {
      igdb: {
        client_id: "igdb-client-id-xyz",
        client_secret: "igdb-client-secret-xyz",
      },
    },
  },
});

let currentConfig = mockConfigData;

const mockMutateAsync = vi.fn().mockResolvedValue(mockConfigData);

vi.mock("../api/get-config", () => ({
  useConfig: () => ({
    data: currentConfig,
    isLoading: false,
    isPending: false,
  }),
  getConfig: vi.fn(),
  getConfigQueryOptions: vi.fn(),
}));

vi.mock("../api/patch-config", () => ({
  usePatchConfig: () => ({
    mutate: mockMutateAsync,
    mutateAsync: mockMutateAsync,
    isPending: false,
  }),
  patchConfig: vi.fn(),
}));

describe("Settings Page with Vertical Layout", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    currentConfig = mockConfigData;
  });

  it("renders Settings page with vertical Storefronts, Assets, and Metadata sections", () => {
    render(<Settings />);

    expect(screen.getByRole("heading", { name: "Settings" })).toBeDefined();
    expect(screen.getByRole("heading", { name: "Storefronts" })).toBeDefined();
    expect(
      screen.getByText(
        "Connect gaming platforms and digital storefronts to sync libraries, playtimes, and achievements.",
      ),
    ).toBeDefined();
    expect(screen.getByRole("heading", { name: "Assets" })).toBeDefined();
    expect(
      screen.getByText(
        "Configure artwork, posters, banners, and media providers for your library.",
      ),
    ).toBeDefined();
    expect(screen.getByRole("heading", { name: "Metadata" })).toBeDefined();
    expect(
      screen.getByText(
        "Configure metadata providers to fetch summaries, release dates, and developer details.",
      ),
    ).toBeDefined();

    expect(screen.getByText("Steam")).toBeDefined();
    expect(screen.getByText("SteamGridDB")).toBeDefined();
    expect(screen.getByText("The Movie Database")).toBeDefined();
    expect(screen.getByText("IGDB")).toBeDefined();

    expect(screen.getAllByText("Game")).toHaveLength(3);
    expect(screen.getByText("Movie")).toBeDefined();
    expect(screen.getByText("TV Show")).toBeDefined();

    expect(screen.queryByText("API key configured")).toBeNull();
    expect(screen.queryByText(/connected as/i)).toBeNull();
  });

  it("displays issue warning when integration is enabled but missing configuration", () => {
    currentConfig = createCombinedConfig({
      secrets: {
        storefronts: {
          steam: {
            api_key: "",
            steam_id: "",
            account_name: null,
            avatar_url: null,
          },
        },
        asset_store: {
          steam_grid_db: {
            api_key: "",
          },
          tmdb: {
            api_key: "",
          },
        },
        metadata_store: {
          igdb: {
            client_id: "",
            client_secret: "",
          },
        },
      },
    });

    render(<Settings />);

    expect(
      screen.getByText("Account not linked — sign in to sync library"),
    ).toBeDefined();
    expect(
      screen.getAllByText("API key required — configure key to fetch assets"),
    ).toHaveLength(2);
    expect(
      screen.getByText(
        "Credentials required — configure Twitch Client ID and Secret to fetch metadata",
      ),
    ).toBeDefined();
  });

  it("instantly auto-saves when flipping Steam row toggle switch", async () => {
    render(<Settings />);

    const steamToggle = screen.getByRole("switch", {
      name: "Toggle Steam",
    });
    fireEvent.click(steamToggle);

    await waitFor(() => {
      expect(mockMutateAsync).toHaveBeenCalledTimes(1);
    });

    expect(mockMutateAsync).toHaveBeenCalledWith({
      settings: {
        metadata: {
          steam: {
            enabled: false,
          },
        },
      },
    });
  });

  it("opens Steam drawer when clicking configure icon and updates config on Done", async () => {
    render(<Settings />);

    const configureSteamBtn = screen.getByRole("button", {
      name: "Configure Steam",
    });
    fireEvent.click(configureSteamBtn);

    expect(screen.getByText("Steam Web API Key")).toBeDefined();
    const steamIdInput = screen.getByDisplayValue("76561198000000000");

    fireEvent.change(steamIdInput, {
      target: { value: "76561198999999999" },
    });

    const doneBtn = screen.getByRole("button", { name: "Done" });
    fireEvent.click(doneBtn);

    await waitFor(() => {
      expect(mockMutateAsync).toHaveBeenCalledTimes(1);
    });

    expect(mockMutateAsync).toHaveBeenCalledWith(
      expect.objectContaining({
        secrets: expect.objectContaining({
          storefronts: {
            steam: {
              api_key: "steam-api-key-xyz",
              steam_id: "76561198999999999",
            },
          },
        }),
      }),
    );
  });

  it("instantly auto-saves when toggling SteamGridDB row switch", async () => {
    render(<Settings />);

    const sgdbToggle = screen.getByRole("switch", {
      name: "Toggle SteamGridDB",
    });
    fireEvent.click(sgdbToggle);

    await waitFor(() => {
      expect(mockMutateAsync).toHaveBeenCalledTimes(1);
    });

    expect(mockMutateAsync).toHaveBeenCalledWith({
      settings: {
        assets: {
          steam_grid_db: {
            enabled: false,
          },
        },
      },
    });
  });

  it("opens SteamGridDbDrawer and updates API key when clicking configure icon on SteamGridDB row", async () => {
    render(<Settings />);

    const configureSgdbBtn = screen.getByRole("button", {
      name: "Configure SteamGridDB",
    });
    fireEvent.click(configureSgdbBtn);

    expect(screen.getByText("SteamGridDB Integration")).toBeDefined();
    const apiKeyInput = screen.getByDisplayValue("sgdb-key");

    fireEvent.change(apiKeyInput, {
      target: { value: "updated-sgdb-key" },
    });

    const doneBtn = screen.getAllByRole("button", { name: "Done" })[0];
    fireEvent.click(doneBtn);

    await waitFor(() => {
      expect(mockMutateAsync).toHaveBeenCalledTimes(1);
    });

    expect(mockMutateAsync).toHaveBeenCalledWith(
      expect.objectContaining({
        secrets: expect.objectContaining({
          asset_store: {
            steam_grid_db: {
              api_key: "updated-sgdb-key",
            },
          },
        }),
      }),
    );
  });

  it("instantly auto-saves when toggling TMDB row switch", async () => {
    render(<Settings />);

    const tmdbToggle = screen.getByRole("switch", {
      name: "Toggle The Movie Database",
    });
    fireEvent.click(tmdbToggle);

    await waitFor(() => {
      expect(mockMutateAsync).toHaveBeenCalledTimes(1);
    });

    expect(mockMutateAsync).toHaveBeenCalledWith({
      settings: {
        assets: {
          tmdb: {
            enabled: false,
          },
        },
      },
    });
  });

  it("opens TmdbDrawer and updates API key when clicking configure icon on TMDB row", async () => {
    render(<Settings />);

    const configureTmdbBtn = screen.getByRole("button", {
      name: "Configure The Movie Database",
    });
    fireEvent.click(configureTmdbBtn);

    expect(
      screen.getByText("The Movie Database (TMDB) Integration"),
    ).toBeDefined();
    const apiKeyInput = screen.getByDisplayValue("tmdb-key");

    fireEvent.change(apiKeyInput, {
      target: { value: "updated-tmdb-key" },
    });

    const doneBtn = screen.getAllByRole("button", { name: "Done" })[0];
    fireEvent.click(doneBtn);

    await waitFor(() => {
      expect(mockMutateAsync).toHaveBeenCalledTimes(1);
    });

    expect(mockMutateAsync).toHaveBeenCalledWith(
      expect.objectContaining({
        secrets: expect.objectContaining({
          asset_store: {
            tmdb: {
              api_key: "updated-tmdb-key",
            },
          },
        }),
      }),
    );
  });

  it("instantly auto-saves when flipping IGDB row toggle switch", async () => {
    render(<Settings />);

    const igdbToggle = screen.getByRole("switch", {
      name: "Toggle IGDB",
    });
    fireEvent.click(igdbToggle);

    await waitFor(() => {
      expect(mockMutateAsync).toHaveBeenCalledTimes(1);
    });

    expect(mockMutateAsync).toHaveBeenCalledWith({
      settings: {
        metadata: {
          igdb: {
            enabled: false,
          },
        },
      },
    });
  });

  it("opens IgdbDrawer and updates credentials when clicking configure icon on IGDB row", async () => {
    render(<Settings />);

    const configureIgdbBtn = screen.getByRole("button", {
      name: "Configure IGDB",
    });
    fireEvent.click(configureIgdbBtn);

    expect(
      screen.getByText("Internet Game Database (IGDB) Integration"),
    ).toBeDefined();
    const clientIdInput = screen.getByDisplayValue("igdb-client-id-xyz");
    const clientSecretInput = screen.getByDisplayValue(
      "igdb-client-secret-xyz",
    );

    fireEvent.change(clientIdInput, {
      target: { value: "updated-igdb-client-id" },
    });
    fireEvent.change(clientSecretInput, {
      target: { value: "updated-igdb-client-secret" },
    });

    const doneBtn = screen.getAllByRole("button", { name: "Done" })[0];
    fireEvent.click(doneBtn);

    await waitFor(() => {
      expect(mockMutateAsync).toHaveBeenCalledTimes(1);
    });

    expect(mockMutateAsync).toHaveBeenCalledWith(
      expect.objectContaining({
        secrets: expect.objectContaining({
          metadata_store: {
            igdb: {
              client_id: "updated-igdb-client-id",
              client_secret: "updated-igdb-client-secret",
            },
          },
        }),
      }),
    );
  });
});
