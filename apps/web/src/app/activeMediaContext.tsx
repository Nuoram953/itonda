import { createContext, useContext } from "react";
import type { components } from "@/api/generated.d";
import { formatElapsedSeconds, formatDurationText } from "@/utils/datetime";
import { STORAGE_KEYS } from "@/utils/storage";

export { formatElapsedSeconds, formatDurationText };

export interface ActiveMediaSession {
  mediaId: string;
  launchId: string;
  agentId: string;
  startedAt: number;
}

export interface ActiveMediaContextValue {
  session: ActiveMediaSession | null;
  media: components["schemas"]["Media"] | undefined;
  isLoadingMedia: boolean;
  isPlaying: boolean;
  elapsedSeconds: number;
  formattedElapsed: string;
  setActiveSession: (session: ActiveMediaSession | null) => void;
}

export const ACTIVE_MEDIA_STORAGE_KEY = STORAGE_KEYS.ACTIVE_MEDIA_SESSION;

export const ActiveMediaContext = createContext<ActiveMediaContextValue | null>(
  null,
);

export function useActiveMedia(): ActiveMediaContextValue {
  const context = useContext(ActiveMediaContext);
  if (!context) {
    throw new Error(
      "useActiveMedia must be used within an ActiveMediaProvider",
    );
  }
  return context;
}
