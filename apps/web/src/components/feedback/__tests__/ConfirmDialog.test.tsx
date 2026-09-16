import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@/test/test-utils";
import { ConfirmDialog } from "../ConfirmDialog";

describe("ConfirmDialog Component", () => {
  it("renders modal when open", () => {
    render(
      <ConfirmDialog
        open={true}
        onOpenChange={vi.fn()}
        title="Delete Item"
        description="Are you sure you want to delete this?"
        onConfirm={vi.fn()}
      />,
    );

    expect(screen.getByText("Delete Item")).toBeDefined();
    expect(screen.getByText("Are you sure you want to delete this?")).toBeDefined();
    expect(screen.getByText("Cancel")).toBeDefined();
    expect(screen.getByText("Delete")).toBeDefined();
  });

  it("calls onConfirm when confirm button is clicked", async () => {
    const onConfirm = vi.fn().mockResolvedValue(undefined);
    const onOpenChange = vi.fn();

    render(
      <ConfirmDialog
        open={true}
        onOpenChange={onOpenChange}
        title="Delete Item"
        description="Are you sure you want to delete this?"
        confirmLabel="Remove Review"
        onConfirm={onConfirm}
      />,
    );

    fireEvent.click(screen.getByText("Remove Review"));
    expect(onConfirm).toHaveBeenCalledTimes(1);
  });

  it("closes modal when cancel button is clicked", () => {
    const onOpenChange = vi.fn();

    render(
      <ConfirmDialog
        open={true}
        onOpenChange={onOpenChange}
        title="Delete Item"
        description="Are you sure you want to delete this?"
        onConfirm={vi.fn()}
      />,
    );

    fireEvent.click(screen.getByText("Cancel"));
    expect(onOpenChange).toHaveBeenCalledWith(false);
  });
});
