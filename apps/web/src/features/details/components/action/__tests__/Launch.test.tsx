import { describe, it, expect, vi, beforeEach } from "vitest";
import { fireEvent, render, screen } from "@/test/test-utils";
import { Launch } from "../Launch";
import type { components } from "@/api/generated.d";
import { useActiveMedia } from "@/hooks/use-active-media";

const mockMutate = vi.fn();

vi.mock("../../../api/post-media-launch", () => ({
  useLaunchMedia: () => ({
    mutate: mockMutate,
  }),
}));

vi.mock("@/hooks/use-active-media", () => ({
  useActiveMedia: vi.fn(() => ({
    session: null,
    isPlaying: false,
    formattedElapsed: "00:00",
  })),
}));

describe("Launch Component", () => {
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

  const singleProfile: components["schemas"]["Launch"][] = [
    { id: "profile-1", name: "Default Profile" },
  ];

  const multipleProfiles: components["schemas"]["Launch"][] = [
    { id: "profile-dx11", name: "DirectX 11" },
    { id: "profile-vulkan", name: "Vulkan" },
  ];

  it("renders the Play action button", () => {
    render(<Launch profiles={singleProfile} />);

    expect(screen.getByRole("button", { name: "Play" })).toBeDefined();
  });

  it("does nothing when Play is clicked with no profiles", () => {
    render(<Launch profiles={[]} />);

    const playButton = screen.getByRole("button", { name: "Play" });
    fireEvent.click(playButton);

    expect(mockMutate).not.toHaveBeenCalled();
    expect(screen.queryByText("Select launch profile")).toBeNull();
  });

  it("launches single profile immediately when Play is clicked", () => {
    render(<Launch profiles={singleProfile} />);

    const playButton = screen.getByRole("button", { name: "Play" });
    fireEvent.click(playButton);

    expect(mockMutate).toHaveBeenCalledOnce();
    expect(mockMutate).toHaveBeenCalledWith("profile-1", expect.any(Object));
    expect(screen.queryByText("Select launch profile")).toBeNull();
  });

  it("opens modal dialog with profile choices when multiple profiles exist", () => {
    render(<Launch profiles={multipleProfiles} />);

    const playButton = screen.getByRole("button", { name: "Play" });
    fireEvent.click(playButton);

    expect(screen.getByText("Select launch profile")).toBeDefined();
    expect(
      screen.getByText("Choose how you want to launch this game."),
    ).toBeDefined();
    expect(screen.getByRole("button", { name: "DirectX 11" })).toBeDefined();
    expect(screen.getByRole("button", { name: "Vulkan" })).toBeDefined();
    expect(mockMutate).not.toHaveBeenCalled();
  });

  it("launches selected profile from modal dialog and closes dialog on success", () => {
    mockMutate.mockImplementation(
      (_id: string, options?: { onSuccess?: () => void }) => {
        options?.onSuccess?.();
      },
    );

    render(<Launch profiles={multipleProfiles} />);

    const playButton = screen.getByRole("button", { name: "Play" });
    fireEvent.click(playButton);

    const vulkanButton = screen.getByRole("button", { name: "Vulkan" });
    fireEvent.click(vulkanButton);

    expect(mockMutate).toHaveBeenCalledWith("profile-vulkan", expect.any(Object));
    expect(screen.queryByText("Select launch profile")).toBeNull();
  });

  it("closes modal dialog when Cancel is clicked", () => {
    render(<Launch profiles={multipleProfiles} />);

    const playButton = screen.getByRole("button", { name: "Play" });
    fireEvent.click(playButton);

    expect(screen.getByText("Select launch profile")).toBeDefined();

    const cancelButton = screen.getByRole("button", { name: "Cancel" });
    fireEvent.click(cancelButton);

    expect(screen.queryByText("Select launch profile")).toBeNull();
    expect(mockMutate).not.toHaveBeenCalled();
  });

  it("renders active playing state when game is running", () => {
    vi.mocked(useActiveMedia).mockReturnValue({
      session: {
        mediaId: "game-99",
        launchId: "profile-1",
        agentId: "agent-1",
        startedAt: Date.now() - 30000,
      },
      isPlaying: true,
      formattedElapsed: "00:30",
      media: undefined,
      isLoadingMedia: false,
      elapsedSeconds: 30,
      setActiveSession: vi.fn(),
    });

    render(<Launch profiles={singleProfile} mediaId="game-99" />);

    expect(screen.getByRole("button", { name: "Now Playing" })).toBeDefined();
    expect(screen.getByText("Playing (00:30)")).toBeDefined();
  });
});
