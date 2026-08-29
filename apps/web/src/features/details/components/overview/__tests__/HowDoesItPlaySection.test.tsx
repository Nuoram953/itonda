import { describe, it, expect } from "vitest";
import {
  render,
  screen,
  createMedia,
  createAsset,
  createMediaDetails,
  createGameplayPillar,
} from "@/test/test-utils";
import { HowDoesItPlaySection } from "../HowDoesItPlaySection";

describe("HowDoesItPlaySection Component", () => {
  it("renders nothing when media details has no pillars", () => {
    const media = createMedia({
      details: createMediaDetails({ pillars: [] }),
    });

    render(<HowDoesItPlaySection media={media} />);
    expect(screen.queryByText("HOW DOES IT PLAY?")).toBeNull();
  });

  it("renders nothing when media details is null", () => {
    const media = createMedia({ details: null });

    render(<HowDoesItPlaySection media={media} />);
    expect(screen.queryByText("HOW DOES IT PLAY?")).toBeNull();
  });


  it("renders gameplay pillars dynamically with icons and descriptions", () => {
    const media = createMedia({
      assets: [
        createAsset({ id: "screenshot-1", asset_type: "screenshot" }),
      ],
      details: createMediaDetails({
        pillars: [
          createGameplayPillar({
            id: "combat-1",
            title: "Cover Combat & Active Reload",
            description: "Master active reload and squad maneuvers.",
            icon: "combat",
          }),
          createGameplayPillar({
            id: "survival-2",
            title: "Crimson Omen & Cover",
            description: "Take cover to regenerate health and bleed-out.",
            icon: "survival",
          }),
        ],
      }),
    });

    render(<HowDoesItPlaySection media={media} />);

    expect(screen.getByText("HOW DOES IT PLAY?")).toBeDefined();
    expect(screen.getByText("Cover Combat & Active Reload")).toBeDefined();
    expect(screen.getByText("Master active reload and squad maneuvers.")).toBeDefined();
    expect(screen.getByText("Crimson Omen & Cover")).toBeDefined();
    expect(screen.getByText("Take cover to regenerate health and bleed-out.")).toBeDefined();
  });
});
