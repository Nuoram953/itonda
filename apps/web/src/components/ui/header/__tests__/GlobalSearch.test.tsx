import { describe, it, expect, vi, beforeEach } from "vitest";
import {
  render,
  screen,
  fireEvent,
  waitFor,
  act,
  createMedia,
  createLaunch,
  createAgent,
  createMediaResponse,
  createActiveMediaSession,
} from "@/test/test-utils";
import { GlobalSearch } from "../GlobalSearch";
import * as getMediaModule from "@/features/library/api/get-media";
import { useActiveMedia } from "@/hooks/use-active-media";

const mockNavigate = vi.fn();
const mockLaunchMutate = vi.fn();
const mockNotify = {
  success: vi.fn(),
  error: vi.fn(),
  info: vi.fn(),
};

vi.mock("@tanstack/react-router", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@tanstack/react-router")>();
  return {
    ...actual,
    useNavigate: () => mockNavigate,
  };
});

vi.mock("@/features/details/api/post-media-launch", () => ({
  useLaunchMedia: () => ({
    mutate: mockLaunchMutate,
    isPending: false,
  }),
}));

vi.mock("@/api/get-agents", () => ({
  useAgents: () => ({
    data: {
      agents: [createAgent({ id: "agent-1", is_connected: true })],
    },
    isLoading: false,
  }),
}));

vi.mock("@/hooks/use-websocket-status", () => ({
  useWebSocketStatus: () => "connected",
}));

vi.mock("@/hooks/use-notification", () => ({
  useNotification: () => ({
    notify: mockNotify,
    notifications: [],
    remove: vi.fn(),
    clearAll: vi.fn(),
  }),
}));

vi.mock("@/hooks/use-active-media", () => ({
  useActiveMedia: vi.fn(() => ({
    session: null,
    isPlaying: false,
    formattedElapsed: "00:00",
  })),
}));

vi.mock("@/features/library/api/get-media", () => ({
  getMedia: vi.fn(),
  getMediaQueryOptions: (params: getMediaModule.GetMediaParams) => ({
    queryKey: ["media", params],
    queryFn: () => vi.mocked(getMediaModule.getMedia)(params),
  }),
}));

