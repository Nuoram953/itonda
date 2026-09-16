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
      category: "general",
      playtime_minutes: 180,
      created_at: "2026-09-16T12:00:00Z",
      updated_at: "2026-09-16T12:00:00Z",
    },
    {
      id: "thought-2",
      media_id: "media-1",
      title: "Boss fight mechanics",
      content: "Tight cover system and satisfying weapon feedback.",
      category: "general",
      playtime_minutes: 240,
      created_at: "2026-09-16T13:00:00Z",
      updated_at: "2026-09-16T13:00:00Z",
    },
  ];

  it("renders empty state without duplicate header Add Thought button when no thoughts exist", () => {
    render(
      <SessionThoughtsSection
        mediaId="media-1"
        thoughts={[]}
        onCreateThought={onCreateThought}
        onUpdateThought={onUpdateThought}
        onDeleteThought={onDeleteThought}
      />,
    );

    expect(screen.getByText("No session thoughts yet")).toBeDefined();
    expect(screen.getByText("Add Your First Thought")).toBeDefined();
    expect(screen.queryByText("Add Thought")).toBeNull();
  });

  it("renders thoughts cards with title, content, playtime, and header Add Thought button", () => {
    render(
      <SessionThoughtsSection
        mediaId="media-1"
        thoughts={mockThoughts}
        onCreateThought={onCreateThought}
        onUpdateThought={onUpdateThought}
        onDeleteThought={onDeleteThought}
      />,
    );

    expect(screen.getByText("Add Thought")).toBeDefined();
    expect(screen.getByText("Music in Gears")).toBeDefined();
    expect(
      screen.getByText("The soundtrack during combat really surprised me."),
    ).toBeDefined();
    expect(screen.getByText("Boss fight mechanics")).toBeDefined();
    expect(
      screen.getByText("Tight cover system and satisfying weapon feedback."),
    ).toBeDefined();
  });

  it("opens thought dialog when Add Thought button is clicked", () => {
    render(
      <SessionThoughtsSection
        mediaId="media-1"
        thoughts={mockThoughts}
        onCreateThought={onCreateThought}
        onUpdateThought={onUpdateThought}
        onDeleteThought={onDeleteThought}
      />,
    );

    fireEvent.click(screen.getByText("Add Thought"));
    expect(screen.getByText("Add Session Thought")).toBeDefined();
    expect(
      screen.getByPlaceholderText(
        "e.g. Music during Act 2 combat, Boss fight phase 3...",
      ),
    ).toBeDefined();
  });
});
