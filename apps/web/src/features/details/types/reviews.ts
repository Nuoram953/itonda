export type ReviewVerdict =
  | "masterpiece"
  | "recommended"
  | "neutral"
  | "do_not_recommend";

export type ThoughtCategory =
  | "audio"
  | "gameplay"
  | "story"
  | "visuals"
  | "performance"
  | "general";

export interface GameReview {
  media_id: string;
  verdict: ReviewVerdict;
  summary: string | null;
  created_at: string;
  updated_at: string;
}

export interface GameThought {
  id: string;
  media_id: string;
  title: string;
  content: string;
  category: string;
  playtime_minutes: number | null;
  created_at: string;
  updated_at: string;
}

export interface GameReviewOverview {
  review: GameReview | null;
  thoughts: GameThought[];
}

export interface UpsertReviewPayload {
  verdict: ReviewVerdict;
  summary?: string | null;
}

export interface CreateThoughtPayload {
  title: string;
  content: string;
  category?: string;
  playtime_minutes?: number | null;
}

export interface UpdateThoughtPayload {
  title: string;
  content: string;
  category?: string;
}
