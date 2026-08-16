import { useState, useEffect, useCallback } from "react";

export const STORAGE_KEYS = {
  ACTIVE_MEDIA_SESSION: "itonda_active_media_session",
  THEME: "itonda_ui_theme",
} as const;

export type StorageKey = (typeof STORAGE_KEYS)[keyof typeof STORAGE_KEYS] | string;
export type StorageType = "session" | "local";

function getStorage(type: StorageType = "session"): Storage | null {
  if (typeof window === "undefined") return null;
  try {
    return type === "local" ? window.localStorage : window.sessionStorage;
  } catch {
    return null;
  }
}

export const safeStorage = {
  get<T>(key: StorageKey, fallback: T, type: StorageType = "session"): T {
    const storage = getStorage(type);
    if (!storage) return fallback;

    try {
      const item = storage.getItem(key);
      if (item === null) return fallback;
      return JSON.parse(item) as T;
    } catch (e) {
      console.warn(`[storage] Failed to get item "${key}"`, e);
      return fallback;
    }
  },

  set<T>(key: StorageKey, value: T, type: StorageType = "session"): boolean {
    const storage = getStorage(type);
    if (!storage) return false;

    try {
      if (value === null || value === undefined) {
        storage.removeItem(key);
      } else {
        storage.setItem(key, JSON.stringify(value));
      }
      return true;
    } catch (e) {
      console.warn(`[storage] Failed to set item "${key}"`, e);
      return false;
    }
  },

  remove(key: StorageKey, type: StorageType = "session"): boolean {
    const storage = getStorage(type);
    if (!storage) return false;

    try {
      storage.removeItem(key);
      return true;
    } catch (e) {
      console.warn(`[storage] Failed to remove item "${key}"`, e);
      return false;
    }
  },

  clear(type: StorageType = "session"): boolean {
    const storage = getStorage(type);
    if (!storage) return false;

    try {
      storage.clear();
      return true;
    } catch (e) {
      console.warn("[storage] Failed to clear storage", e);
      return false;
    }
  },
};

export function useStorageState<T>(
  key: StorageKey,
  initialValue: T,
  options: { type?: StorageType; syncTabs?: boolean } = {},
) {
  const { type = "session", syncTabs = true } = options;

  const [state, setState] = useState<T>(() =>
    safeStorage.get<T>(key, initialValue, type),
  );

  const setValue = useCallback(
    (newValue: T | ((prev: T) => T)) => {
      setState((current) => {
        const resolvedValue =
          typeof newValue === "function"
            ? (newValue as (prev: T) => T)(current)
            : newValue;

        safeStorage.set<T>(key, resolvedValue, type);
        return resolvedValue;
      });
    },
    [key, type],
  );

  useEffect(() => {
    if (!syncTabs || typeof window === "undefined") return;

    const handleStorage = (event: StorageEvent) => {
      if (event.key === key) {
        if (event.newValue !== null) {
          try {
            setState(JSON.parse(event.newValue) as T);
          } catch {
            // Ignore parse errors
          }
        } else {
          setState(initialValue);
        }
      }
    };

    window.addEventListener("storage", handleStorage);
    return () => window.removeEventListener("storage", handleStorage);
  }, [key, initialValue, syncTabs]);

  return [state, setValue] as const;
}
