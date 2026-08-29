import { describe, it, expect, vi, beforeEach } from "vitest";
import { fireEvent, render, screen, createMedia } from "@/test/test-utils";
import { MoreOptions } from "../MoreOptions";

const mockMutate = vi.fn();

vi.mock("../../../api/post-media-refresh", () => ({
  useRefreshSingleMedia: () => ({
    mutate: mockMutate,
    isPending: false,
  }),
}));

describe("MoreOptions Component", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  const media = createMedia({
    id: "game-123",
    title: "Elden Ring",
    storefronts: [
      {
        external_id: "1245620",
        storefront_id: "steam",
        last_played_at: null,
        playtime_minutes: null,
      },
    ],
  });

  it("renders the trigger button with title 'More options'", () => {
    render(<MoreOptions media={media} />);

    expect(screen.getByRole("button", { name: "More options" })).toBeDefined();
  });

  it("opens menu displaying Refresh button and Force toggle switch when trigger is clicked", () => {
    render(<MoreOptions media={media} />);

    const trigger = screen.getByRole("button", { name: "More options" });
    fireEvent.click(trigger);

    expect(screen.getByRole("button", { name: "Refresh" })).toBeDefined();
    expect(screen.getByRole("switch", { name: /force/i })).toBeDefined();
    expect(screen.getByText("Force")).toBeDefined();
    expect(screen.getByText("View on Steam")).toBeDefined();
    expect(screen.getByText("Copy Media ID")).toBeDefined();
  });

  it("calls refresh mutation with force: false by default when Refresh is clicked", () => {
    render(<MoreOptions media={media} />);

    const trigger = screen.getByRole("button", { name: "More options" });
    fireEvent.click(trigger);

    const refreshOption = screen.getByRole("button", { name: "Refresh" });
    fireEvent.click(refreshOption);

    expect(mockMutate).toHaveBeenCalledOnce();
    expect(mockMutate).toHaveBeenCalledWith({
      mediaId: "game-123",
      force: false,
    });
  });

  it("calls refresh mutation with force: true when Force switch is toggled on", () => {
    render(<MoreOptions media={media} />);

    const trigger = screen.getByRole("button", { name: "More options" });
    fireEvent.click(trigger);

    const forceSwitch = screen.getByRole("switch", { name: /force/i });
    fireEvent.click(forceSwitch);

    const forceRefreshOption = screen.getByRole("button", {
      name: "Force Refresh",
    });
    fireEvent.click(forceRefreshOption);

    expect(mockMutate).toHaveBeenCalledOnce();
    expect(mockMutate).toHaveBeenCalledWith({
      mediaId: "game-123",
      force: true,
    });
  });

  it("opens Steam store link when View on Steam is clicked", () => {
    const windowOpenSpy = vi.spyOn(window, "open").mockImplementation(() => null);

    render(<MoreOptions media={media} />);

    const trigger = screen.getByRole("button", { name: "More options" });
    fireEvent.click(trigger);

    const steamOption = screen.getByText("View on Steam");
    fireEvent.click(steamOption);

    expect(windowOpenSpy).toHaveBeenCalledWith(
      "https://store.steampowered.com/app/1245620",
      "_blank",
      "noopener,noreferrer",
    );
  });

  it("copies media ID to clipboard when Copy Media ID is clicked", async () => {
    const writeTextMock = vi.fn().mockResolvedValue(undefined);
    Object.assign(navigator, {
      clipboard: {
        writeText: writeTextMock,
      },
    });

    render(<MoreOptions media={media} />);

    const trigger = screen.getByRole("button", { name: "More options" });
    fireEvent.click(trigger);

    const copyOption = screen.getByText("Copy Media ID");
    fireEvent.click(copyOption);

    expect(writeTextMock).toHaveBeenCalledWith("game-123");
  });
});
