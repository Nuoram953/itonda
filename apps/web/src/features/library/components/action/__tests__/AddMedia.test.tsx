import { describe, it, expect, vi } from "vitest";
import { fireEvent, render, screen } from "@/test/test-utils";
import { AddMedia } from "../AddMedia";

describe("AddMedia Component", () => {
  it("renders Add Media action button", () => {
    render(<AddMedia />);

    expect(screen.getByRole("button", { name: "Add Media" })).toBeDefined();
  });

  it("handles click events", () => {
    const handleClick = vi.fn();
    render(<AddMedia onClick={handleClick} />);

    const button = screen.getByRole("button", { name: "Add Media" });
    fireEvent.click(button);

    expect(handleClick).toHaveBeenCalledTimes(1);
  });
});
