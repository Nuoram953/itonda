import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, act, createMedia, createAsset } from "@/test/test-utils";
import { MediaDetails } from "../index";
import * as activeMediaModule from "@/hooks/use-active-media";
import * as appFocusModule from "@/hooks/use-app-focus";
import * as mediaApiModule from "../api/get-media-id";

vi.mock("@tanstack/react-router", () => ({
  useParams: () => ({ mediaId: "media-1" }),
  Link: ({ children }: { children: React.ReactNode }) => children,
}));

vi.mock("../api/get-media-id", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../api/get-media-id")>();
  return {
    ...actual,
    useMediaById: vi.fn(),
  };
});

vi.mock("@/hooks/use-active-media", () => ({
  useActiveMedia: vi.fn(),
}));

vi.mock("@/hooks/use-app-focus", () => ({
  useAppFocus: vi.fn(),
}));

describe("MediaDetails Audio playback", () => {
  const mediaWithAudio = createMedia({
    id: "media-1",
    title: "Elden Ring",
    assets: [
      createAsset({
        id: "audio-asset-1",
        asset_type: "audio",
        name: "Main Theme",
      }),
    ],
  });

  let playSpy: any;
  let pauseSpy: any;

  beforeEach(() => {
    vi.clearAllMocks();

    playSpy = vi.spyOn(window.HTMLMediaElement.prototype, "play").mockImplementation(() => Promise.resolve());
    pauseSpy = vi.spyOn(window.HTMLMediaElement.prototype, "pause").mockImplementation(() => {});

    vi.mocked(mediaApiModule.useMediaById).mockReturnValue({
      data: mediaWithAudio,
      isPending: false,
    } as any);

    // Default: application is focused, no game is playing
    vi.mocked(appFocusModule.useAppFocus).mockReturnValue(true);
    vi.mocked(activeMediaModule.useActiveMedia).mockReturnValue({
      isPlaying: false,
      session: null,
      media: undefined,
      isLoadingMedia: false,
      elapsedSeconds: 0,
      formattedElapsed: "00:00",
      setActiveSession: vi.fn(),
    });
  });

  it("plays music on mount when app is focused and no game is playing", () => {
    render(<MediaDetails />);

    expect(playSpy).toHaveBeenCalled();
    expect(screen.getByText("Elden Ring OST")).toBeDefined();
    expect(screen.getByText("Main Theme")).toBeDefined();
  });

  it("stops music when playing a game", () => {
    // Game is active
    vi.mocked(activeMediaModule.useActiveMedia).mockReturnValue({
      isPlaying: true,
      session: {
        mediaId: "media-1",
        launchId: "l1",
        agentId: "a1",
        startedAt: Date.now(),
      },
      media: mediaWithAudio,
      isLoadingMedia: false,
      elapsedSeconds: 60,
      formattedElapsed: "01:00",
      setActiveSession: vi.fn(),
    });

    render(<MediaDetails />);

    expect(playSpy).not.toHaveBeenCalled();
    expect(pauseSpy).toHaveBeenCalled();
  });

  it("stops music when the application is not focused", () => {
    // App is blurred / unfocused
    vi.mocked(appFocusModule.useAppFocus).mockReturnValue(false);

    render(<MediaDetails />);

    expect(playSpy).not.toHaveBeenCalled();
    expect(pauseSpy).toHaveBeenCalled();
  });

  it("resumes music when application regains focus and no game is playing", () => {
    const { rerender } = render(<MediaDetails />);

    expect(playSpy).toHaveBeenCalled();
    pauseSpy.mockClear();
    playSpy.mockClear();

    // App loses focus
    vi.mocked(appFocusModule.useAppFocus).mockReturnValue(false);
    rerender(<MediaDetails />);
    expect(pauseSpy).toHaveBeenCalled();

    pauseSpy.mockClear();

    // App regains focus
    vi.mocked(appFocusModule.useAppFocus).mockReturnValue(true);
    rerender(<MediaDetails />);
    expect(playSpy).toHaveBeenCalled();
  });

  it("resumes music when a running game stops and app is focused", () => {
    // Initially game is running
    vi.mocked(activeMediaModule.useActiveMedia).mockReturnValue({
      isPlaying: true,
      session: {
        mediaId: "media-1",
        launchId: "l1",
        agentId: "a1",
        startedAt: Date.now(),
      },
      media: mediaWithAudio,
      isLoadingMedia: false,
      elapsedSeconds: 60,
      formattedElapsed: "01:00",
      setActiveSession: vi.fn(),
    });

    const { rerender } = render(<MediaDetails />);
    expect(playSpy).not.toHaveBeenCalled();
    expect(pauseSpy).toHaveBeenCalled();

    pauseSpy.mockClear();
    playSpy.mockClear();

    // Game ends
    vi.mocked(activeMediaModule.useActiveMedia).mockReturnValue({
      isPlaying: false,
      session: null,
      media: undefined,
      isLoadingMedia: false,
      elapsedSeconds: 0,
      formattedElapsed: "00:00",
      setActiveSession: vi.fn(),
    });

    rerender(<MediaDetails />);
    expect(playSpy).toHaveBeenCalled();
  });
});
