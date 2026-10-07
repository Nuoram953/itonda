import { describe, it, expect, vi, beforeEach } from "vitest";
import {
  render,
  screen,
  fireEvent,
  waitFor,
  createTmdbConfig,
} from "@/test/test-utils";
import { TmdbDrawer } from "../TmdbDrawer";

const mockConfigData = createTmdbConfig();

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

describe("TmdbDrawer", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders drawer fields when open", () => {
    render(<TmdbDrawer open={true} onOpenChange={vi.fn()} />);

    expect(
      screen.getByText("The Movie Database (TMDB) Integration"),
    ).toBeDefined();
    expect(screen.getByDisplayValue("test-tmdb-api-key")).toBeDefined();
    expect(
      screen.getByRole("switch", { name: "Toggle TMDB Enabled" }),
    ).toBeDefined();
    expect(screen.getByText("Get TMDB API key")).toBeDefined();
  });

  it("calls mutateAsync and onOpenChange when clicking Done button", async () => {
    const onOpenChangeMock = vi.fn();
    render(<TmdbDrawer open={true} onOpenChange={onOpenChangeMock} />);

    const apiKeyInput = screen.getByDisplayValue("test-tmdb-api-key");
    fireEvent.change(apiKeyInput, { target: { value: "new-tmdb-key-456" } });

    const doneBtn = screen.getByRole("button", { name: "Done" });
    fireEvent.click(doneBtn);

    await waitFor(() => {
      expect(mockMutateAsync).toHaveBeenCalledTimes(1);
    });

    expect(mockMutateAsync).toHaveBeenCalledWith({
      settings: {
        assets: {
          tmdb: {
            enabled: true,
          },
        },
      },
      secrets: {
        asset_store: {
          tmdb: {
            api_key: "new-tmdb-key-456",
          },
        },
      },
    });

    expect(onOpenChangeMock).toHaveBeenCalledWith(false);
  });

  it("toggles enabled switch and triggers save", async () => {
    render(<TmdbDrawer open={true} onOpenChange={vi.fn()} />);

    const switchBtn = screen.getByRole("switch", {
      name: "Toggle TMDB Enabled",
    });
    fireEvent.click(switchBtn);

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
      secrets: {
        asset_store: {
          tmdb: {
            api_key: "test-tmdb-api-key",
          },
        },
      },
    });
  });
});
