import { describe, it, expect } from "vitest";
import { render, screen } from "@/test/test-utils";
import {
  Dialog,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "../dialog";
import { Plus } from "lucide-react";

describe("Dialog UI Component", () => {
  it("renders with compound component API and built-in typography", () => {
    render(
      <Dialog open={true}>
        <Dialog.Content>
          <Dialog.Header>
            <Dialog.Title>Test Title</Dialog.Title>
            <Dialog.Description>Test Description</Dialog.Description>
          </Dialog.Header>
          <div>Body content</div>
          <Dialog.Footer>
            <button type="button">Action</button>
          </Dialog.Footer>
        </Dialog.Content>
      </Dialog>,
    );

    const title = screen.getByText("Test Title");
    expect(title).toBeDefined();
    expect(title.getAttribute("data-slot")).toBe("dialog-title");
    expect(title.className).toContain("text-lg");
    expect(title.className).toContain("font-bold");

    const desc = screen.getByText("Test Description");
    expect(desc).toBeDefined();
    expect(desc.getAttribute("data-slot")).toBe("dialog-description");
    expect(desc.className).toContain("text-xs");

    expect(screen.getByText("Body content")).toBeDefined();
    expect(screen.getByRole("button", { name: "Action" })).toBeDefined();
  });

  it("renders built-in icon badge when icon prop is passed to Dialog.Header", () => {
    render(
      <Dialog open={true}>
        <Dialog.Content>
          <Dialog.Header icon={Plus}>
            <Dialog.Title>Add Item</Dialog.Title>
            <Dialog.Description>
              Add a new item to your collection
            </Dialog.Description>
          </Dialog.Header>
        </Dialog.Content>
      </Dialog>,
    );

    expect(screen.getByText("Add Item")).toBeDefined();
    expect(screen.getByText("Add a new item to your collection")).toBeDefined();

    const iconContainer = document.querySelector(".from-primary");
    expect(iconContainer).not.toBeNull();
    expect(iconContainer?.className).toContain("w-10");
    expect(iconContainer?.className).toContain("h-10");
  });

  it("accepts ReactNode elements as icon", () => {
    render(
      <Dialog open={true}>
        <Dialog.Content>
          <Dialog.Header icon={<Plus data-testid="custom-icon" />}>
            <Dialog.Title>Custom Icon</Dialog.Title>
          </Dialog.Header>
        </Dialog.Content>
      </Dialog>,
    );

    expect(screen.getByTestId("custom-icon")).toBeDefined();
    const iconContainer = document.querySelector(".from-primary");
    expect(iconContainer).not.toBeNull();
  });

  it("supports standalone named exports for backwards compatibility", () => {
    render(
      <Dialog open={true}>
        <Dialog.Content>
          <DialogHeader>
            <DialogTitle>Standalone Title</DialogTitle>
            <DialogDescription>Standalone Description</DialogDescription>
          </DialogHeader>
        </Dialog.Content>
      </Dialog>,
    );

    expect(screen.getByText("Standalone Title")).toBeDefined();
    expect(screen.getByText("Standalone Description")).toBeDefined();
  });
});
