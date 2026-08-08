import { describe, it, expect, vi, beforeEach } from "vitest";
import { fireEvent, render, screen } from "@/test/test-utils";
import { Status } from "../status";
import type { components } from "@/api/generated.d";

const mockMutate = vi.fn();

type MediaStatus = components["schemas"]["MediaStatus"];
const STATUS_OPTIONS: MediaStatus[] = [
  "not_started",
  "in_progress",
  "completed",
  "abandoned",
  "paused",
];

vi.mock("../../../api/patch-media-status", () => ({
  usePatchMediaStatus: () => ({
    mutate: mockMutate,
  }),
}));

describe("Status Component", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders the trigger button displaying currentStatus when provided", () => {
    render(<Status mediaId="media-123" currentStatus="in_progress" />);

    expect(screen.getByRole("button", { name: "in_progress" })).toBeDefined();
  });

  it("opens menu displaying all status options when trigger button is clicked", () => {
    render(<Status mediaId="media-123" currentStatus="not_started" />);

    const trigger = screen.getByRole("button", { name: "not_started" });
    fireEvent.click(trigger);

    STATUS_OPTIONS.forEach((status) => {
      expect(screen.getAllByText(status).length).toBeGreaterThan(0);
    });
  });

  it("calls patchStatusMutation when a different status option is selected", () => {
    render(<Status mediaId="media-123" currentStatus="not_started" />);

    const trigger = screen.getByRole("button", { name: "not_started" });
    fireEvent.click(trigger);

    const completedOption = screen.getByText("completed");
    fireEvent.click(completedOption);

    expect(mockMutate).toHaveBeenCalledOnce();
    expect(mockMutate).toHaveBeenCalledWith({
      mediaId: "media-123",
      statusId: "completed",
    });
  });

  it("does not call patchStatusMutation when selecting the status that matches currentStatus", () => {
    render(<Status mediaId="media-123" currentStatus="in_progress" />);

    const trigger = screen.getByRole("button", { name: "in_progress" });
    fireEvent.click(trigger);

    const menuItems = screen.getAllByText("in_progress");
    // Click the dropdown menu item (the option inside the open dropdown menu)
    const inProgressOption = menuItems[menuItems.length - 1];
    fireEvent.click(inProgressOption);

    expect(mockMutate).not.toHaveBeenCalled();
  });
});
