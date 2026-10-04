import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@/test/test-utils";
import { AddMediaDialog } from "../AddMediaDialog";
import * as searchMediaModule from "../../../api/search-media";
import * as postMediaModule from "../../../api/post-media";

vi.mock("../../../api/search-media", () => ({
  useSearchMedia: vi.fn(),
}));

vi.mock("../../../api/post-media", () => ({
  useCreateMedia: vi.fn(),
}));

describe("AddMediaDialog Component", () => {
  const mockMutateAsync = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(postMediaModule.useCreateMedia).mockReturnValue({
      mutateAsync: mockMutateAsync,
      isPending: false,
    } as unknown as ReturnType<typeof postMediaModule.useCreateMedia>);

    vi.mocked(searchMediaModule.useSearchMedia).mockReturnValue({
      data: undefined,
      isFetching: false,
    } as unknown as ReturnType<typeof searchMediaModule.useSearchMedia>);
  });

  it("renders media type options when open", () => {
    render(<AddMediaDialog open={true} onOpenChange={vi.fn()} />);

    expect(screen.getByText("Add Media")).toBeDefined();
    expect(screen.getByRole("button", { name: /game/i })).toBeDefined();
    expect(screen.getByRole("button", { name: /movie/i })).toBeDefined();
    expect(screen.getByRole("button", { name: /tv show/i })).toBeDefined();
    expect(screen.queryByPlaceholderText(/type a few letters/i)).toBeNull();
  });

  it("shows search input after selecting a media type", () => {
    render(<AddMediaDialog open={true} onOpenChange={vi.fn()} />);

    const gameButton = screen.getByRole("button", { name: /game/i });
    fireEvent.click(gameButton);

    expect(
      screen.getByPlaceholderText(/type a few letters to search for game/i),
    ).toBeDefined();
  });

  it("enables Add to Library button when title is typed", () => {
    render(<AddMediaDialog open={true} onOpenChange={vi.fn()} />);

    const gameButton = screen.getByRole("button", { name: /game/i });
    fireEvent.click(gameButton);

    const input = screen.getByPlaceholderText(/type a few letters to search for game/i);
    const addButton = screen.getByRole("button", { name: /add to library/i });

    expect(addButton).toHaveProperty("disabled", true);

    fireEvent.change(input, { target: { value: "Hades" } });

    expect(addButton).toHaveProperty("disabled", false);
  });

  it("submits media creation and closes dialog on success", async () => {
    const handleOpenChange = vi.fn();
    mockMutateAsync.mockResolvedValueOnce({
      id: "media-1",
      title: "Hades",
      media_type: "game",
    });

    render(<AddMediaDialog open={true} onOpenChange={handleOpenChange} />);

    fireEvent.click(screen.getByRole("button", { name: /game/i }));

    const input = screen.getByPlaceholderText(/type a few letters to search for game/i);
    fireEvent.change(input, { target: { value: "Hades" } });

    const addButton = screen.getByRole("button", { name: /add to library/i });
    fireEvent.click(addButton);

    await waitFor(() => {
      expect(mockMutateAsync).toHaveBeenCalledWith({
        title: "Hades",
        media_type: "game",
        external_id: null,
      });
      expect(handleOpenChange).toHaveBeenCalledWith(false);
    });
  });

  it("renders search results and selects item", async () => {
    vi.mocked(searchMediaModule.useSearchMedia).mockReturnValue({
      data: [
        {
          external_id: "1942",
          title: "The Witcher 3: Wild Hunt",
          media_type: "game",
          year: 2015,
          summary: "An open world RPG.",
          cover_url: "https://example.com/cover.jpg",
        },
      ],
      isFetching: false,
    } as unknown as ReturnType<typeof searchMediaModule.useSearchMedia>);

    render(<AddMediaDialog open={true} onOpenChange={vi.fn()} />);

    fireEvent.click(screen.getByRole("button", { name: /game/i }));

    const input = screen.getByPlaceholderText(/type a few letters to search for game/i);
    fireEvent.change(input, { target: { value: "Witcher" } });

    await waitFor(() => {
      expect(screen.getByText("The Witcher 3: Wild Hunt")).toBeDefined();
      expect(screen.getByText("2015")).toBeDefined();
    });

    // Click result item
    fireEvent.click(screen.getByText("The Witcher 3: Wild Hunt"));

    // Input should be updated with the selected item's title
    expect((input as HTMLInputElement).value).toBe("The Witcher 3: Wild Hunt");
    // Search result should remain in the document and visible
    expect(screen.getByText("The Witcher 3: Wild Hunt")).toBeDefined();

    // Submitting should now include the external_id
    mockMutateAsync.mockResolvedValueOnce({
      id: "media-2",
      title: "The Witcher 3: Wild Hunt",
      media_type: "game",
    });

    const addButton = screen.getByRole("button", { name: /add to library/i });
    fireEvent.click(addButton);

    await waitFor(() => {
      expect(mockMutateAsync).toHaveBeenCalledWith({
        title: "The Witcher 3: Wild Hunt",
        media_type: "game",
        external_id: "1942",
      });
    });
  });

  it("keeps search results visible and updates input field when selecting different items", async () => {
    vi.mocked(searchMediaModule.useSearchMedia).mockReturnValue({
      data: [
        {
          external_id: "1942",
          title: "The Witcher 3: Wild Hunt",
          media_type: "game",
          year: 2015,
          summary: "An open world RPG.",
          cover_url: "https://example.com/cover.jpg",
        },
        {
          external_id: "1943",
          title: "The Witcher 2: Assassins of Kings",
          media_type: "game",
          year: 2011,
          summary: "A fantasy RPG.",
          cover_url: null,
        },
      ],
      isFetching: false,
    } as unknown as ReturnType<typeof searchMediaModule.useSearchMedia>);

    render(<AddMediaDialog open={true} onOpenChange={vi.fn()} />);

    fireEvent.click(screen.getByRole("button", { name: /game/i }));

    const input = screen.getByPlaceholderText(/type a few letters to search for game/i);
    fireEvent.change(input, { target: { value: "Witcher" } });

    await waitFor(() => {
      expect(screen.getByText("The Witcher 3: Wild Hunt")).toBeDefined();
      expect(screen.getByText("The Witcher 2: Assassins of Kings")).toBeDefined();
    });

    // Select the first item
    fireEvent.click(screen.getByText("The Witcher 3: Wild Hunt"));
    expect((input as HTMLInputElement).value).toBe("The Witcher 3: Wild Hunt");
    expect(screen.getByText("The Witcher 3: Wild Hunt")).toBeDefined();
    expect(screen.getByText("The Witcher 2: Assassins of Kings")).toBeDefined();

    // Select the second item
    fireEvent.click(screen.getByText("The Witcher 2: Assassins of Kings"));
    expect((input as HTMLInputElement).value).toBe("The Witcher 2: Assassins of Kings");
    expect(screen.getByText("The Witcher 3: Wild Hunt")).toBeDefined();
    expect(screen.getByText("The Witcher 2: Assassins of Kings")).toBeDefined();

    // Deselect by clicking again
    fireEvent.click(screen.getByText("The Witcher 2: Assassins of Kings"));
    expect((input as HTMLInputElement).value).toBe("Witcher");
  });

  it("closes dialog when Cancel button is clicked", () => {
    const handleOpenChange = vi.fn();
    render(<AddMediaDialog open={true} onOpenChange={handleOpenChange} />);

    fireEvent.click(screen.getByRole("button", { name: /cancel/i }));
    expect(handleOpenChange).toHaveBeenCalledWith(false);
  });

  it("displays searching catalog indicator while debouncing query input", () => {
    render(<AddMediaDialog open={true} onOpenChange={vi.fn()} />);

    fireEvent.click(screen.getByRole("button", { name: /game/i }));

    const input = screen.getByPlaceholderText(/type a few letters to search for game/i);
    fireEvent.change(input, { target: { value: "Ze" } });

    expect(screen.getByText(/searching catalog\.\.\./i)).toBeDefined();
  });

  it("displays loading button state when mutation is pending", () => {
    vi.mocked(postMediaModule.useCreateMedia).mockReturnValue({
      mutateAsync: mockMutateAsync,
      isPending: true,
    } as unknown as ReturnType<typeof postMediaModule.useCreateMedia>);

    render(<AddMediaDialog open={true} onOpenChange={vi.fn()} />);

    expect(screen.getByText("Adding...")).toBeDefined();
    const button = screen.getByRole("button", { name: /adding/i });
    expect(button).toHaveProperty("disabled", true);
  });
});

