import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@/test/test-utils";
import { Button } from "../button";

describe("Button UI Component", () => {
  it("renders children and handles clicks", () => {
    const handleClick = vi.fn();
    render(<Button onClick={handleClick}>Click Me</Button>);

    const button = screen.getByRole("button", { name: "Click Me" });
    expect(button).toBeDefined();

    fireEvent.click(button);
    expect(handleClick).toHaveBeenCalledTimes(1);
  });

  it("applies variant and size classes", () => {
    const { container } = render(
      <Button variant="destructive" size="sm">
        Delete
      </Button>,
    );

    const button = container.querySelector("button");
    expect(button?.getAttribute("data-slot")).toBe("button");
    expect(button?.className).toContain("bg-destructive/10");
    expect(button?.className).toContain("h-8");
  });

  it("disables button and displays loader when loading is true", () => {
    const handleClick = vi.fn();
    const { container } = render(
      <Button loading onClick={handleClick}>
        Submit
      </Button>,
    );

    const button = screen.getByRole("button");
    expect(button).toHaveProperty("disabled", true);
    expect(button.getAttribute("aria-busy")).toBe("true");

    const spinner = container.querySelector(".animate-spin");
    expect(spinner).not.toBeNull();

    fireEvent.click(button);
    expect(handleClick).not.toHaveBeenCalled();
  });

  it("renders loadingText when loading is true and loadingText is provided", () => {
    render(
      <Button loading loadingText="Saving...">
        Save
      </Button>,
    );

    expect(screen.queryByText("Save")).toBeNull();
    expect(screen.getByText("Saving...")).toBeDefined();
  });

  it("renders children alongside spinner when loading is true without loadingText", () => {
    render(<Button loading>Save</Button>);

    expect(screen.getByText("Save")).toBeDefined();
  });
});
