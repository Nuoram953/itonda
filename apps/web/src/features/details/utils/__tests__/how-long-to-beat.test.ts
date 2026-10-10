import { describe, it, expect } from "vitest";
import { createMedia } from "@/test/test-utils";
import {
  formatHltbHours,
  normalizeHltbData,
  getMediaHltbData,
} from "../how-long-to-beat";

describe("how-long-to-beat utils", () => {
  describe("formatHltbHours", () => {
    it("formats whole hours correctly", () => {
      expect(formatHltbHours(41)).toBe("41 Hours");
      expect(formatHltbHours(1)).toBe("1 Hour");
    });

    it("formats fractional hours correctly", () => {
      expect(formatHltbHours(25.5)).toBe("25.5 Hours");
    });

    it("formats sub-hour durations in minutes", () => {
      expect(formatHltbHours(0.5)).toBe("30 mins");
    });

    it("returns placeholder for null or invalid inputs", () => {
      expect(formatHltbHours(null)).toBe("--");
      expect(formatHltbHours(undefined)).toBe("--");
      expect(formatHltbHours(0)).toBe("--");
      expect(formatHltbHours(-5)).toBe("--");
    });
  });

  describe("normalizeHltbData", () => {
    it("handles snake_case format", () => {
      const result = normalizeHltbData({
        main_story: 20,
        main_extra: 40,
        completionist: 80,
        all_styles: 35,
      });

      expect(result).toEqual({
        mainStory: 20,
        mainExtra: 40,
        completionist: 80,
        allStyles: 35,
      });
    });

    it("handles _hours suffix format", () => {
      const result = normalizeHltbData({
        main_story_hours: 15,
        main_extra_hours: 30,
        completionist_hours: 60,
        all_styles_hours: 25,
      });

      expect(result).toEqual({
        mainStory: 15,
        mainExtra: 30,
        completionist: 60,
        allStyles: 25,
      });
    });

    it("returns null for empty payload", () => {
      expect(normalizeHltbData(null)).toBeNull();
      expect(normalizeHltbData({})).toBeNull();
    });
  });

  describe("getMediaHltbData", () => {
    it("prioritizes data attached to media payload", () => {
      const media = createMedia({
        title: "Some Random Game",
      });
      (media as unknown as Record<string, unknown>).how_long_to_beat = {
        main_story: 12,
        main_extra: 24,
      };

      const result = getMediaHltbData(media);
      expect(result?.mainStory).toBe(12);
      expect(result?.mainExtra).toBe(24);
    });

    it("falls back to known database entries for game titles", () => {
      const media = createMedia({
        title: "Kingdom Come: Deliverance",
        media_type: "game",
      });

      const result = getMediaHltbData(media);
      expect(result?.mainStory).toBe(41);
      expect(result?.mainExtra).toBe(81);
      expect(result?.completionist).toBe(129);
      expect(result?.allStyles).toBe(72);
    });
  });
});
