import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@/test/test-utils";
import { SessionThoughtsSection } from "../SessionThoughtsSection";
import type { GameThought } from "../../../types/reviews";

describe("SessionThoughtsSection Component", () => {
  const onCreateThought = vi.fn().mockResolvedValue(undefined);
  const onUpdateThought = vi.fn().mockResolvedValue(undefined);
  const onDeleteThought = vi.fn().mockResolvedValue(undefined);

  const mockThoughts: GameThought[] = [
    {
      id: "thought-1",
      media_id: "media-1",
      title: "Music in Gears",
      content: "The soundtrack during combat really surprised me.",
      category: "audio",
      playtime_minutes: 180,
      created_at: "2026-09-16T12:00:00Z",
      updated_at: "2026-09-16T12:00:00Z",
    },
    {
      id: "thought-2",
      media_id: "media-1",
      title: "Boss fight mechanics",
      content: "Tight cover system and satisfying weapon feedback.",
      category: "gameplay",
      playtime_minutes: 240,
      created_at: "2026-09-16T13:00:00Z",
      updated_at: "2026-09-16T13:00:00Z",
    },
  ];

  it("renders empty state when no thoughts exist", () => {
    render(
      <SessionThoughtsSection
        thoughts={[]}
        onCreateThought={onCreateThought}
        onUpdateThought={onUpdateThought}
        onDeleteThought={onDeleteThought}
      />,
    );

    expect(screen.getByText("No session thoughts yet")).toBeDefined();
    expect(screen.getByText("Add Your First Thought")).toBeDefined();
  });

  it("renders thoughts cards with category badges and playtime", () => {
    render(
      <SessionThoughtsSection
        thoughts={mockThoughts}
        onCreateThought={onCreateThought}
        onUpdateThought={onUpdateThought}
        onDeleteThought={onDeleteThought}
      />,
    );

    expect(screen.getByText("Music in Gears")).toBeDefined();
    expect(screen.getByText("Boss fight mechanics")).toBeDefined();
    expect(screen.getAllByText("Music & Audio").length).toBeGreaterThanOrEqual(1);
    expect(screen.getAllByText("Gameplay").length).toBeGreaterThanOrEqual(1);
    expect(screen.getByText("3.0h")).toBeDefined();
    expect(screen.getByText("4.0h")).toBeDefined();
  });

  it("filters thoughts by text search query", () => {
    render(
      <SessionThoughtsSection
        thoughts={mockThoughts}
        onCreateThought={onCreateThought}
        onUpdateThought={onUpdateThought}
        onDeleteThought={onDeleteThought}
      />,
    );

    const searchInput = screen.getByPlaceholderText(
      "Search thoughts (e.g. music, combat, boss)...",
    );
    fireEvent.change(searchInput, { target: { value: "soundtrack" } });

    expect(screen.getByText("Music in Gears")).toBeDefined();
    expect(screen.queryByText("Boss fight mechanics")).toBeNull();
  });

  it("filters thoughts by category filter pill", () => {
    render(
      <SessionThoughtsSection
        thoughts={mockThoughts}
        onCreateThought={onCreateThought}
        onUpdateThought={onUpdateThought}
        onDeleteThought={onDeleteThought}
      />,
    );

    const gameplayButtons = screen.getAllByText("Gameplay");
    fireEvent.click(gameplayButtons[0]);

    expect(screen.getByText("Boss fight mechanics")).toBeDefined();
    expect(screen.queryByText("Music in Gears")).toBeNull();
  });
});
