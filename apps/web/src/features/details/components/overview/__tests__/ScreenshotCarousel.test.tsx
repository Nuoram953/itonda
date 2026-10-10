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
});
