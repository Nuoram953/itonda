import { describe, it, expect, vi, beforeEach } from "vitest";
import {
  render,
  screen,
  fireEvent,
  waitFor,
  createCombinedConfig,
} from "@/test/test-utils";
import { SteamDrawer } from "../SteamDrawer";

const mockConfigData = createCombinedConfig({
  secrets: {
    storefronts: {
      steam: {
        api_key: "test-steam-api-key",
        steam_id: "76561198000000000",
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

const mockMutateAsync = vi.fn().mockResolvedValue(mockConfigData);


vi.mock("../../../api/get-config", () => ({
  useConfig: () => ({
    data: mockConfigData,
    isLoading: false,
    isPending: false,
  }),
  getConfig: vi.fn(),
  getConfigQueryOptions: vi.fn(),
}));

vi.mock("../../../api/patch-config", () => ({
  usePatchConfig: () => ({
    mutateAsync: mockMutateAsync,
    mutate: mockMutateAsync,
    isPending: false,
  }),
  patchConfig: vi.fn(),
}));

describe("SteamDrawer", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders drawer fields when open", () => {
    render(<SteamDrawer open={true} onOpenChange={vi.fn()} />);

    expect(screen.getByText("Steam Integration")).toBeDefined();
    expect(screen.getByDisplayValue("test-steam-api-key")).toBeDefined();
    expect(screen.getByDisplayValue("76561198000000000")).toBeDefined();
    expect(screen.getByText("Sync Playtime & Last Played")).toBeDefined();
    expect(screen.getByText("Fetch Achievements")).toBeDefined();
  });

  it("shows validation error when entering non-numeric characters in Steam ID", () => {
    render(<SteamDrawer open={true} onOpenChange={vi.fn()} />);

    const steamIdInput = screen.getByDisplayValue("76561198000000000");
    fireEvent.change(steamIdInput, { target: { value: "invalid_id_abc" } });

    expect(
      screen.getByText("Steam ID must contain numbers only"),
    ).toBeDefined();
  });

  it("calls mutateAsync and onOpenChange when clicking Done button", async () => {
    const onOpenChangeMock = vi.fn();
    render(<SteamDrawer open={true} onOpenChange={onOpenChangeMock} />);

    const doneBtn = screen.getByRole("button", { name: "Done" });
    fireEvent.click(doneBtn);

    await waitFor(() => {
      expect(mockMutateAsync).toHaveBeenCalledTimes(1);
    });
    expect(onOpenChangeMock).toHaveBeenCalledWith(false);
  });
});
