import { useRef, useCallback, useEffect } from "react";

type UseAutoSaveOptions = {
  debounceMs?: number;
};

/**
 * Hook for managing debounced and instant auto-save operations with automatic timer cleanup.
 */
export function useAutoSave(
  onSave: () => void,
  { debounceMs = 500 }: UseAutoSaveOptions = {},
) {
  const timerRef = useRef<number | null>(null);

  const cancelPendingSave = useCallback(() => {
    if (timerRef.current) {
      clearTimeout(timerRef.current);
      timerRef.current = null;
    }
  }, []);

  const triggerSave = useCallback(
    (instant = false) => {
      cancelPendingSave();

      if (instant) {
        onSave();
      } else {
        timerRef.current = setTimeout(() => {
          onSave();
        }, debounceMs);
      }
    },
    [onSave, debounceMs, cancelPendingSave],
  );

  useEffect(() => {
    return cancelPendingSave;
  }, [cancelPendingSave]);

  return {
    triggerSave,
    cancelPendingSave,
  };
}
