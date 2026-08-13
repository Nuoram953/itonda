import { describe, it, expect } from "vitest";
import { render, screen } from "@/test/test-utils";
import { HeroHeader } from "../HeroHeader";
import type { components } from "@/api/generated.d";

type Media = components["schemas"]["Media"];

describe("HeroHeader Component", () => {
  const mockMedia: Media = {
    id: "media-1",
    title: "Kingdom Come: Deliverance",
    media_type: "game",
    status: "in_progress",
    assets: [
      { id: "poster-1", asset_type: "poster" },
      { id: "banner-1", asset_type: "banner" },
      { id: "trailer-1", asset_type: "trailer" },
    ],
    launches: [],
    details: {
      playtime_minutes: 180,
      last_played_at: 1700000000,
    },
  };

  it("renders media title, playtime, and last played metadata", () => {
    render(<HeroHeader media={mockMedia} />);

    expect(screen.getByText("Kingdom Come: Deliverance")).toBeDefined();
    expect(screen.getByText("3 Hours")).toBeDefined();
    expect(screen.getByText("Playtime")).toBeDefined();
    expect(screen.getByText("Last Played")).toBeDefined();
    expect(screen.getByText("Trailer")).toBeDefined();
  });

  it("renders trailer controls when trailer asset is present", () => {
    render(<HeroHeader media={mockMedia} />);

    expect(screen.getByTitle("Pause Trailer")).toBeDefined();
    expect(screen.getByTitle("Unmute Audio")).toBeDefined();
  });
});
