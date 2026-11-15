import { describe, it, expect, vi } from "vitest";
import { fireEvent, render, screen, createMedia, createAsset } from "@/test/test-utils";
import { ScreenshotCarousel } from "../ScreenshotCarousel";

describe("ScreenshotCarousel Component", () => {
  it("renders screenshots, count chip, and navigates to gallery on view all", () => {
    const media = createMedia({
      assets: [
        createAsset({ id: "s1", asset_type: "screenshot" }),
        createAsset({ id: "s2", asset_type: "screenshot" }),
      ],
    });
    const handleViewGallery = vi.fn();

    render(
      <ScreenshotCarousel
        media={media}
        onViewGallery={handleViewGallery}
      />,
    );

    expect(screen.getByText("Screenshots")).toBeDefined();
    expect(screen.getByText("2")).toBeDefined();

    const viewAllBtn = screen.getByRole("button", { name: /View all/i });
    fireEvent.click(viewAllBtn);
    expect(handleViewGallery).toHaveBeenCalledTimes(1);

    const prevBtn = screen.getByRole("button", { name: /Previous screenshots/i });
    const nextBtn = screen.getByRole("button", { name: /Next screenshots/i });
    expect(prevBtn).toBeDefined();
    expect(nextBtn).toBeDefined();
  });

  it("returns null when no screenshots are present", () => {
    const media = createMedia({
      assets: [createAsset({ id: "t1", asset_type: "trailer" })],
    });

    render(
      <ScreenshotCarousel
        media={media}
        onViewGallery={vi.fn()}
      />,
    );

    expect(screen.queryByText("Screenshots")).toBeNull();
    expect(screen.queryByTestId("screenshot-track")).toBeNull();
  });

  it("groups screenshots into pages of 8 (2 rows of 4 columns)", () => {
    const media = createMedia({
      title: "Test Game",
      assets: Array.from({ length: 10 }, (_, idx) =>
        createAsset({ id: `s-${idx + 1}`, asset_type: "screenshot" }),
      ),
    });

    render(<ScreenshotCarousel media={media} onViewGallery={vi.fn()} />);

    const pages = screen.getAllByTestId("screenshot-page");
    expect(pages).toHaveLength(2);
    expect(pages[0].className).toContain("lg:grid-cols-4");
    expect(pages[0].querySelectorAll("img")).toHaveLength(8);
    expect(pages[1].querySelectorAll("img")).toHaveLength(2);
    expect(screen.getByAltText("Test Game screenshot 10")).toBeDefined();
  });
});
