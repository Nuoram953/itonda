import { describe, it, expect } from "vitest";
import { screen, fireEvent, renderWithRouter } from "@/test/test-utils";
import { Sidebar } from "./Siderbar";

describe("Sidebar", () => {
  it("renders all top-level and nested navigation links", () => {
    renderWithRouter(<Sidebar />);

    expect(screen.findByText("Home")).toBeDefined();
    expect(screen.findByText("Rankings")).toBeDefined();
    expect(screen.findByText("Library")).toBeDefined();
  });

  it("toggles nested links when clicking the chevron icon", () => {
    renderWithRouter(<Sidebar />);

    expect(screen.findByText("Games")).toBeDefined();

    const toggleIcon = screen.getByLabelText("toggle");
    fireEvent.click(toggleIcon);

    expect(screen.queryByText("Games")).toBeNull();
  });
});
