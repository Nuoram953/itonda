import { describe, it, expect } from "vitest";
import { render, screen } from "@/test/test-utils";
import { LoadingState } from "../LoadingState";

describe("LoadingState", () => {
  it("renders default loading message", () => {
    render(<LoadingState />);
    expect(screen.getByText("Loading...")).toBeDefined();
  });

  it("renders custom message", () => {
    render(<LoadingState message="Loading media details..." />);
    expect(screen.getByText("Loading media details...")).toBeDefined();
  });

  it("renders without workspace wrapper when withWorkspace is false", () => {
    const { container } = render(
      <LoadingState message="Loading items..." withWorkspace={false} />,
    );
    expect(screen.getByText("Loading items...")).toBeDefined();
    expect(container.querySelector("[data-workspace]")).toBeNull();
  });
});
