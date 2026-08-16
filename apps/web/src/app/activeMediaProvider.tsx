import {
  useState,
  useEffect,
  useCallback,
  useMemo,
  type ReactNode,
} from "react";
import { useQuery } from "@tanstack/react-query";
import { getMediaByIdQueryOptions } from "@/features/details/api/get-media-id";
import { STORAGE_KEYS, safeStorage } from "@/utils/storage";
import { useElapsedTimer } from "@/hooks/use-elapsed-timer";
import {
  ActiveMediaContext,
  type ActiveMediaContextValue,
  type ActiveMediaSession,
} from "./activeMediaContext";

export function ActiveMediaProvider({ children }: { children: ReactNode }) {
  const [session, setSessionState] = useState<ActiveMediaSession | null>(() =>
    safeStorage.get<ActiveMediaSession | null>(
      STORAGE_KEYS.ACTIVE_MEDIA_SESSION,
      null,
      "session",
    ),
  );

  const startedAt = session?.startedAt ?? null;
  const { elapsedSeconds, formattedElapsed } = useElapsedTimer(startedAt);

  const setActiveSession = useCallback(
    (newSession: ActiveMediaSession | null) => {
      setSessionState(newSession);
      safeStorage.set(STORAGE_KEYS.ACTIVE_MEDIA_SESSION, newSession, "session");
    },
    [],
  );

  useEffect(() => {
    const handleStorage = (event: StorageEvent) => {
      if (event.key === STORAGE_KEYS.ACTIVE_MEDIA_SESSION) {
        if (event.newValue) {
          try {
            const parsed = JSON.parse(event.newValue) as ActiveMediaSession;
            setSessionState(parsed);
          } catch {
            // Ignore invalid JSON from storage event
          }
        } else {
          setSessionState(null);
        }
      }
    };

    window.addEventListener("storage", handleStorage);
    return () => window.removeEventListener("storage", handleStorage);
  }, []);

  const mediaId = session?.mediaId ?? "";

  const { data: media, isLoading: isLoadingMedia } = useQuery({
    ...getMediaByIdQueryOptions(mediaId),
    enabled: Boolean(mediaId),
  });

  const contextValue = useMemo<ActiveMediaContextValue>(
    () => ({
      session,
      media,
      isLoadingMedia,
      isPlaying: Boolean(session),
      elapsedSeconds,
      formattedElapsed,
      setActiveSession,
    }),
    [
      session,
      media,
      isLoadingMedia,
      elapsedSeconds,
      formattedElapsed,
      setActiveSession,
    ],
  );

  return (
    <ActiveMediaContext.Provider value={contextValue}>
      {children}
    </ActiveMediaContext.Provider>
  );
}
