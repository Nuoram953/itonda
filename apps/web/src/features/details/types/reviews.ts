import type { components } from "@/api/generated.d";

export type ReviewVerdict = components["schemas"]["ReviewVerdict"];

export type ThoughtCategory =
  | "audio"
  | "gameplay"
  | "story"
  | "visuals"
  | "performance"
  | "general";

export type GameReview = components["schemas"]["GameReview"];
export type GameThought = components["schemas"]["GameThought"];
export type GameReviewOverview = components["schemas"]["GameReviewOverview"];
export type UpsertReviewPayload = components["schemas"]["UpsertReviewPayload"];
export type CreateThoughtPayload = components["schemas"]["CreateThoughtPayload"];
export type UpdateThoughtPayload = components["schemas"]["UpdateThoughtPayload"];
