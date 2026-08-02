import { describe, it, expect, vi, beforeEach } from "vitest";
import { fireEvent, render, screen } from "@/test/test-utils";
import { Search } from "../search";
import { LibraryProvider } from "../../../store/LibraryContext";

const mockSetSearch = vi.fn();

vi.mock("../../../hooks/useLibrary", () => ({
  useLibrary: () => ({
    search: "",
    setSearch: mockSetSearch,
  }),
}));

function renderSearch() {
  return render(
    <LibraryProvider>
      <Search />
    </LibraryProvider>,
  );
}

describe("Search Component", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders search action trigger button", () => {
    renderSearch();

    expect(screen.getByRole("button", { name: "Search" })).toBeDefined();
  });

  it("opens sheet modal when search button is clicked", () => {
    renderSearch();

    const trigger = screen.getByRole("button", { name: "Search" });
    fireEvent.click(trigger);

    expect(screen.getByText("Search library")).toBeDefined();
    expect(screen.getByText("Search your media collection.")).toBeDefined();
    expect(screen.getByPlaceholderText("Search by title")).toBeDefined();
  });

  it("submits search query and calls setSearch", () => {
    renderSearch();

    // Open sheet
    const trigger = screen.getByRole("button", { name: "Search" });
    fireEvent.click(trigger);

    // Type query
    const input = screen.getByPlaceholderText("Search by title");
    fireEvent.change(input, { target: { value: "Elden Ring" } });

    // Submit form by clicking submit button inside sheet
    const submitButtons = screen.getAllByRole("button", { name: "Search" });
    const submitButton = submitButtons[submitButtons.length - 1];
    fireEvent.click(submitButton);

    expect(mockSetSearch).toHaveBeenCalledWith("Elden Ring");
  });
});
