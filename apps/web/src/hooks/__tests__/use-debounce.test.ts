import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { useDebounce } from "../use-debounce";

describe("useDebounce", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("returns initial value immediately", () => {
    const { result } = renderHook(() => useDebounce("hello", 400));
    expect(result.current).toBe("hello");
  });

  it("debounces value changes according to delayMs", () => {
    let value = "first";
    const { result, rerender } = renderHook(() => useDebounce(value, 400));

    expect(result.current).toBe("first");

    value = "second";
    rerender();

    act(() => {
      vi.advanceTimersByTime(200);
    });
    expect(result.current).toBe("first");

    value = "second again";
    rerender();

    act(() => {
      vi.advanceTimersByTime(300);
    });
    expect(result.current).toBe("first");

    act(() => {
      vi.advanceTimersByTime(100);
    });
    expect(result.current).toBe("second again");
  });
});
