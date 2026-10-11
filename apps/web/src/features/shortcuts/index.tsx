/* eslint-disable react-refresh/only-export-components */
import {
  createContext,
  useContext,
  useState,
  useCallback,
  useRef,
  useEffect,
  type ReactNode,
} from "react";
import {
  matchesShortcut,
  formatShortcutDisplay,
  isEditableElement,
} from "./utils";

export type ShortcutActionId =
  | "search.open"
  | "search.switchAction"
  | "search.navigate"
  | "search.select"
  | "search.close"
  | "sidebar.toggle"
  | (string & {});

export interface ShortcutDefinition {
  id: ShortcutActionId;
  label: string;
  defaultCombo: string;
}

export type ShortcutBindings = Record<string, string>;

export interface UseShortcutOptions {
  enabled?: boolean;
  preventDefault?: boolean;
  stopPropagation?: boolean;
  target?: HTMLElement | Window | Document | null;
  ignoreInputs?: boolean;
}

export interface ShortcutsContextValue {
  bindings: ShortcutBindings;
  definitions: Record<string, ShortcutDefinition>;
  getShortcut: (actionId: ShortcutActionId) => string;
  getDisplayString: (actionId: ShortcutActionId) => string;
  updateShortcut: (actionId: ShortcutActionId, combo: string) => void;
  resetShortcut: (actionId: ShortcutActionId) => void;
  resetAll: () => void;
  matches: (event: KeyboardEvent, actionId: ShortcutActionId) => boolean;
}

export const SHORTCUT_STORAGE_KEY = "itonda:shortcuts";

export const DEFAULT_SHORTCUTS: Record<string, ShortcutDefinition> = {
  "search.open": {
    id: "search.open",
    label: "Search",
    defaultCombo: "Mod+K",
  },
  "search.switchAction": {
    id: "search.switchAction",
    label: "Switch action",
    defaultCombo: "Tab",
  },
  "search.navigate": {
    id: "search.navigate",
    label: "Navigate",
    defaultCombo: "↑↓",
  },
  "search.select": {
    id: "search.select",
    label: "Select",
    defaultCombo: "↵",
  },
  "search.close": {
    id: "search.close",
    label: "Close",
    defaultCombo: "Esc",
  },
  "sidebar.toggle": {
    id: "sidebar.toggle",
    label: "Toggle sidebar",
    defaultCombo: "Mod+B",
  },
};

const DEFAULT_BINDINGS: ShortcutBindings = Object.fromEntries(
  Object.entries(DEFAULT_SHORTCUTS).map(([id, def]) => [id, def.defaultCombo]),
);

const fallbackContextValue: ShortcutsContextValue = {
  bindings: DEFAULT_BINDINGS,
  definitions: DEFAULT_SHORTCUTS,
  getShortcut: (id) => DEFAULT_BINDINGS[id] ?? "",
  getDisplayString: (id) => formatShortcutDisplay(DEFAULT_BINDINGS[id] ?? ""),
  updateShortcut: () => {},
  resetShortcut: () => {},
  resetAll: () => {},
  matches: (e, id) => matchesShortcut(e, DEFAULT_BINDINGS[id] ?? ""),
};

export const ShortcutsContext =
  createContext<ShortcutsContextValue>(fallbackContextValue);

export function ShortcutsProvider({
  children,
  storageKey = SHORTCUT_STORAGE_KEY,
}: {
  children: ReactNode;
  storageKey?: string;
}) {
  const [bindings, setBindings] = useState<ShortcutBindings>(() => {
    if (typeof window !== "undefined" && window.localStorage) {
      try {
        const stored = window.localStorage.getItem(storageKey);
        if (stored) return { ...DEFAULT_BINDINGS, ...JSON.parse(stored) };
      } catch {
        // Fallback to defaults
      }
    }
    return DEFAULT_BINDINGS;
  });

  const updateShortcut = useCallback(
    (actionId: ShortcutActionId, combo: string) => {
      setBindings((prev) => {
        const next = { ...prev, [actionId]: combo.trim() };
        try {
          window.localStorage?.setItem(storageKey, JSON.stringify(next));
        } catch {
          // Ignore storage errors
        }
        return next;
      });
    },
    [storageKey],
  );

  const resetShortcut = useCallback(
    (actionId: ShortcutActionId) => {
      updateShortcut(actionId, DEFAULT_SHORTCUTS[actionId]?.defaultCombo ?? "");
    },
    [updateShortcut],
  );

  const resetAll = useCallback(() => {
    setBindings(DEFAULT_BINDINGS);
    try {
      window.localStorage?.setItem(
        storageKey,
        JSON.stringify(DEFAULT_BINDINGS),
      );
    } catch {
      // Ignore storage errors
    }
  }, [storageKey]);

  const getShortcut = useCallback(
    (id: ShortcutActionId) =>
      bindings[id] ?? DEFAULT_SHORTCUTS[id]?.defaultCombo ?? "",
    [bindings],
  );

  const getDisplayString = useCallback(
    (id: ShortcutActionId) => formatShortcutDisplay(getShortcut(id)),
    [getShortcut],
  );

  const matches = useCallback(
    (event: KeyboardEvent, id: ShortcutActionId) =>
      matchesShortcut(event, getShortcut(id)),
    [getShortcut],
  );

  return (
    <ShortcutsContext.Provider
      value={{
        bindings,
        definitions: DEFAULT_SHORTCUTS,
        getShortcut,
        getDisplayString,
        updateShortcut,
        resetShortcut,
        resetAll,
        matches,
      }}
    >
      {children}
    </ShortcutsContext.Provider>
  );
}

export function useShortcuts(): ShortcutsContextValue {
  return useContext(ShortcutsContext);
}

export function useShortcut(
  actionId: ShortcutActionId,
  handler: (event: KeyboardEvent) => void,
  options: UseShortcutOptions = {},
) {
  const {
    enabled = true,
    preventDefault = true,
    stopPropagation = false,
    target,
    ignoreInputs = true,
  } = options;

  const { matches } = useShortcuts();
  const savedHandler = useRef(handler);

  useEffect(() => {
    savedHandler.current = handler;
  }, [handler]);

  useEffect(() => {
    if (!enabled) return;

    const el = target ?? (typeof window !== "undefined" ? window : null);
    if (!el) return;

    const onKeyDown = (event: Event) => {
      const e = event as KeyboardEvent;
      if (ignoreInputs && isEditableElement(e.target as HTMLElement)) return;

      if (matches(e, actionId)) {
        if (preventDefault) e.preventDefault();
        if (stopPropagation) e.stopPropagation();
        savedHandler.current(e);
      }
    };

    el.addEventListener("keydown", onKeyDown as EventListener);
    return () => el.removeEventListener("keydown", onKeyDown as EventListener);
  }, [
    actionId,
    matches,
    enabled,
    preventDefault,
    stopPropagation,
    target,
    ignoreInputs,
  ]);
}

export * from "./utils";
