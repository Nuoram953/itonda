import { describe, it, expect, beforeEach } from "vitest";
import { renderHook, act } from "@testing-library/react";
import {
  safeStorage,
  useStorageState,
  STORAGE_KEYS,
} from "../storage";

describe("storage utilities", () => {
  beforeEach(() => {
    sessionStorage.clear();
    localStorage.clear();
  });

  describe("STORAGE_KEYS registry", () => {
    it("has defined keys", () => {
      expect(STORAGE_KEYS.ACTIVE_MEDIA_SESSION).toBe("itonda_active_media_session");
      expect(STORAGE_KEYS.THEME).toBe("itonda_ui_theme");
    });
  });

  describe("safeStorage operations", () => {
    it("gets fallback when item does not exist", () => {
      const result = safeStorage.get("non_existent_key", { default: true }, "session");
      expect(result).toEqual({ default: true });
    });

    it("sets and gets JSON objects cleanly in sessionStorage", () => {
      const payload = { mediaId: "m1", startedAt: 12345 };
      const success = safeStorage.set("test_key", payload, "session");
      expect(success).toBe(true);

      const retrieved = safeStorage.get("test_key", null, "session");
      expect(retrieved).toEqual(payload);
    });

    it("sets and gets JSON objects in localStorage", () => {
      const theme = "dark";
      const success = safeStorage.set(STORAGE_KEYS.THEME, theme, "local");
      expect(success).toBe(true);

      const retrieved = safeStorage.get(STORAGE_KEYS.THEME, "system", "local");
      expect(retrieved).toBe("dark");
    });

    it("removes item safely", () => {
      safeStorage.set("remove_me", "data", "session");
      expect(safeStorage.get("remove_me", null, "session")).toBe("data");

      safeStorage.remove("remove_me", "session");
      expect(safeStorage.get("remove_me", null, "session")).toBeNull();
    });

    it("removes item when setting null or undefined", () => {
      safeStorage.set("null_key", "val", "session");
      safeStorage.set("null_key", null, "session");
      expect(safeStorage.get("null_key", null, "session")).toBeNull();
    });

    it("handles corrupt JSON gracefully and returns fallback", () => {
      sessionStorage.setItem("corrupt_key", "{ bad json");
      const result = safeStorage.get("corrupt_key", "fallback_val", "session");
      expect(result).toBe("fallback_val");
    });

    it("clears storage cleanly", () => {
      safeStorage.set("k1", "v1", "session");
      safeStorage.set("k2", "v2", "session");
      safeStorage.clear("session");
      expect(safeStorage.get("k1", null, "session")).toBeNull();
      expect(safeStorage.get("k2", null, "session")).toBeNull();
    });
  });

  describe("useStorageState hook", () => {
    it("initializes with fallback value", () => {
      const { result } = renderHook(() =>
        useStorageState("hook_key", "initial_value", { type: "session" }),
      );

      expect(result.current[0]).toBe("initial_value");
    });

    it("updates value and persists to storage", () => {
      const { result } = renderHook(() =>
        useStorageState("hook_key", "initial_value", { type: "session" }),
      );

      act(() => {
        result.current[1]("updated_value");
      });

      expect(result.current[0]).toBe("updated_value");
      expect(safeStorage.get("hook_key", null, "session")).toBe("updated_value");
    });

    it("supports updater function", () => {
      const { result } = renderHook(() =>
        useStorageState("counter_key", 0, { type: "session" }),
      );

      act(() => {
        result.current[1]((prev) => prev + 1);
      });

      expect(result.current[0]).toBe(1);
    });
  });
});
