import { useSyncExternalStore, useMemo } from "react";
import { formatElapsedSeconds } from "@/utils/datetime";

export function useElapsedTimer(startedAt: number | null) {
  const elapsedSeconds = useSyncExternalStore(
    (onStoreChange) => {
      if (!startedAt) {
        return () => {};
      }
      const interval = setInterval(onStoreChange, 1000);
      return () => clearInterval(interval);
    },
    () =>
      startedAt
        ? Math.max(0, Math.floor((Date.now() - startedAt) / 1000))
        : 0,
    () => 0,
  );

  const formattedElapsed = useMemo(
    () => formatElapsedSeconds(elapsedSeconds),
    [elapsedSeconds],
  );

  return { elapsedSeconds, formattedElapsed };
}
