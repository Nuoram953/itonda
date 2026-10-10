import { describe, it, expect } from "vitest";
import { render, screen } from "@/test/test-utils";
import { GameLengthSection } from "../GameLengthSection";

describe("GameLengthSection Component", () => {
  it("renders 4 metric categories with formatted hours", () => {
    render(<GameLengthSection />);

    expect(screen.getByTestId("game-length-section")).toBeDefined();
    expect(screen.getByText("Game Length")).toBeDefined();
    expect(
      screen.getByText("Average completion estimates from the community."),
    ).toBeDefined();

    expect(screen.getByText("Main Story")).toBeDefined();
    expect(screen.getByText("25 Hours")).toBeDefined();
    expect(screen.getByText("Core objectives & story")).toBeDefined();

    expect(screen.getByText("Main + Extra")).toBeDefined();
    expect(screen.getByText("33 Hours")).toBeDefined();
    expect(screen.getByText("Story plus side quests")).toBeDefined();

    expect(screen.getByText("Completionist")).toBeDefined();
    expect(screen.getByText("65 Hours")).toBeDefined();
    expect(screen.getByText("100% completion & all extras")).toBeDefined();

    expect(screen.getByText("All Styles")).toBeDefined();
    expect(screen.getByText("99 Hours")).toBeDefined();
    expect(screen.getByText("Average of all playstyles")).toBeDefined();
  });
});
