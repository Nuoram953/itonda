import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@/test/test-utils";
import { GameVerdictCard } from "../GameVerdictCard";
import type { GameReview } from "../../../types/reviews";

describe("GameVerdictCard Component", () => {
  const onSaveVerdict = vi.fn().mockResolvedValue(undefined);
  const onDeleteVerdict = vi.fn().mockResolvedValue(undefined);

  it("renders empty callout when review is null", () => {
    render(
      <GameVerdictCard
        review={null}
        onSaveVerdict={onSaveVerdict}
        onDeleteVerdict={onDeleteVerdict}
      />,
    );

    expect(screen.getByText("Overall Game Verdict & Score")).toBeDefined();
    expect(screen.getByText("+ Add Verdict & Review")).toBeDefined();
  });

  it("renders review verdict and summary when review exists", () => {
    const mockReview: GameReview = {
      media_id: "media-1",
      verdict: "masterpiece",
      summary: "One of the greatest tactical shooters of all time.",
      created_at: "2026-09-16T12:00:00Z",
      updated_at: "2026-09-16T12:00:00Z",
    };

    render(
      <GameVerdictCard
        review={mockReview}
        onSaveVerdict={onSaveVerdict}
        onDeleteVerdict={onDeleteVerdict}
      />,
    );

    expect(screen.getByText("Masterpiece")).toBeDefined();
    expect(
      screen.getByText(
        `"One of the greatest tactical shooters of all time."`,
      ),
    ).toBeDefined();
    expect(screen.getByText("Edit")).toBeDefined();
    expect(screen.getByText("Remove")).toBeDefined();
  });

  it("opens verdict dialog on click", () => {
    render(
      <GameVerdictCard
        review={null}
        onSaveVerdict={onSaveVerdict}
        onDeleteVerdict={onDeleteVerdict}
      />,
    );

    fireEvent.click(screen.getByText("+ Add Verdict & Review"));
    expect(screen.getByText("Game Verdict & Overall Review")).toBeDefined();
    expect(screen.getByText("Select Verdict")).toBeDefined();
  });
});
