import { describe, it, expect, vi, beforeEach } from "vitest";
import { fireEvent, render, screen } from "@/test/test-utils";
import { Filters } from "../Filters";
import { LibraryProvider } from "../../../store/LibraryContext";

const mockSetSearch = vi.fn();

vi.mock("../../../hooks/useLibrary", () => ({
  useLibrary: () => ({
    search: "",
    setSearch: mockSetSearch,
  }),
}));

function renderFilters() {
  return render(
    <LibraryProvider>
      <Filters />
    </LibraryProvider>,
  );
}

describe("Filters Component", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders search action trigger button", () => {
    renderFilters();

    expect(screen.getByRole("button", { name: "Filters" })).toBeDefined();
  });

  it("opens sheet modal when filters button is clicked", () => {
    renderFilters();

    const trigger = screen.getByRole("button", { name: "Filters" });
    fireEvent.click(trigger);

    expect(screen.getByText("Search library")).toBeDefined();
    expect(screen.getByText("Search your media collection.")).toBeDefined();
    expect(screen.getByPlaceholderText("Search by title")).toBeDefined();
  });

  it("submits search query and calls setSearch", () => {
    renderFilters();

    const trigger = screen.getByRole("button", { name: "Filters" });
    fireEvent.click(trigger);

    const input = screen.getByPlaceholderText("Search by title");
    fireEvent.change(input, { target: { value: "Elden Ring" } });

    const submitButton = screen.getByRole("button", { name: "Search" });
    fireEvent.click(submitButton);

    expect(mockSetSearch).toHaveBeenCalledWith("Elden Ring");
  });
});
