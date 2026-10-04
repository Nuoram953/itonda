import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@/test/test-utils";
import {
  Form,
  FormField,
  FormItem,
  FormLabel,
  FormControl,
  FormDescription,
  FormMessage,
  FormSelection,
  FormSelectionGroup,
  FormInput,
  FormTextarea,
} from "../form";

describe("Form UI Components", () => {
  it("renders Form and Form.Field with proper spacing and slots", () => {
    const { container } = render(
      <Form>
        <Form.Field data-testid="field-1">
          <Form.Label>Field 1</Form.Label>
          <input placeholder="input 1" />
        </Form.Field>
      </Form>
    );

    const formElement = container.querySelector("form");
    expect(formElement).toBeDefined();
    expect(formElement?.getAttribute("data-slot")).toBe("form");

    const field = screen.getByTestId("field-1");
    expect(field.getAttribute("data-slot")).toBe("form-item");
    expect(field.className).toContain("flex-col");
    expect(field.className).toContain("gap-2.5");
  });

  it("renders Form.Label with uppercase section styling by default and standard variant", () => {
    render(
      <div>
        <Form.Label data-testid="default-label">Section Title</Form.Label>
        <Form.Label data-testid="standard-label" variant="standard">
          Standard Title
        </Form.Label>
      </div>
    );

    const defaultLabel = screen.getByTestId("default-label");
    expect(defaultLabel.className).toContain("uppercase");
    expect(defaultLabel.className).toContain("text-xs");

    const standardLabel = screen.getByTestId("standard-label");
    expect(standardLabel.className).toContain("text-sm");
    expect(standardLabel.className).not.toContain("uppercase");
  });

  it("renders Form.Control wrapper", () => {
    render(
      <Form.Control data-testid="control">
        <input placeholder="text" />
      </Form.Control>
    );

    const control = screen.getByTestId("control");
    expect(control.getAttribute("data-slot")).toBe("form-control");
  });

  it("renders Form.Description and Form.Message", () => {
    render(
      <div>
        <Form.Description data-testid="desc">Helper note</Form.Description>
        <Form.Message data-testid="msg">Required field</Form.Message>
      </div>
    );

    const desc = screen.getByTestId("desc");
    expect(desc.getAttribute("data-slot")).toBe("form-description");
    expect(desc.textContent).toBe("Helper note");

    const msg = screen.getByTestId("msg");
    expect(msg.getAttribute("data-slot")).toBe("form-message");
    expect(msg.getAttribute("role")).toBe("alert");
    expect(msg.textContent).toBe("Required field");
  });

  it("renders Form.Selection and handles click and selection states", () => {
    const handleClick = vi.fn();
    const { rerender } = render(
      <Form.SelectionGroup data-testid="group">
        <Form.Selection selected={false} onClick={handleClick}>
          Option A
        </Form.Selection>
        <Form.Selection selected={true}>
          Option B
        </Form.Selection>
      </Form.SelectionGroup>
    );

    const group = screen.getByTestId("group");
    expect(group.getAttribute("data-slot")).toBe("form-selection-group");

    const optionA = screen.getByRole("button", { name: "Option A" });
    const optionB = screen.getByRole("button", { name: "Option B" });

    expect(optionA.getAttribute("aria-pressed")).toBe("false");
    expect(optionB.getAttribute("aria-pressed")).toBe("true");
    expect(optionB.className).toContain("border-primary");

    fireEvent.click(optionA);
    expect(handleClick).toHaveBeenCalledTimes(1);

    rerender(
      <Form.SelectionGroup>
        <Form.Selection selected={true}>Option A</Form.Selection>
      </Form.SelectionGroup>
    );
    expect(screen.getByRole("button", { name: "Option A" }).getAttribute("aria-pressed")).toBe("true");
  });

  it("renders Form.Input with common stylings and slots", () => {
    render(
      <Form.Input
        placeholder="Search title..."
        data-testid="form-input"
        className="pl-9"
      />
    );

    const input = screen.getByTestId("form-input");
    expect(input.getAttribute("data-slot")).toBe("form-input");
    expect(input.className).toContain("bg-surface/50");
    expect(input.className).toContain("rounded-xl");
    expect(input.className).toContain("h-10");
    expect(input.className).toContain("text-sm");
    expect(input.className).toContain("pl-9");
  });

  it("renders Form.Textarea with common stylings and slots", () => {
    render(
      <Form.Textarea
        placeholder="Write thoughts..."
        data-testid="form-textarea"
        rows={4}
      />
    );

    const textarea = screen.getByTestId("form-textarea");
    expect(textarea.getAttribute("data-slot")).toBe("form-textarea");
    expect(textarea.className).toContain("bg-surface/50");
    expect(textarea.className).toContain("rounded-xl");
    expect(textarea.className).toContain("border-input");
    expect(textarea.className).toContain("text-sm");
    expect(textarea.getAttribute("rows")).toBe("4");
  });

  it("supports both compound Form and named exports", () => {
    expect(Form.Field).toBe(FormItem);
    expect(Form.Item).toBe(FormItem);
    expect(Form.Field).toBe(FormField);
    expect(Form.Label).toBe(FormLabel);
    expect(Form.Control).toBe(FormControl);
    expect(Form.Input).toBe(FormInput);
    expect(Form.Textarea).toBe(FormTextarea);
    expect(Form.Description).toBe(FormDescription);
    expect(Form.Message).toBe(FormMessage);
    expect(Form.Selection).toBe(FormSelection);
    expect(Form.SelectionGroup).toBe(FormSelectionGroup);
  });
});
