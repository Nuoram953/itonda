import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, createMedia } from "@/test/test-utils";
import { ReviewsTab } from "../ReviewsTab";
import { useReviewOverview } from "../../../api/get-review-overview";

vi.mock("../../../api/get-review-overview", () => ({
  useReviewOverview: vi.fn(),
}));

describe("ReviewsTab Component", () => {
  const mockMedia = createMedia({
    id: "media-1",
    title: "Kingdom Come: Deliverance",
  });

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders loading state when overview query is pending", () => {
    vi.mocked(useReviewOverview).mockReturnValue({
      data: undefined,
      isPending: true,
      isError: false,
    } as ReturnType<typeof useReviewOverview>);

    render(<ReviewsTab media={mockMedia} />);

    expect(
      screen.getByText("Loading game reviews and session notes..."),
    ).toBeDefined();
  });

  it("renders error state when overview query fails", () => {
    vi.mocked(useReviewOverview).mockReturnValue({
      data: undefined,
      isPending: false,
      isError: true,
    } as ReturnType<typeof useReviewOverview>);

    render(<ReviewsTab media={mockMedia} />);

    expect(screen.getByText("Unable to load reviews")).toBeDefined();
  });

  it("renders reviews header and sections when data is loaded", () => {
    vi.mocked(useReviewOverview).mockReturnValue({
      data: {
        review: {
          media_id: "media-1",
          verdict: "masterpiece",
          summary: "Brilliant medieval RPG.",
          created_at: "2026-09-16T12:00:00Z",
          updated_at: "2026-09-16T12:00:00Z",
        },
        thoughts: [
          {
            id: "thought-1",
            media_id: "media-1",
            title: "Combat Depth",
            content: "Directional swordsmanship feels deliberate and rewarding.",
            category: "gameplay",
            playtime_minutes: 120,
            created_at: "2026-09-16T12:00:00Z",
            updated_at: "2026-09-16T12:00:00Z",
          },
        ],
      },
      isPending: false,
      isError: false,
    } as ReturnType<typeof useReviewOverview>);

    render(<ReviewsTab media={mockMedia} />);

    expect(screen.getByText("Reviews & Session Notes")).toBeDefined();
    expect(
      screen.getByText(
        "Your overall recommendation score and session highlights for Kingdom Come: Deliverance.",
      ),
    ).toBeDefined();
    expect(screen.getByText("Masterpiece")).toBeDefined();
    expect(screen.getByText(`"Brilliant medieval RPG."`)).toBeDefined();
    expect(screen.getByText("Combat Depth")).toBeDefined();
  });
});
