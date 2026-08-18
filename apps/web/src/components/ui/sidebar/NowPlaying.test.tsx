import { describe, it, expect, vi, beforeEach } from "vitest";
import { screen, renderWithRouter } from "@/test/test-utils";
import { NowPlaying } from "./NowPlaying";
import { useActiveMedia } from "@/hooks/use-active-media";
import type { components } from "@/api/generated.d";

vi.mock("@/hooks/use-active-media", () => ({
  useActiveMedia: vi.fn(),
}));

describe("NowPlayingGame", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders nothing when no game is active", () => {
    vi.mocked(useActiveMedia).mockReturnValue({
      session: null,
      media: undefined,
      isLoadingMedia: false,
      isPlaying: false,
      elapsedSeconds: 0,
      formattedElapsed: "00:00",
      setActiveSession: vi.fn(),
    });

    renderWithRouter(<NowPlaying />);
    expect(screen.queryByTitle(/Now Playing/i)).toBeNull();
  });

  it("renders game title, live elapsed time, and poster when playing", async () => {
    const mockMedia: components["schemas"]["Media"] = {
      id: "media-game-1",
      title: "Cyberpunk 2077",
      media_type: "game",
      status: "in_progress",
      launches: [{ id: "launch-1", name: "Default" }],
      assets: [{ id: "poster-cp2077", asset_type: "poster" }],
      storefronts: [],
      installations: [],
    };

    vi.mocked(useActiveMedia).mockReturnValue({
      session: {
        mediaId: "media-game-1",
        launchId: "launch-1",
        agentId: "agent-1",
        startedAt: Date.now() - 125000,
      },
      media: mockMedia,
      isLoadingMedia: false,
      isPlaying: true,
      elapsedSeconds: 125,
      formattedElapsed: "02:05",
      setActiveSession: vi.fn(),
    });

    renderWithRouter(<NowPlaying />);

    expect(
      await screen.findByTitle(/Now Playing: Cyberpunk 2077/i),
    ).toBeDefined();
    expect(await screen.findByText("Cyberpunk 2077")).toBeDefined();
    expect(await screen.findByText("02:05")).toBeDefined();

    const images = (await screen.findAllByAltText(
      "Cyberpunk 2077",
    )) as HTMLImageElement[];
    expect(images.length).toBeGreaterThan(0);
    expect(images[0].src).toContain("/api/v1/assets/poster-cp2077");
  });

  it("renders fallback gamepad icon when poster is not present", async () => {
    const mockMedia: components["schemas"]["Media"] = {
      id: "media-game-2",
      title: "Hollow Knight",
      media_type: "game",
      status: "in_progress",
      launches: [],
      assets: [],
      storefronts: [],
      installations: [],
    };

    vi.mocked(useActiveMedia).mockReturnValue({
      session: {
        mediaId: "media-game-2",
        launchId: "launch-2",
        agentId: "agent-1",
        startedAt: Date.now() - 60000,
      },
      media: mockMedia,
      isLoadingMedia: false,
      isPlaying: true,
      elapsedSeconds: 60,
      formattedElapsed: "01:00",
      setActiveSession: vi.fn(),
    });

    renderWithRouter(<NowPlaying />);

    expect(await screen.findByText("Hollow Knight")).toBeDefined();
    expect(await screen.findByText("01:00")).toBeDefined();
    expect(screen.queryByRole("img")).toBeNull();
  });
});
