import { describe, it, expect, vi } from "vitest";
import { fireEvent, render, screen } from "@/test/test-utils";
import { OverviewTab } from "../OverviewTab";
import type { components } from "@/api/generated.d";

type Media = components["schemas"]["Media"];

describe("OverviewTab Component", () => {
  const mockMedia: Media = {
    id: "media-1",
    title: "Kingdom Come: Deliverance",
    media_type: "game",
    status: "in_progress",
    assets: [
      { id: "s1", asset_type: "screenshot" },
      { id: "t1", asset_type: "trailer" },
    ],
    launches: [],
    details: {
      playtime_minutes: 120,
      last_played_at: 1700000000,
    },
  };

  it("renders all overview sections", () => {
    const handleNavigateTab = vi.fn();
    render(<OverviewTab media={mockMedia} onNavigateTab={handleNavigateTab} />);

    expect(screen.getByText("WHAT IS THIS GAME?")).toBeDefined();
    expect(screen.getByText("HOW DOES IT PLAY?")).toBeDefined();
    expect(screen.getByText("SEE IT IN ACTION")).toBeDefined();
  });

  it("calls onNavigateTab with 'gallery' when 'View all videos' is clicked", () => {
    const handleNavigateTab = vi.fn();
    render(<OverviewTab media={mockMedia} onNavigateTab={handleNavigateTab} />);

    const viewAllBtn = screen.getByRole("button", { name: /View all videos/i });
    fireEvent.click(viewAllBtn);

    expect(handleNavigateTab).toHaveBeenCalledWith("gallery");
  });
});
