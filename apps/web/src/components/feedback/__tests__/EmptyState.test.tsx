import { describe, it, expect } from "vitest";
import { render, screen } from "@/test/test-utils";
import { EmptyState } from "../EmptyState";

describe("EmptyState", () => {
  it("renders default title and message", () => {
    render(<EmptyState />);
    expect(screen.getByText("No items found")).toBeDefined();
    expect(
      screen.getByText("There are no items to display at this time."),
    ).toBeDefined();
  });

  it("renders custom title, message, and icon", () => {
    render(
      <EmptyState
        title="No media found"
        message="No items match your active filters."
        icon={<span data-testid="custom-icon">Icon</span>}
      />,
    );
    expect(screen.getByText("No media found")).toBeDefined();
    expect(screen.getByText("No items match your active filters.")).toBeDefined();
    expect(screen.getByTestId("custom-icon")).toBeDefined();
  });

  it("renders custom action element", () => {
    render(
      <EmptyState
        title="No items"
        action={<button type="button">Reset</button>}
      />,
    );
    expect(screen.getByRole("button", { name: "Reset" })).toBeDefined();
  });

  it("renders inside workspace when withWorkspace is true", () => {
    render(<EmptyState withWorkspace={true} title="Workspace Empty" />);
    expect(screen.getByText("Workspace Empty")).toBeDefined();
  });
});
