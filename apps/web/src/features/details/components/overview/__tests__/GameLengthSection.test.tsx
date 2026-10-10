import { describe, it, expect } from "vitest";
import { render, screen, createMedia, createMediaDetails } from "@/test/test-utils";
import { GameLengthSection } from "../GameLengthSection";

describe("GameLengthSection Component", () => {
  it("renders 4 HLTB metric categories with formatted hours", () => {
    const media = createMedia({
      title: "Custom RPG",
      media_type: "game",
    });

    render(
      <GameLengthSection
        media={media}
        customHltbData={{
          main_story: 30,
          main_extra: 60,
          completionist: 120,
          all_styles: 50,
        }}
      />,
    );

    expect(screen.getByText("Game Length")).toBeDefined();
    expect(screen.getByText("HowLongToBeat")).toBeDefined();

    expect(screen.getByText("Main Story")).toBeDefined();
    expect(screen.getByText("30 Hours")).toBeDefined();

    expect(screen.getByText("Main + Extra")).toBeDefined();
    expect(screen.getByText("60 Hours")).toBeDefined();

    expect(screen.getByText("Completionist")).toBeDefined();
    expect(screen.getByText("120 Hours")).toBeDefined();

    expect(screen.getByText("All Styles")).toBeDefined();
    expect(screen.getByText("50 Hours")).toBeDefined();
  });

  it("displays user playtime and percentage progress against main story", () => {
    const media = createMedia({
      title: "Custom RPG",
      media_type: "game",
      details: createMediaDetails({
        playtime_minutes: 900, // 15 hours
      }),
    });

    render(
      <GameLengthSection
        media={media}
        customHltbData={{
          main_story: 30,
          main_extra: 60,
          completionist: 120,
          all_styles: 50,
        }}
      />,
    );

    expect(screen.getByText("Your playtime:")).toBeDefined();
    expect(screen.getByText("15h")).toBeDefined();
    expect(screen.getByText("(50% of story)")).toBeDefined();
  });

  it("returns null when no HLTB data is available", () => {
    const media = createMedia({
      title: "Movie Title",
      media_type: "movie",
    });

    render(<GameLengthSection media={media} customHltbData={null} />);

    expect(screen.queryByTestId("game-length-section")).toBeNull();
    expect(screen.queryByText("Game Length")).toBeNull();
  });
});
