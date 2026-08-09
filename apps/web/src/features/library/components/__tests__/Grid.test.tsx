import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@/test/test-utils";
import { MediaGrid } from "../Grid";
import { LibraryProvider } from "../../store/LibraryContext";
import type { components } from "@/api/generated.d";

type Media = components["schemas"]["Media"];

const mockMediaItems: Media[] = [
  {
    id: "media-1",
    title: "Elden Ring",
    media_type: "game",
    status: "in_progress",
    assets: [],
    launches: [],
  },
  {
    id: "media-2",
    title: "Cyberpunk 2077",
    media_type: "game",
    status: "completed",
    assets: [],
    launches: [],
  },
];

let mockMediaData: { items: Media[]; total: number } = {
  items: mockMediaItems,
  total: mockMediaItems.length,
};

vi.mock("../../api/get-media", () => ({
  useMedia: () => ({
    data: mockMediaData,
    isLoading: false,
  }),
  useInfiniteMedia: () => ({
    data: {
      pages: [
        {
          items: mockMediaData.items,
          total: mockMediaData.total,
          page: 1,
          limit: 24,
          total_pages: 1,
          has_next: false,
        },
      ],
      pageParams: [1],
    },
    isLoading: false,
    hasNextPage: false,
    isFetchingNextPage: false,
    fetchNextPage: vi.fn(),
  }),
}));

vi.mock("../../api/post-media-refresh", () => ({
  useRefreshMedia: () => ({
    mutate: vi.fn(),
    isPending: false,
  }),
}));

vi.mock("@tanstack/react-router", () => ({
  useSearch: () => ({ type: undefined }),
  Link: ({
    children,
    to,
    params,
  }: {
    children: React.ReactNode;
    to: string;
    params?: { mediaId?: string };
  }) => {
    const href = to.replace("$mediaId", params?.mediaId ?? "");
    return <a href={href}>{children}</a>;
  },
}));

function renderMediaGrid() {
  return render(
    <LibraryProvider>
      <MediaGrid />
    </LibraryProvider>,
  );
}

describe("MediaGrid Component", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockMediaData = { items: mockMediaItems, total: mockMediaItems.length };
  });

  it("renders workspace header with title and item count", () => {
    renderMediaGrid();

    expect(screen.getByText("Media")).toBeDefined();
    expect(screen.getByText("2 items")).toBeDefined();
  });

  it("renders card for each media item", () => {
    renderMediaGrid();

    expect(screen.getByText("Elden Ring")).toBeDefined();
    expect(screen.getByText("Cyberpunk 2077")).toBeDefined();
    expect(screen.getAllByRole("article").length).toBe(2);
  });

  it("renders header action buttons for Search and Refresh", () => {
    renderMediaGrid();

    expect(screen.getByRole("button", { name: "Search" })).toBeDefined();
    expect(screen.getByRole("button", { name: "Refresh" })).toBeDefined();
  });

  it("renders 0 items subtitle and no cards when library is empty", () => {
    mockMediaData = { items: [], total: 0 };

    renderMediaGrid();

    expect(screen.getByText("Media")).toBeDefined();
    expect(screen.getByText("0 items")).toBeDefined();
    expect(screen.queryAllByRole("article").length).toBe(0);
  });

  it("renders link elements leading to media details", () => {
    renderMediaGrid();

    const links = screen.getAllByRole("link");
    const hrefs = links.map((link) => link.getAttribute("href"));
    expect(hrefs).toContain("/media/media-1");
    expect(hrefs).toContain("/media/media-2");
  });
});