describe("GlobalSearch Component", () => {
  const singleProfileGame = createMedia({
    id: "game-1",
    title: "Elden Ring",
    media_type: "game",
    status: "in_progress",
    launches: [createLaunch({ id: "launch-1", name: "Default Profile" })],
  });

  const multiProfileGame = createMedia({
    id: "game-2",
    title: "Cyberpunk 2077",
    media_type: "game",
    status: "not_started",
    launches: [
      createLaunch({ id: "launch-dx12", name: "DirectX 12" }),
      createLaunch({ id: "launch-vk", name: "Vulkan" }),
    ],
  });

  const movieMedia = createMedia({
    id: "movie-1",
    title: "Interstellar",
    media_type: "movie",
    status: "completed",
    launches: [],
  });

  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(useActiveMedia).mockReturnValue({
      session: null,
      isPlaying: false,
      formattedElapsed: "00:00",
      media: undefined,
      isLoadingMedia: false,
      elapsedSeconds: 0,
      setActiveSession: vi.fn(),
    });
  });

  it("renders search input with placeholder and ⌘K shortcut badge", () => {
    render(<GlobalSearch />);

    expect(
      screen.getByPlaceholderText("Search media, games, movies..."),
    ).toBeDefined();
    expect(screen.getByText("⌘K")).toBeDefined();
  });

  it("searches and renders matching media results when typing letters", async () => {
    vi.mocked(getMediaModule.getMedia).mockResolvedValueOnce(
      createMediaResponse({ items: [singleProfileGame, movieMedia] }),
    );

    render(<GlobalSearch />);

    const input = screen.getByPlaceholderText("Search media, games, movies...");
    fireEvent.change(input, { target: { value: "el" } });

    await waitFor(() => {
      expect(screen.getByText("Elden Ring")).toBeDefined();
    });

    expect(screen.getByText("Interstellar")).toBeDefined();
    expect(screen.getByText("In Progress")).toBeDefined();
    expect(screen.getByText("Completed")).toBeDefined();
  });

  it("shows empty state when no media items match query", async () => {
    vi.mocked(getMediaModule.getMedia).mockResolvedValueOnce(
      createMediaResponse({ items: [] }),
    );

    render(<GlobalSearch />);

    const input = screen.getByPlaceholderText("Search media, games, movies...");
    fireEvent.change(input, { target: { value: "nonexistent" } });

    await waitFor(() => {
      expect(
        screen.getByText(/no media found matching/i),
      ).toBeDefined();
    });
  });

  it("navigates rows with ArrowDown and ArrowUp", async () => {
    vi.mocked(getMediaModule.getMedia).mockResolvedValueOnce(
      createMediaResponse({ items: [singleProfileGame, movieMedia] }),
    );

    render(<GlobalSearch />);

    const input = screen.getByPlaceholderText("Search media, games, movies...");
    fireEvent.change(input, { target: { value: "test" } });

    await waitFor(() => {
      expect(screen.getByText("Elden Ring")).toBeDefined();
    });

    const rows = screen.getAllByRole("option");
    expect(rows[0].getAttribute("aria-selected")).toBe("true");
    expect(rows[1].getAttribute("aria-selected")).toBe("false");

    // Arrow down to second row
    fireEvent.keyDown(input, { key: "ArrowDown" });
    expect(rows[0].getAttribute("aria-selected")).toBe("false");
    expect(rows[1].getAttribute("aria-selected")).toBe("true");

    // Arrow up back to first row
    fireEvent.keyDown(input, { key: "ArrowUp" });
    expect(rows[0].getAttribute("aria-selected")).toBe("true");
    expect(rows[1].getAttribute("aria-selected")).toBe("false");
  });

  it("strictly toggles between 'Start Game' and 'Details' using Tab on a launchable game (Option A)", async () => {
    vi.mocked(getMediaModule.getMedia).mockResolvedValueOnce(
      createMediaResponse({ items: [singleProfileGame] }),
    );

    render(<GlobalSearch />);

    const input = screen.getByPlaceholderText("Search media, games, movies...");
    fireEvent.change(input, { target: { value: "elden" } });

    await waitFor(() => {
      expect(screen.getByText("Elden Ring")).toBeDefined();
    });

    const startButton = screen.getByRole("button", { name: /start game/i });
    const detailsButton = screen.getByRole("button", { name: /details/i });

    // Initially, for a launchable game, activeAction is "start"
    expect(startButton.className).toContain("bg-primary");
    expect(detailsButton.className).not.toContain("bg-primary");

    // Press Tab: switches from "start" to "details"
    fireEvent.keyDown(input, { key: "Tab" });
    expect(detailsButton.className).toContain("bg-primary");
    expect(startButton.className).not.toContain("bg-primary");

    // Press Tab again: strictly toggles back from "details" to "start"
    fireEvent.keyDown(input, { key: "Tab" });
    expect(startButton.className).toContain("bg-primary");
    expect(detailsButton.className).not.toContain("bg-primary");
  });

  it("keeps 'Details' active on Tab when item is not launchable (e.g. Movie)", async () => {
    vi.mocked(getMediaModule.getMedia).mockResolvedValueOnce(
      createMediaResponse({ items: [movieMedia] }),
    );

    render(<GlobalSearch />);

    const input = screen.getByPlaceholderText("Search media, games, movies...");
    fireEvent.change(input, { target: { value: "inter" } });

    await waitFor(() => {
      expect(screen.getByText("Interstellar")).toBeDefined();
    });

    const detailsButton = screen.getByRole("button", { name: /details/i });
    expect(detailsButton.className).toContain("bg-primary");

    // Press Tab on non-launchable media
    fireEvent.keyDown(input, { key: "Tab" });
    expect(detailsButton.className).toContain("bg-primary");
  });

  it("launches game directly on Enter when 'Start Game' is active for single-profile game", async () => {
    vi.mocked(getMediaModule.getMedia).mockResolvedValueOnce(
      createMediaResponse({ items: [singleProfileGame] }),
    );

    render(<GlobalSearch />);

    const input = screen.getByPlaceholderText("Search media, games, movies...");
    fireEvent.change(input, { target: { value: "elden" } });

    await waitFor(() => {
      expect(screen.getByText("Elden Ring")).toBeDefined();
    });

    // Press Enter while 'start' is active
    fireEvent.keyDown(input, { key: "Enter" });

    expect(mockLaunchMutate).toHaveBeenCalledWith("launch-1", expect.any(Object));
  });

  it("navigates to /media/:mediaId on Enter when 'Details' is active", async () => {
    vi.mocked(getMediaModule.getMedia).mockResolvedValueOnce(
      createMediaResponse({ items: [singleProfileGame] }),
    );

    render(<GlobalSearch />);

    const input = screen.getByPlaceholderText("Search media, games, movies...");
    fireEvent.change(input, { target: { value: "elden" } });

    await waitFor(() => {
      expect(screen.getByText("Elden Ring")).toBeDefined();
    });

    // Press Tab to switch to 'detail'
    fireEvent.keyDown(input, { key: "Tab" });

    // Press Enter to go to detail page
    fireEvent.keyDown(input, { key: "Enter" });

    expect(mockNavigate).toHaveBeenCalledWith({
      to: "/media/$mediaId",
      params: { mediaId: "game-1" },
    });
  });

  it("opens profile selector dialog on Enter when game has multiple launch profiles", async () => {
    vi.mocked(getMediaModule.getMedia).mockResolvedValueOnce(
      createMediaResponse({ items: [multiProfileGame] }),
    );

    render(<GlobalSearch />);

    const input = screen.getByPlaceholderText("Search media, games, movies...");
    fireEvent.change(input, { target: { value: "cyber" } });

    await waitFor(() => {
      expect(screen.getByText("Cyberpunk 2077")).toBeDefined();
    });

    // Press Enter while 'Start Game' is active
    fireEvent.keyDown(input, { key: "Enter" });

    // Modal opens with profiles
    expect(screen.getByText("Select launch profile")).toBeDefined();
    expect(screen.getByRole("button", { name: "DirectX 12" })).toBeDefined();
    expect(screen.getByRole("button", { name: "Vulkan" })).toBeDefined();

    // Clicking Vulkan profile launches it
    fireEvent.click(screen.getByRole("button", { name: "Vulkan" }));
    expect(mockLaunchMutate).toHaveBeenCalledWith("launch-vk", expect.any(Object));
  });

  it("closes dropdown on Escape key", async () => {
    vi.mocked(getMediaModule.getMedia).mockResolvedValueOnce(
      createMediaResponse({ items: [singleProfileGame] }),
    );

    render(<GlobalSearch />);

    const input = screen.getByPlaceholderText("Search media, games, movies...");
    fireEvent.change(input, { target: { value: "elden" } });

    await waitFor(() => {
      expect(screen.getByText("Elden Ring")).toBeDefined();
    });

    fireEvent.keyDown(input, { key: "Escape" });

    await waitFor(() => {
      expect(screen.queryByText("Elden Ring")).toBeNull();
    });
  });

  it("clears query and closes dropdown when clicking X button", async () => {
    vi.mocked(getMediaModule.getMedia).mockResolvedValueOnce(
      createMediaResponse({ items: [singleProfileGame] }),
    );

    render(<GlobalSearch />);

    const input = screen.getByPlaceholderText("Search media, games, movies...");
    fireEvent.change(input, { target: { value: "elden" } });

    await waitFor(() => {
      expect(screen.getByText("Elden Ring")).toBeDefined();
    });

    const clearButton = screen.getByLabelText("Clear search");
    fireEvent.click(clearButton);

    expect(screen.queryByText("Elden Ring")).toBeNull();
    expect((input as HTMLInputElement).value).toBe("");
  });

  it("focuses input when pressing ⌘K or Ctrl+K shortcut", () => {
    render(<GlobalSearch />);

    const input = screen.getByPlaceholderText("Search media, games, movies...");
    expect(document.activeElement).not.toBe(input);

    act(() => {
      window.dispatchEvent(
        new KeyboardEvent("keydown", { key: "k", metaKey: true }),
      );
    });

    expect(document.activeElement).toBe(input);
  });

  it("indicates 'Playing' when active media session matches", async () => {
    vi.mocked(useActiveMedia).mockReturnValue({
      session: createActiveMediaSession({
        mediaId: "game-1",
      }),
      isPlaying: true,
      formattedElapsed: "02:00",
      media: undefined,
      isLoadingMedia: false,
      elapsedSeconds: 120,
      setActiveSession: vi.fn(),
    });

    vi.mocked(getMediaModule.getMedia).mockResolvedValueOnce(
      createMediaResponse({ items: [singleProfileGame] }),
    );

    render(<GlobalSearch />);

    const input = screen.getByPlaceholderText("Search media, games, movies...");
    fireEvent.change(input, { target: { value: "elden" } });

    await waitFor(() => {
      expect(screen.getByText("Elden Ring")).toBeDefined();
    });

    expect(screen.getByText("Playing")).toBeDefined();
  });
});
