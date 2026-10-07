import { describe, it, expect, vi, beforeEach } from "vitest";
import {
  render,
  screen,
  fireEvent,
  waitFor,
  createIgdbConfig,
} from "@/test/test-utils";
import { IgdbDrawer } from "../IgdbDrawer";

const mockConfigData = createIgdbConfig();

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

describe("IgdbDrawer", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders drawer fields when open", () => {
    render(<IgdbDrawer open={true} onOpenChange={vi.fn()} />);

    expect(
      screen.getByText("Internet Game Database (IGDB) Integration"),
    ).toBeDefined();
    expect(screen.getByDisplayValue("test-twitch-client-id")).toBeDefined();
    expect(screen.getByDisplayValue("test-twitch-client-secret")).toBeDefined();
    expect(
      screen.getByRole("switch", { name: "Toggle IGDB Enabled" }),
    ).toBeDefined();
    expect(screen.getByText("Twitch Developer Console")).toBeDefined();
  });

  it("calls mutateAsync and onOpenChange when clicking Done button", async () => {
    const onOpenChangeMock = vi.fn();
    render(<IgdbDrawer open={true} onOpenChange={onOpenChangeMock} />);

    const clientIdInput = screen.getByDisplayValue("test-twitch-client-id");
    fireEvent.change(clientIdInput, {
      target: { value: "new-twitch-client-id" },
    });

    const clientSecretInput = screen.getByDisplayValue(
      "test-twitch-client-secret",
    );
    fireEvent.change(clientSecretInput, {
      target: { value: "new-twitch-client-secret" },
    });

    const doneBtn = screen.getByRole("button", { name: "Done" });
    fireEvent.click(doneBtn);

    await waitFor(() => {
      expect(mockMutateAsync).toHaveBeenCalledTimes(1);
    });

    expect(mockMutateAsync).toHaveBeenCalledWith({
      settings: {
        metadata: {
          igdb: {
            enabled: true,
          },
        },
      },
      secrets: {
        metadata_store: {
          igdb: {
            client_id: "new-twitch-client-id",
            client_secret: "new-twitch-client-secret",
          },
        },
      },
    });

    expect(onOpenChangeMock).toHaveBeenCalledWith(false);
  });

  it("toggles enabled switch and triggers save", async () => {
    render(<IgdbDrawer open={true} onOpenChange={vi.fn()} />);

    const switchBtn = screen.getByRole("switch", {
      name: "Toggle IGDB Enabled",
    });
    fireEvent.click(switchBtn);

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
      secrets: {
        metadata_store: {
          igdb: {
            client_id: "test-twitch-client-id",
            client_secret: "test-twitch-client-secret",
          },
        },
      },
    });
  });
});
