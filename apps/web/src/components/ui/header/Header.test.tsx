import { describe, it, expect } from "vitest";
import { screen, render } from "@/test/test-utils";
import { Header } from "./Header";

describe("Header", () => {
  it("renders search input and action buttons", () => {
    render(<Header />);

    expect(screen.getByPlaceholderText("Search media, games, movies...")).toBeDefined();
    expect(screen.getByText("Add Media")).toBeDefined();
    expect(screen.getByLabelText("Notifications")).toBeDefined();
  });
});

