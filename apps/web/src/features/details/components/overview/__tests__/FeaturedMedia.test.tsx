import { describe, it, expect } from "vitest";
import { render, screen, createMedia, createAsset } from "@/test/test-utils";
import { FeaturedMedia } from "../FeaturedMedia";

describe("FeaturedMedia Component", () => {
  it("renders a video with controls and poster when trailer is available", () => {
    const media = createMedia({
      assets: [
        createAsset({ id: "t1", asset_type: "trailer" }),
        createAsset({ id: "s1", asset_type: "screenshot" }),
      ],
    });

    render(<FeaturedMedia media={media} />);

    const video = screen.getByTestId("featured-media").querySelector("video");
    expect(video).not.toBeNull();
    expect(video?.getAttribute("src")).toContain("t1");
    expect(video?.getAttribute("poster")).toContain("s1");
    expect(video?.hasAttribute("controls")).toBe(true);
  });

  it("falls back to screenshot when no trailer is available", () => {
    const media = createMedia({
      assets: [createAsset({ id: "s1", asset_type: "screenshot" })],
    });

    render(<FeaturedMedia media={media} />);

    const img = screen.getByTestId("featured-media").querySelector("img");
    expect(img).not.toBeNull();
    expect(img?.getAttribute("src")).toContain("s1");
  });

  it("falls back to backdrop or banner when neither trailer nor screenshot is available", () => {
    const media = createMedia({
      assets: [createAsset({ id: "b1", asset_type: "backdrop" })],
    });

    render(<FeaturedMedia media={media} />);

    const img = screen.getByTestId("featured-media").querySelector("img");
    expect(img).not.toBeNull();
    expect(img?.getAttribute("src")).toContain("b1");
  });

  it("renders empty state when no visual asset exists", () => {
    const media = createMedia({
      assets: [createAsset({ id: "p1", asset_type: "poster" })],
    });

    render(<FeaturedMedia media={media} />);

    expect(screen.getByText("No trailer or screenshots yet")).toBeDefined();
    expect(screen.queryByTestId("featured-media")).toBeNull();
  });
});
