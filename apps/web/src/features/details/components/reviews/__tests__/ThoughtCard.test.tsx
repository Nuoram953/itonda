import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@/test/test-utils";
import { ThoughtCard } from "../ThoughtCard";
import type { GameThought } from "../../../types/reviews";

describe("ThoughtCard Component", () => {
  const onEdit = vi.fn();
  const onDelete = vi.fn().mockResolvedValue(undefined);

  const mockThought: GameThought = {
    id: "thought-1",
    media_id: "media-1",
    title: "Awesome combat mechanics",
    content: "The gunplay and movement feels very responsive.",
    category: "general",
    playtime_minutes: 150,
    created_at: "2026-09-16T12:00:00Z",
    updated_at: "2026-09-16T12:00:00Z",
  };

  it("renders thought title, content, playtime, and formatted date", () => {
    render(
      <ThoughtCard thought={mockThought} onEdit={onEdit} onDelete={onDelete} />,
    );

    expect(screen.getByText("Awesome combat mechanics")).toBeDefined();
    expect(
      screen.getByText("The gunplay and movement feels very responsive."),
    ).toBeDefined();
    expect(screen.getByText("2h 30m")).toBeDefined();
    expect(screen.getByText("Sep 16, 2026")).toBeDefined();
  });

  it("calls onEdit when edit button is clicked", () => {
    render(
      <ThoughtCard thought={mockThought} onEdit={onEdit} onDelete={onDelete} />,
    );

    fireEvent.click(screen.getByTitle("Edit thought"));
    expect(onEdit).toHaveBeenCalledWith(mockThought);
  });

  it("opens ConfirmDialog when delete button is clicked and triggers onDelete on confirm", async () => {
    render(
      <ThoughtCard thought={mockThought} onEdit={onEdit} onDelete={onDelete} />,
    );

    fireEvent.click(screen.getByTitle("Delete thought"));

    expect(screen.getByText("Delete Session Note")).toBeDefined();
    expect(
      screen.getByText(
        'Are you sure you want to delete "Awesome combat mechanics"? This note will be permanently removed.',
      ),
    ).toBeDefined();

    fireEvent.click(screen.getByText("Delete Note"));
    expect(onDelete).toHaveBeenCalledWith("thought-1");
  });

  it("does not call onDelete when cancel button is clicked", () => {
    onDelete.mockClear();

    render(
      <ThoughtCard thought={mockThought} onEdit={onEdit} onDelete={onDelete} />,
    );

    fireEvent.click(screen.getByTitle("Delete thought"));
    expect(screen.getByText("Delete Session Note")).toBeDefined();

    fireEvent.click(screen.getByText("Cancel"));
    expect(onDelete).not.toHaveBeenCalled();
  });
});
