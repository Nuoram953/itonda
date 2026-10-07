import { describe, it, expect, vi, beforeEach } from "vitest";
import {
  render,
  screen,
  fireEvent,
  waitFor,
  createSteamGridDbConfig,
} from "@/test/test-utils";
import { SteamGridDbDrawer } from "../SteamGridDbDrawer";

const mockConfigData = createSteamGridDbConfig();

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

describe("SteamGridDbDrawer", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders drawer fields when open", () => {
    render(<SteamGridDbDrawer open={true} onOpenChange={vi.fn()} />);

    expect(screen.getByText("SteamGridDB Integration")).toBeDefined();
    expect(screen.getByDisplayValue("test-sgdb-api-key")).toBeDefined();
    expect(
      screen.getByRole("switch", { name: "Toggle SteamGridDB Enabled" }),
    ).toBeDefined();
    expect(screen.getByText("Get SteamGridDB API key")).toBeDefined();
  });

  it("calls mutateAsync and onOpenChange when clicking Done button", async () => {
    const onOpenChangeMock = vi.fn();
    render(<SteamGridDbDrawer open={true} onOpenChange={onOpenChangeMock} />);

    const apiKeyInput = screen.getByDisplayValue("test-sgdb-api-key");
    fireEvent.change(apiKeyInput, { target: { value: "new-api-key-123" } });

    const doneBtn = screen.getByRole("button", { name: "Done" });
    fireEvent.click(doneBtn);

    await waitFor(() => {
      expect(mockMutateAsync).toHaveBeenCalledTimes(1);
    });

    expect(mockMutateAsync).toHaveBeenCalledWith({
      settings: {
        assets: {
          steam_grid_db: {
            enabled: true,
          },
        },
      },
      secrets: {
        asset_store: {
          steam_grid_db: {
            api_key: "new-api-key-123",
          },
        },
      },
    });

    expect(onOpenChangeMock).toHaveBeenCalledWith(false);
  });

  it("toggles enabled switch and triggers save", async () => {
    render(<SteamGridDbDrawer open={true} onOpenChange={vi.fn()} />);

    const switchBtn = screen.getByRole("switch", {
      name: "Toggle SteamGridDB Enabled",
    });
    fireEvent.click(switchBtn);

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
      secrets: {
        asset_store: {
          steam_grid_db: {
            api_key: "test-sgdb-api-key",
          },
        },
      },
    });
  });
});
