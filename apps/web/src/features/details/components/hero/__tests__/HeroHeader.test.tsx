import { describe, it, expect } from "vitest";
import {
  render,
  screen,
  createMedia,
  createAsset,
  createMediaDetails,
} from "@/test/test-utils";
import { HeroHeader } from "../HeroHeader";

describe("HeroHeader Component", () => {
  const mockMedia = createMedia({
    id: "media-1",
    title: "Kingdom Come: Deliverance",
    status: "in_progress",
    assets: [
      createAsset({ id: "poster-1", asset_type: "poster" }),
      createAsset({ id: "banner-1", asset_type: "banner" }),
      createAsset({ id: "trailer-1", asset_type: "trailer" }),
    ],
    details: createMediaDetails({
      playtime_minutes: 180,
      last_played_at: 1700000000,
    }),
  });


  it("renders media title, playtime, and last played metadata", () => {
    render(<HeroHeader media={mockMedia} />);

    expect(screen.getByText("Kingdom Come: Deliverance")).toBeDefined();
    expect(screen.getByText("3h")).toBeDefined();
    expect(screen.getByText("Playtime")).toBeDefined();
    expect(screen.getByText("Last Played")).toBeDefined();
  });

  it("does not render trailer video or trailer controls in hero header", () => {
    render(<HeroHeader media={mockMedia} />);

    expect(screen.queryByTitle("Pause Trailer")).toBeNull();
    expect(screen.queryByTitle("Play Trailer")).toBeNull();
    expect(screen.queryByText("Trailer")).toBeNull();
  });

  it("renders genres, developers, release year, and tags from media without hardcoded values", () => {
    const mediaWithMetadata = createMedia({
      title: "Kingdom Come: Deliverance",
      genres: ["RPG", "Adventure"],
      tags: ["Historical", "Realistic"],
      release_date: 1518480000, // 2018-02-13 UTC
      details: createMediaDetails({
        developers: ["Warhorse Studios"],
      }),
    });

    render(<HeroHeader media={mediaWithMetadata} />);

    expect(
      screen.getByText("RPG · Adventure · Warhorse Studios · 2018"),
    ).toBeDefined();
    expect(screen.getByText("#Historical")).toBeDefined();
    expect(screen.getByText("#Realistic")).toBeDefined();

    expect(screen.queryByText(/FromSoftware/)).toBeNull();
    expect(screen.queryByText("#Souls-like")).toBeNull();
    expect(screen.queryByText("#Masterpiece")).toBeNull();
  });

  it("omits metadata and tags sections when media has none", () => {
    const emptyMedia = createMedia({
      title: "Empty Game",
      genres: [],
      tags: [],
      release_date: null,
      details: null,
    });

    render(<HeroHeader media={emptyMedia} />);

    expect(screen.getByRole("heading", { name: "Empty Game" })).toBeDefined();
    expect(screen.queryByText(/FromSoftware/)).toBeNull();
    expect(screen.queryByText("#Souls-like")).toBeNull();
  });
});
