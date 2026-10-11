import { describe, it, expect } from "vitest";
import {
  matchesShortcut,
  formatShortcutDisplay,
  eventToShortcutCombo,
  parseShortcutCombo,
  isEditableElement,
} from "../utils";

describe("Shortcuts Utils", () => {
  describe("parseShortcutCombo", () => {
    it("parses Mod+K correctly", () => {
      const parsed = parseShortcutCombo("Mod+K");
      expect(parsed.hasMod).toBe(true);
      expect(parsed.key).toBe("k");
      expect(parsed.hasShift).toBe(false);
      expect(parsed.hasAlt).toBe(false);
    });

    it("parses Shift+Tab correctly", () => {
      const parsed = parseShortcutCombo("Shift+Tab");
      expect(parsed.hasShift).toBe(true);
      expect(parsed.key).toBe("tab");
      expect(parsed.hasMod).toBe(false);
    });

    it("parses Alt+Shift+Mod+X correctly", () => {
      const parsed = parseShortcutCombo("Alt+Shift+Mod+X");
      expect(parsed.hasAlt).toBe(true);
      expect(parsed.hasShift).toBe(true);
      expect(parsed.hasMod).toBe(true);
      expect(parsed.key).toBe("x");
    });
  });

  describe("matchesShortcut", () => {
    it("matches Mod+K with metaKey", () => {
      const event = new KeyboardEvent("keydown", {
        key: "k",
        metaKey: true,
      });
      expect(matchesShortcut(event, "Mod+K")).toBe(true);
    });

    it("matches Mod+K with ctrlKey", () => {
      const event = new KeyboardEvent("keydown", {
        key: "k",
        ctrlKey: true,
      });
      expect(matchesShortcut(event, "Mod+K")).toBe(true);
    });

    it("does not match Mod+K when no modifier is pressed", () => {
      const event = new KeyboardEvent("keydown", {
        key: "k",
      });
      expect(matchesShortcut(event, "Mod+K")).toBe(false);
    });

    it("matches Tab key with no modifiers", () => {
      const event = new KeyboardEvent("keydown", {
        key: "Tab",
      });
      expect(matchesShortcut(event, "Tab")).toBe(true);
    });

    it("does not match Tab when Shift is held if combo is Tab", () => {
      const event = new KeyboardEvent("keydown", {
        key: "Tab",
        shiftKey: true,
      });
      expect(matchesShortcut(event, "Tab")).toBe(false);
    });

    it("matches Shift+Tab correctly", () => {
      const event = new KeyboardEvent("keydown", {
        key: "Tab",
        shiftKey: true,
      });
      expect(matchesShortcut(event, "Shift+Tab")).toBe(true);
    });

    it("matches Escape key", () => {
      const event = new KeyboardEvent("keydown", {
        key: "Escape",
      });
      expect(matchesShortcut(event, "Escape")).toBe(true);
      expect(matchesShortcut(event, "Esc")).toBe(true);
    });
  });

  describe("formatShortcutDisplay", () => {
    it("formats Mod+K as ⌘K by default", () => {
      expect(formatShortcutDisplay("Mod+K")).toBe("⌘K");
    });

    it("formats Mod+K as Ctrl+K when mac symbols disabled and not mac", () => {
      expect(
        formatShortcutDisplay("Mod+K", { preferMacSymbols: false }),
      ).toBe("Ctrl+K");
    });

    it("formats symbols as-is", () => {
      expect(formatShortcutDisplay("↑↓")).toBe("↑↓");
      expect(formatShortcutDisplay("↵")).toBe("↵");
      expect(formatShortcutDisplay("Esc")).toBe("Esc");
      expect(formatShortcutDisplay("Tab")).toBe("Tab");
    });
  });

  describe("eventToShortcutCombo", () => {
    it("returns null for bare modifier keydown", () => {
      const event = new KeyboardEvent("keydown", { key: "Shift", shiftKey: true });
      expect(eventToShortcutCombo(event)).toBeNull();
    });

    it("records Mod+P correctly", () => {
      const event = new KeyboardEvent("keydown", {
        key: "p",
        metaKey: true,
      });
      expect(eventToShortcutCombo(event)).toBe("Mod+P");
    });

    it("records Shift+Tab correctly", () => {
      const event = new KeyboardEvent("keydown", {
        key: "Tab",
        shiftKey: true,
      });
      expect(eventToShortcutCombo(event)).toBe("Shift+Tab");
    });
  });

  describe("isEditableElement", () => {
    it("identifies input elements", () => {
      const input = document.createElement("input");
      expect(isEditableElement(input)).toBe(true);
    });

    it("identifies textarea elements", () => {
      const textarea = document.createElement("textarea");
      expect(isEditableElement(textarea)).toBe(true);
    });

    it("identifies non-editable div", () => {
      const div = document.createElement("div");
      expect(isEditableElement(div)).toBe(false);
    });
  });
});
