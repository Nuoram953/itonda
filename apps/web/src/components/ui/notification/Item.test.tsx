import { fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@/test/test-utils";
import { NotificationItem } from "./Item";

describe("NotificationItem", () => {
  it("renders the notification title and description", () => {
    render(
      <NotificationItem
        notification={{
          id: "test-1",
          title: "Profile Updated",
          description: "Your changes have been saved.",
          severity: "success",
        }}
      />,
    );
    expect(screen.findByText("Profile Updated")).toBeDefined();
  });

  it("fires the custom action when the action button is clicked", () => {
    const mockActionClick = vi.fn();

    render(
      <NotificationItem
        notification={{
          id: "test-2",
          title: "Connection Lost",
          severity: "error",
          action: { label: "Retry Connection", onClick: mockActionClick },
        }}
      />,
    );

    const actionButton = screen.getByRole("button", {
      name: "Retry Connection",
    });
    fireEvent.click(actionButton);

    expect(mockActionClick).toHaveBeenCalledOnce();
  });

  it("renders an indeterminate loading progress bar when severity is loading", () => {
    render(
      <NotificationItem
        notification={{
          id: "test-loading",
          title: "Syncing Library",
          description: "Scanning and updating media...",
          severity: "loading",
        }}
      />,
    );

    const progressBar = screen.getByRole("progressbar", { name: "Loading" });
    expect(progressBar).toBeDefined();
    expect(progressBar.firstElementChild?.className).toContain(
      "animate-indeterminate",
    );
  });

  it("does not render a progress bar for non-loading notifications", () => {
    render(
      <NotificationItem
        notification={{
          id: "test-info",
          title: "Notice",
          description: "Everything is fine.",
          severity: "info",
        }}
      />,
    );

    expect(screen.queryByRole("progressbar")).toBeNull();
  });
});

