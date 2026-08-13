import { describe, it, expect } from "vitest";
import { render, screen } from "@/test/test-utils";
import { ErrorState } from "../ErrorState";

describe("ErrorState", () => {
  it("renders default title and message", () => {
    render(<ErrorState />);
    expect(screen.getByText("Something went wrong")).toBeDefined();
    expect(
      screen.getByText(
        "The requested item could not be loaded or may have been deleted.",
      ),
    ).toBeDefined();
  });

  it("renders custom title and message", () => {
    render(
      <ErrorState
        title="Media not found"
        message="The media file does not exist."
      />,
    );
    expect(screen.getByText("Media not found")).toBeDefined();
    expect(screen.getByText("The media file does not exist.")).toBeDefined();
  });

  it("renders custom action element", () => {
    render(
      <ErrorState
        title="Error"
        action={<button type="button">Retry</button>}
      />,
    );
    expect(screen.getByRole("button", { name: "Retry" })).toBeDefined();
  });
});
