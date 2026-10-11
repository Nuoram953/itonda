import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, act, fireEvent } from "@testing-library/react";
import { ShortcutsProvider, useShortcuts, useShortcut } from "../index";

function TestComponent({ onAction }: { onAction: (text: string) => void }) {
  const { getDisplayString, updateShortcut, resetShortcut, resetAll } =
    useShortcuts();

  useShortcut("search.open", () => onAction("search opened"));
  useShortcut("search.switchAction", () => onAction("switched action"));

  return (
    <div>
      <span data-testid="search-display">
        {getDisplayString("search.open")}
      </span>
      <span data-testid="switch-display">
        {getDisplayString("search.switchAction")}
      </span>
      <button
        type="button"
        onClick={() => updateShortcut("search.open", "Mod+P")}
      >
        Change search key
      </button>
      <button
        type="button"
        onClick={() => updateShortcut("search.switchAction", "Shift+Tab")}
      >
        Change switch key
      </button>
      <button type="button" onClick={() => resetShortcut("search.open")}>
        Reset search key
      </button>
      <button type="button" onClick={resetAll}>
        Reset all
      </button>
    </div>
  );
}

describe("ShortcutsProvider & hooks", () => {
  beforeEach(() => {
    window.localStorage.clear();
  });

  it("provides default shortcuts", () => {
    render(
      <ShortcutsProvider>
        <TestComponent onAction={() => {}} />
      </ShortcutsProvider>,
    );

    expect(screen.getByTestId("search-display").textContent).toBe("⌘K");
    expect(screen.getByTestId("switch-display").textContent).toBe("Tab");
  });

  it("handles key triggers with default bindings", () => {
    const handleAction = vi.fn();
    render(
      <ShortcutsProvider>
        <TestComponent onAction={handleAction} />
      </ShortcutsProvider>,
    );

    act(() => {
      window.dispatchEvent(
        new KeyboardEvent("keydown", { key: "k", metaKey: true }),
      );
    });

    expect(handleAction).toHaveBeenCalledWith("search opened");
  });

  it("updates shortcut and triggers with new keybinding", () => {
    const handleAction = vi.fn();
    render(
      <ShortcutsProvider>
        <TestComponent onAction={handleAction} />
      </ShortcutsProvider>,
    );

    // Change shortcut to Mod+P
    fireEvent.click(screen.getByText("Change search key"));

    expect(screen.getByTestId("search-display").textContent).toBe("⌘P");

    // Old key (Mod+K) should no longer trigger
    act(() => {
      window.dispatchEvent(
        new KeyboardEvent("keydown", { key: "k", metaKey: true }),
      );
    });
    expect(handleAction).not.toHaveBeenCalled();

    // New key (Mod+P) should trigger
    act(() => {
      window.dispatchEvent(
        new KeyboardEvent("keydown", { key: "p", metaKey: true }),
      );
    });
    expect(handleAction).toHaveBeenCalledWith("search opened");
  });

  it("persists custom shortcut to localStorage", () => {
    render(
      <ShortcutsProvider>
        <TestComponent onAction={() => {}} />
      </ShortcutsProvider>,
    );

    fireEvent.click(screen.getByText("Change search key"));

    const stored = JSON.parse(
      window.localStorage.getItem("itonda:shortcuts") || "{}",
    );
    expect(stored["search.open"]).toBe("Mod+P");
  });

  it("resets shortcut to default", () => {
    render(
      <ShortcutsProvider>
        <TestComponent onAction={() => {}} />
      </ShortcutsProvider>,
    );

    fireEvent.click(screen.getByText("Change search key"));
    expect(screen.getByTestId("search-display").textContent).toBe("⌘P");

    fireEvent.click(screen.getByText("Reset search key"));
    expect(screen.getByTestId("search-display").textContent).toBe("⌘K");
  });
});
