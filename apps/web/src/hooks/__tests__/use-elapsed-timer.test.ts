import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { useElapsedTimer } from "@/hooks/use-elapsed-timer";

describe("useElapsedTimer", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("returns 0 elapsed time when startedAt is null", () => {
    const { result } = renderHook(() => useElapsedTimer(null));
    expect(result.current.elapsedSeconds).toBe(0);
    expect(result.current.formattedElapsed).toBe("00:00");
  });

  it("calculates elapsed time and ticks every second", () => {
    const startTime = Date.now();
    const { result } = renderHook(() => useElapsedTimer(startTime));

    expect(result.current.elapsedSeconds).toBe(0);
    expect(result.current.formattedElapsed).toBe("00:00");

    act(() => {
      vi.advanceTimersByTime(3000);
    });

    expect(result.current.elapsedSeconds).toBe(3);
    expect(result.current.formattedElapsed).toBe("00:03");

    act(() => {
      vi.advanceTimersByTime(65000);
    });

    expect(result.current.elapsedSeconds).toBe(68);
    expect(result.current.formattedElapsed).toBe("01:08");
  });

  it("handles transition from active session to null", () => {
    let startedAt: number | null = Date.now();
    const { result, rerender } = renderHook(() => useElapsedTimer(startedAt));

    act(() => {
      vi.advanceTimersByTime(10000);
    });
    expect(result.current.elapsedSeconds).toBe(10);

    startedAt = null;
    rerender();

    expect(result.current.elapsedSeconds).toBe(0);
    expect(result.current.formattedElapsed).toBe("00:00");
  });
});
