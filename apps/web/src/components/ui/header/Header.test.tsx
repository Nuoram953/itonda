import { describe, it, expect } from "vitest";
import { screen, renderWithRouter } from "@/test/test-utils";
import { Header } from "./Header";

describe("Header", () => {
  it("renders search input and action buttons", async () => {
    renderWithRouter(<Header />);

    expect(
      await screen.findByPlaceholderText("Search media, games, movies..."),
    ).toBeDefined();
    expect(screen.queryByText("Add Media")).toBeNull();
    expect(screen.getByLabelText("Notifications")).toBeDefined();
  });
});

