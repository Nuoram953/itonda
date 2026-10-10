import type { components } from "@/api/generated.d";
import type {
  HowLongToBeatData,
  NormalizedHowLongToBeat,
} from "../types/how-long-to-beat";

/**
 * Format hours into human-readable duration, e.g. "41 Hours", "25.5 Hours", "45 mins".
 */
export function formatHltbHours(hours?: number | null): string {
  if (hours === null || hours === undefined || isNaN(hours) || hours <= 0) {
    return "--";
  }

  if (hours < 1) {
    const mins = Math.round(hours * 60);
    return `${mins} mins`;
  }

  const rounded = Number(hours.toFixed(1));
  const isWhole = rounded % 1 === 0;
  const valueStr = isWhole ? String(Math.round(rounded)) : String(rounded);

  return `${valueStr} ${rounded === 1 ? "Hour" : "Hours"}`;
}

/**
 * Normalize HowLongToBeat payload to standard field structure.
 */
export function normalizeHltbData(
  data?: HowLongToBeatData | null,
): NormalizedHowLongToBeat | null {
  if (!data) return null;

  const mainStory = data.main_story ?? data.main_story_hours ?? null;
  const mainExtra = data.main_extra ?? data.main_extra_hours ?? null;
  const completionist =
    data.completionist ?? data.completionist_hours ?? null;
  const allStyles = data.all_styles ?? data.all_styles_hours ?? null;

  if (
    mainStory === null &&
    mainExtra === null &&
    completionist === null &&
    allStyles === null
  ) {
    return null;
  }

  return {
    mainStory,
    mainExtra,
    completionist,
    allStyles,
  };
}

/**
 * Known fallback data for common games while backend metadata worker is in development.
 */
const MOCK_HLTB_DATABASE: Record<string, HowLongToBeatData> = {
  "kingdom come": {
    main_story: 41,
    main_extra: 81,
    completionist: 129,
    all_styles: 72,
  },
  "elden ring": {
    main_story: 58,
    main_extra: 101,
    completionist: 133,
    all_styles: 78,
  },
  "cyberpunk": {
    main_story: 25,
    main_extra: 60,
    completionist: 104,
    all_styles: 45,
  },
  witcher: {
    main_story: 51,
    main_extra: 103,
    completionist: 173,
    all_styles: 75,
  },
  hades: {
    main_story: 22,
    main_extra: 48,
    completionist: 96,
    all_styles: 35,
  },
};

/**
 * Extract HLTB data from media payload, falling back to mock estimation if backend has not populated it yet.
 */
export function getMediaHltbData(
  media: components["schemas"]["Media"],
): NormalizedHowLongToBeat | null {
  // Check if backend payload includes HLTB data
  const rawFromDetails = (media.details as unknown as Record<string, unknown>)
    ?.how_long_to_beat as HowLongToBeatData | undefined;
  const rawFromMedia = (media as unknown as Record<string, unknown>)
    ?.how_long_to_beat as HowLongToBeatData | undefined;

  const normalizedReal = normalizeHltbData(rawFromDetails ?? rawFromMedia);
  if (normalizedReal) {
    return normalizedReal;
  }

  // Fallback for frontend preview while backend is under construction
  const lowerTitle = media.title.toLowerCase();
  for (const [key, data] of Object.entries(MOCK_HLTB_DATABASE)) {
    if (lowerTitle.includes(key)) {
      return normalizeHltbData(data);
    }
  }

  // Generic fallback based on media_type if it is a game
  if (media.media_type === "game") {
    return {
      mainStory: 25,
      mainExtra: 55,
      completionist: 100,
      allStyles: 48,
    };
  }

  return null;
}
