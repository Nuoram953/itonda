import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@/test/test-utils";
import { Settings } from "../index";
import type { components } from "@/api/generated.d";

const mockConfigData: components["schemas"]["CombinedConfig"] = {
  settings: {
    metadata: {
      steam: {
        enabled: true,
        fetch_achievements: true,
        fetch_playtime: true,
      },
    },
  },
  secrets: {
    storefronts: {
      steam: {
        api_key: "steam-api-key-xyz",
        steam_id: "76561198000000000",
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
  },
  app: {
    server: {
      host: "127.0.0.1",
      port: 3005,
    },
  },
};

const mockMutateAsync = vi.fn().mockResolvedValue(mockConfigData);

vi.mock("../api/get-config", () => ({
  useConfig: () => ({
    data: mockConfigData,
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

describe("Settings Page with TanStack Form & Auto-Save", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders Settings & Integrations page with loaded config values", () => {
    render(<Settings />);

    expect(screen.getByText("Settings & Integrations")).toBeDefined();
    expect(screen.getByText("Steam")).toBeDefined();
    expect(screen.getByRole("tab", { name: "Preferences" })).toBeDefined();
    expect(screen.getByRole("tab", { name: "Storefronts" })).toBeDefined();
  });

  it("filters cards when clicking category tabs", () => {
    render(<Settings />);

    const preferencesFilter = screen.getByRole("tab", {
      name: "Preferences",
    });
    fireEvent.click(preferencesFilter);

    expect(screen.queryByText("Steam")).toBeNull();

    const storefrontsFilter = screen.getByRole("tab", {
      name: "Storefronts",
    });
    fireEvent.click(storefrontsFilter);

    expect(screen.getByText("Steam")).toBeDefined();
  });

  it("instantly auto-saves when flipping a card toggle switch", async () => {
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

  it("auto-saves debounced or when clicking Done in drawer", async () => {
    render(<Settings />);

    const manageButtons = screen.getAllByRole("button", { name: /manage/i });
    fireEvent.click(manageButtons[0]);

    expect(screen.getByText("Steam Web API Key")).toBeDefined();
    const steamIdInput = screen.getByDisplayValue("76561198000000000");

    fireEvent.change(steamIdInput, {
      target: { value: "76561198999999999" },
    });

    // Clicking Done flushes auto-save immediately
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
});

