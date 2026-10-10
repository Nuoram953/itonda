import { describe, it, expect, vi } from "vitest";
import {
  fireEvent,
  render,
  screen,
  createMedia,
  createAsset,
  createMediaDetails,
} from "@/test/test-utils";
import { OverviewTab } from "../OverviewTab";

describe("OverviewTab Component", () => {
  const mockMedia = createMedia({
    id: "media-1",
    title: "Kingdom Come: Deliverance",
    media_type: "game",
    status: "in_progress",
    summary: "A story-driven open-world RPG set in medieval Bohemia.",
    description: "Detailed description of the game mechanics and world.",
    genres: ["RPG", "Adventure"],
    tags: ["Open World", "Medieval"],
    assets: [
      createAsset({ id: "s1", asset_type: "screenshot" }),
      createAsset({ id: "s2", asset_type: "screenshot" }),
      createAsset({ id: "t1", asset_type: "trailer" }),
    ],
    details: createMediaDetails({
      developers: ["Warhorse Studios"],
      publishers: ["Deep Silver"],
      series: null,
      playtime_minutes: 120,
      last_played_at: 1700000000,
    }),
  });

  it("renders featured media, screenshot carousel, game length, and about sections without top title", () => {
    render(<OverviewTab media={mockMedia} onNavigateTab={vi.fn()} />);

    // Top title should be removed
    expect(
      screen.queryByText("About Kingdom Come: Deliverance"),
    ).toBeNull();
    // Trailer & screenshots title above stage should be removed
    expect(screen.queryByText("Trailers & Screenshots")).toBeNull();

    // Featured media stage should exist with video
    const featuredStage = screen.getByTestId("featured-media");
    expect(featuredStage).toBeDefined();
    const video = featuredStage.querySelector("video");
    expect(video).not.toBeNull();
    expect(video?.getAttribute("src")).toContain("t1");

    // Screenshot carousel
    expect(screen.getByText("Screenshots")).toBeDefined();
    expect(screen.getByTestId("screenshot-track")).toBeDefined();

    // Game length (HowLongToBeat) section
    expect(screen.getByText("Game Length")).toBeDefined();
    expect(screen.getByText("HowLongToBeat")).toBeDefined();
    expect(screen.getByText("Main Story")).toBeDefined();
    expect(screen.getByText("Main + Extra")).toBeDefined();
    expect(screen.getByText("Completionist")).toBeDefined();

    // About section & Genres & Tags
    expect(screen.getByText("About This Game")).toBeDefined();
    expect(screen.getByText(mockMedia.summary!)).toBeDefined();
    expect(screen.getByText("Genres & Tags")).toBeDefined();
    expect(screen.getByText("RPG")).toBeDefined();
    expect(screen.getByText("Open World")).toBeDefined();
    expect(screen.getByText("Warhorse Studios")).toBeDefined();
    expect(screen.getByText("Deep Silver")).toBeDefined();
  });

  it("calls onNavigateTab with 'gallery' when 'View all' is clicked in screenshots carousel", () => {
    const handleNavigateTab = vi.fn();
    render(<OverviewTab media={mockMedia} onNavigateTab={handleNavigateTab} />);

    fireEvent.click(screen.getByRole("button", { name: /View all/i }));

    expect(handleNavigateTab).toHaveBeenCalledWith("gallery");
  });

  it("falls back to screenshot in featured media if no trailer exists", () => {
    const mediaWithoutTrailer = createMedia({
      title: "No Trailer Game",
      assets: [createAsset({ id: "s1", asset_type: "screenshot" })],
    });

    render(
      <OverviewTab media={mediaWithoutTrailer} onNavigateTab={vi.fn()} />,
    );

    const featuredStage = screen.getByTestId("featured-media");
    expect(featuredStage.querySelector("video")).toBeNull();
    const img = featuredStage.querySelector("img");
    expect(img).not.toBeNull();
    expect(img?.getAttribute("src")).toContain("s1");
  });

  it("hides screenshots carousel and genres card when empty, and shows empty state for media", () => {
    render(
      <OverviewTab
        media={createMedia({ title: "Empty Movie", media_type: "movie" })}
        onNavigateTab={vi.fn()}
      />,
    );

    expect(
      screen.getByText("No trailer or screenshots yet"),
    ).toBeDefined();
    expect(screen.queryByText("Screenshots")).toBeNull();
    expect(screen.queryByText("Genres & Tags")).toBeNull();
    expect(screen.queryByText("Game Length")).toBeNull();
    expect(
      screen.getByText("No description available for this game yet."),
    ).toBeDefined();
  });
});
