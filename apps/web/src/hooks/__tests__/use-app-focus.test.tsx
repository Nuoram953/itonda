import { describe, it, expect, vi } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { useAppFocus } from "../use-app-focus";

describe("useAppFocus hook", () => {
  it("defaults to true when document has focus", () => {
    const { result } = renderHook(() => useAppFocus());
    expect(result.current).toBe(true);
  });

  it("updates to false when window loses focus (blur event)", () => {
    const { result } = renderHook(() => useAppFocus());

    act(() => {
      window.dispatchEvent(new Event("blur"));
    });

    expect(result.current).toBe(false);
  });

  it("updates to true when window regains focus", () => {
    const { result } = renderHook(() => useAppFocus());

    act(() => {
      window.dispatchEvent(new Event("blur"));
    });
    expect(result.current).toBe(false);

    act(() => {
      window.dispatchEvent(new Event("focus"));
    });
    expect(result.current).toBe(true);
  });

  it("responds to document visibility change", () => {
    const { result } = renderHook(() => useAppFocus());

    act(() => {
      Object.defineProperty(document, "visibilityState", {
        value: "hidden",
        writable: true,
      });
      document.dispatchEvent(new Event("visibilitychange"));
    });

    expect(result.current).toBe(false);

    act(() => {
      Object.defineProperty(document, "visibilityState", {
        value: "visible",
        writable: true,
      });
      document.dispatchEvent(new Event("visibilitychange"));
    });

    expect(result.current).toBe(true);
  });

  it("removes event listeners upon unmount", () => {
    const removeWindowListenerSpy = vi.spyOn(window, "removeEventListener");
    const removeDocListenerSpy = vi.spyOn(document, "removeEventListener");

    const { unmount } = renderHook(() => useAppFocus());
    unmount();

    expect(removeWindowListenerSpy).toHaveBeenCalledWith("focus", expect.any(Function));
    expect(removeWindowListenerSpy).toHaveBeenCalledWith("blur", expect.any(Function));
    expect(removeDocListenerSpy).toHaveBeenCalledWith("visibilitychange", expect.any(Function));

    removeWindowListenerSpy.mockRestore();
    removeDocListenerSpy.mockRestore();
  });
});
