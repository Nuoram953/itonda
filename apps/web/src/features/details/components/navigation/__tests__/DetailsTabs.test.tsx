import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@/test/test-utils";
import { DetailsTabs } from "../DetailsTabs";

describe("DetailsTabs Component", () => {
  it("renders all four tabs", () => {
    const handleChange = vi.fn();
    render(<DetailsTabs activeTab="overview" onChange={handleChange} />);

    expect(screen.getByRole("tab", { name: /Overview/i })).toBeDefined();
    expect(screen.getByRole("tab", { name: /Reviews & Notes/i })).toBeDefined();
    expect(screen.getByRole("tab", { name: /Gallery & Clips/i })).toBeDefined();
    expect(screen.getByRole("tab", { name: /Details/i })).toBeDefined();
  });

  it("marks the active tab correctly", () => {
    const handleChange = vi.fn();
    render(<DetailsTabs activeTab="reviews" onChange={handleChange} />);

    const reviewsTab = screen.getByRole("tab", { name: /Reviews & Notes/i });
    expect(reviewsTab.getAttribute("aria-selected")).toBe("true");

    const overviewTab = screen.getByRole("tab", { name: /Overview/i });
    expect(overviewTab.getAttribute("aria-selected")).toBe("false");
  });

  it("calls onChange when a tab is clicked", () => {
    const handleChange = vi.fn();
    render(<DetailsTabs activeTab="overview" onChange={handleChange} />);

    const galleryTab = screen.getByRole("tab", { name: /Gallery & Clips/i });
    fireEvent.click(galleryTab);

    expect(handleChange).toHaveBeenCalledWith("gallery");
  });

  it("renders as full-width navigation element", () => {
    const handleChange = vi.fn();
    const { container } = render(
      <DetailsTabs activeTab="overview" onChange={handleChange} />,
    );

    const nav = container.querySelector("nav");
    expect(nav).toBeDefined();
    expect(nav?.getAttribute("aria-label")).toBe("Media navigation");
    expect(nav?.className).toContain("w-full");
    expect(nav?.className).toContain("border-b");
    expect(nav?.className).toContain("sticky");
  });
});
