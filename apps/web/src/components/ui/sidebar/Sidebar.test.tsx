import { describe, it, expect } from "vitest";
import { screen, fireEvent, renderWithRouter } from "@/test/test-utils";
import { Sidebar } from "./Sidebar";

describe("Sidebar", () => {
  it("renders all top-level and nested navigation links", async () => {
    renderWithRouter(<Sidebar />);

    expect(await screen.findByText("Home")).toBeDefined();
    expect(await screen.findByText("Rankings")).toBeDefined();
    expect(await screen.findAllByText("Library")).toBeDefined();
  });

  it("toggles nested links when clicking the chevron icon", async () => {
    renderWithRouter(<Sidebar />);

    expect(await screen.findByText("Games")).toBeDefined();

    const toggleIcon = screen.getByLabelText("toggle");

    fireEvent.click(toggleIcon);

    expect(screen.queryByText("Games")).toBeNull();
  });
});
