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
});
