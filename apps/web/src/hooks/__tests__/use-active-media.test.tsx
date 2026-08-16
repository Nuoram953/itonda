import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { useActiveMedia } from "@/app/activeMediaContext";
import { formatElapsedSeconds, formatDurationText } from "@/utils/datetime";

import { ActiveMediaProvider } from "@/app/activeMediaProvider";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import type { PropsWithChildren } from "react";

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
      },
    },
  });

  return function Wrapper({ children }: PropsWithChildren) {
    return (
      <QueryClientProvider client={queryClient}>
        <ActiveMediaProvider>{children}</ActiveMediaProvider>
      </QueryClientProvider>
    );
  };
}

describe("formatElapsedSeconds", () => {
  it("formats seconds and minutes correctly", () => {
    expect(formatElapsedSeconds(0)).toBe("00:00");
    expect(formatElapsedSeconds(9)).toBe("00:09");
    expect(formatElapsedSeconds(45)).toBe("00:45");
    expect(formatElapsedSeconds(65)).toBe("01:05");
    expect(formatElapsedSeconds(600)).toBe("10:00");
  });

  it("formats hours, minutes, and seconds correctly", () => {
    expect(formatElapsedSeconds(3600)).toBe("1:00:00");
    expect(formatElapsedSeconds(3665)).toBe("1:01:05");
    expect(formatElapsedSeconds(7322)).toBe("2:02:02");
  });

  it("handles negative numbers safely", () => {
    expect(formatElapsedSeconds(-5)).toBe("00:00");
  });
});

describe("formatDurationText", () => {
  it("formats seconds, minutes, and hours in human-readable strings", () => {
    expect(formatDurationText(0)).toBe("0s");
    expect(formatDurationText(45)).toBe("45s");
    expect(formatDurationText(65)).toBe("1m 5s");
    expect(formatDurationText(125)).toBe("2m 5s");
    expect(formatDurationText(3600)).toBe("1h");
    expect(formatDurationText(3665)).toBe("1h 1m");
    expect(formatDurationText(7200)).toBe("2h");
  });
});

describe("useActiveMedia", () => {
  beforeEach(() => {
    sessionStorage.clear();
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("throws error when used outside ActiveMediaProvider", () => {
    expect(() => renderHook(() => useActiveMedia())).toThrow(
      "useActiveMedia must be used within an ActiveMediaProvider",
    );
  });

  it("provides initial inactive state when no session is active", () => {
    const { result } = renderHook(() => useActiveMedia(), {
      wrapper: createWrapper(),
    });

    expect(result.current.isPlaying).toBe(false);
    expect(result.current.session).toBeNull();
    expect(result.current.elapsedSeconds).toBe(0);
    expect(result.current.formattedElapsed).toBe("00:00");
  });

  it("updates active session and increments timer over time", () => {
    const { result } = renderHook(() => useActiveMedia(), {
      wrapper: createWrapper(),
    });

    const now = Date.now();

    act(() => {
      result.current.setActiveSession({
        mediaId: "game-123",
        launchId: "launch-456",
        agentId: "agent-789",
        startedAt: now,
      });
    });

    expect(result.current.isPlaying).toBe(true);
    expect(result.current.session?.mediaId).toBe("game-123");

    act(() => {
      vi.advanceTimersByTime(65000);
    });

    expect(result.current.elapsedSeconds).toBe(65);
    expect(result.current.formattedElapsed).toBe("01:05");

    act(() => {
      result.current.setActiveSession(null);
    });

    expect(result.current.isPlaying).toBe(false);
    expect(result.current.session).toBeNull();
    expect(result.current.elapsedSeconds).toBe(0);
  });
});
