import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import {
  parseBackendTimestamp,
  formatDate,
  formatDateTime,
  formatTime,
  formatLastPlayedDate,
  formatRelativeTime,
  formatPlaytimeHours,
  formatPlaytime,
  formatDuration,
  formatElapsedSeconds,
  formatDurationText,
} from "../datetime";


describe("datetime utilities", () => {
  describe("parseBackendTimestamp", () => {
    it("returns null for empty/invalid inputs", () => {
      expect(parseBackendTimestamp(null)).toBeNull();
      expect(parseBackendTimestamp(undefined)).toBeNull();
      expect(parseBackendTimestamp("")).toBeNull();
      expect(parseBackendTimestamp(0)).toBeNull();
      expect(parseBackendTimestamp(-100)).toBeNull();
      expect(parseBackendTimestamp("invalid-date")).toBeNull();
      expect(parseBackendTimestamp(NaN)).toBeNull();
    });

    it("parses numeric UNIX timestamps in seconds correctly", () => {
      const date = parseBackendTimestamp(1700000000);
      expect(date).not.toBeNull();
      expect(date?.getTime()).toBe(1700000000 * 1000);
    });

    it("parses numeric UNIX timestamps in milliseconds correctly", () => {
      const ms = 1700000000000;
      const date = parseBackendTimestamp(ms);
      expect(date).not.toBeNull();
      expect(date?.getTime()).toBe(ms);
    });

    it("parses ISO strings correctly", () => {
      const iso = "2023-11-14T22:13:20.000Z";
      const date = parseBackendTimestamp(iso);
      expect(date).not.toBeNull();
      expect(date?.toISOString()).toBe(iso);
    });

    it("handles Date objects directly", () => {
      const now = new Date();
      expect(parseBackendTimestamp(now)).toBe(now);
      expect(parseBackendTimestamp(new Date("invalid"))).toBeNull();
    });
  });

  describe("formatDate", () => {
    it("formats valid timestamps to date string", () => {
      const formatted = formatDate(1700000000);
      expect(formatted).toContain("2023");
    });

    it("returns fallback for null/undefined/invalid values", () => {
      expect(formatDate(null)).toBe("");
      expect(formatDate(null, undefined, "N/A")).toBe("N/A");
      expect(formatDate(undefined, undefined, "-")).toBe("-");
    });
  });

  describe("formatDateTime", () => {
    it("formats timestamp with both date and time", () => {
      const formatted = formatDateTime(1700000000);
      expect(formatted).toContain("2023");
    });

    it("returns fallback for missing values", () => {
      expect(formatDateTime(null, undefined, "Never")).toBe("Never");
    });
  });

  describe("formatTime", () => {
    it("formats timestamp to time string", () => {
      const formatted = formatTime(1700000000);
      expect(formatted).toBeDefined();
      expect(formatted.length).toBeGreaterThan(0);
    });

    it("returns fallback for missing values", () => {
      expect(formatTime(null, undefined, "--:--")).toBe("--:--");
    });
  });

  describe("formatLastPlayedDate", () => {
    it("returns 'Never played' for null/undefined/0", () => {
      expect(formatLastPlayedDate(null)).toBe("Never played");
      expect(formatLastPlayedDate(undefined)).toBe("Never played");
      expect(formatLastPlayedDate(0)).toBe("Never played");
    });

    it("allows custom fallback", () => {
      expect(formatLastPlayedDate(null, "Not yet played")).toBe("Not yet played");
    });

    it("formats valid last played timestamp", () => {
      const formatted = formatLastPlayedDate(1700000000);
      expect(formatted).toContain("2023");
    });
  });

  describe("formatRelativeTime", () => {
    beforeEach(() => {
      vi.useFakeTimers();
      vi.setSystemTime(new Date("2023-11-15T12:00:00Z"));
    });

    afterEach(() => {
      vi.useRealTimers();
    });

    it("returns null or fallback for empty inputs", () => {
      expect(formatRelativeTime(null)).toBeNull();
      expect(formatRelativeTime(undefined, "unknown")).toBe("unknown");
    });

    it("returns 'just now' for recent timestamps", () => {
      const now = new Date("2023-11-15T12:00:00Z").getTime();
      expect(formatRelativeTime(now - 10000)).toBe("just now");
    });

    it("returns minutes ago", () => {
      const now = new Date("2023-11-15T12:00:00Z").getTime();
      expect(formatRelativeTime(now - 5 * 60 * 1000)).toBe("5m ago");
    });

    it("returns hours ago", () => {
      const now = new Date("2023-11-15T12:00:00Z").getTime();
      expect(formatRelativeTime(now - 3 * 3600 * 1000)).toBe("3h ago");
    });

    it("returns days ago", () => {
      const now = new Date("2023-11-15T12:00:00Z").getTime();
      expect(formatRelativeTime(now - 4 * 86400 * 1000)).toBe("4d ago");
    });

    it("returns months ago", () => {
      const now = new Date("2023-11-15T12:00:00Z").getTime();
      expect(formatRelativeTime(now - 65 * 86400 * 1000)).toBe("2mo ago");
    });

    it("returns years ago", () => {
      const now = new Date("2023-11-15T12:00:00Z").getTime();
      expect(formatRelativeTime(now - 400 * 86400 * 1000)).toBe("1y ago");
    });
  });

  describe("formatPlaytimeHours", () => {
    it("returns 0 for null, undefined, or zero", () => {
      expect(formatPlaytimeHours(null)).toBe(0);
      expect(formatPlaytimeHours(undefined)).toBe(0);
      expect(formatPlaytimeHours(0)).toBe(0);
      expect(formatPlaytimeHours(-10)).toBe(0);
    });

    it("calculates floor hours correctly", () => {
      expect(formatPlaytimeHours(59)).toBe(0);
      expect(formatPlaytimeHours(60)).toBe(1);
      expect(formatPlaytimeHours(125)).toBe(2);
      expect(formatPlaytimeHours(300)).toBe(5);
    });
  });

  describe("formatPlaytime", () => {
    it("formats in default 'hours' mode", () => {
      expect(formatPlaytime(0)).toBe("0 Hours");
      expect(formatPlaytime(60)).toBe("1 Hour");
      expect(formatPlaytime(125)).toBe("2 Hours");
      expect(formatPlaytime(null)).toBe("0 Hours");
    });

    it("formats in 'compact' mode", () => {
      expect(formatPlaytime(0, { mode: "compact" })).toBe("0m");
      expect(formatPlaytime(45, { mode: "compact" })).toBe("45m");
      expect(formatPlaytime(60, { mode: "compact" })).toBe("1h");
      expect(formatPlaytime(125, { mode: "compact" })).toBe("2h 5m");
    });

    it("formats in 'detailed' mode", () => {
      expect(formatPlaytime(0, { mode: "detailed" })).toBe("0 mins");
      expect(formatPlaytime(45, { mode: "detailed" })).toBe("45 mins");
      expect(formatPlaytime(60, { mode: "detailed" })).toBe("1 hr");
      expect(formatPlaytime(125, { mode: "detailed" })).toBe("2 hrs 5 mins");
    });

    it("formats in 'approx' mode", () => {
      expect(formatPlaytime(0, { mode: "approx" })).toBe("0 hours");
      expect(formatPlaytime(45, { mode: "approx" })).toBe("< 1 hour");
      expect(formatPlaytime(60, { mode: "approx" })).toBe("1 hour");
      expect(formatPlaytime(180, { mode: "approx" })).toBe("3 hours");
    });

    it("supports custom fallback", () => {
      expect(formatPlaytime(null, { fallback: "No playtime" })).toBe("No playtime");
    });
  });

  describe("formatDuration", () => {
    it("formats durations in compact form", () => {
      expect(formatDuration(0)).toBe("0m");
      expect(formatDuration(45)).toBe("45m");
      expect(formatDuration(125)).toBe("2h 5m");
    });
  });

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
});

