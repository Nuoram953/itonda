export function isMacPlatform(): boolean {
  if (typeof window === "undefined" || typeof navigator === "undefined") {
    return false;
  }
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const navData = (navigator as any).userAgentData;
  if (navData?.platform) {
    return /mac/i.test(navData.platform);
  }
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const platform = (navigator as any).platform || navigator.userAgent || "";
  return /mac|iphone|ipad|ipod/i.test(platform);
}

export function parseShortcutCombo(combo: string) {
  const parts = combo
    .toLowerCase()
    .split("+")
    .map((s) => s.trim());

  const hasMod = parts.includes("mod") || parts.includes("cmdorctrl");
  const hasCtrl = parts.includes("ctrl") || parts.includes("control");
  const hasAlt = parts.includes("alt") || parts.includes("option");
  const hasShift = parts.includes("shift");
  const hasMeta =
    parts.includes("meta") || parts.includes("cmd") || parts.includes("command");

  const key =
    parts.find(
      (p) =>
        ![
          "mod",
          "cmdorctrl",
          "ctrl",
          "control",
          "alt",
          "option",
          "shift",
          "meta",
          "cmd",
          "command",
        ].includes(p),
    ) ?? "";

  return { hasMod, hasCtrl, hasAlt, hasShift, hasMeta, key };
}

export function matchesShortcut(event: KeyboardEvent, combo: string): boolean {
  if (!combo || !event) return false;

  const { hasMod, hasCtrl, hasAlt, hasShift, hasMeta, key } =
    parseShortcutCombo(combo);
  if (!key) return false;

  const hasModKey = event.metaKey || event.ctrlKey;
  if (hasMod) {
    if (!hasModKey) return false;
  } else {
    if (hasCtrl !== event.ctrlKey) return false;
    if (hasMeta !== event.metaKey) return false;
  }

  if (hasAlt !== event.altKey) return false;
  if (hasShift !== event.shiftKey) return false;

  const eventKey = event.key.toLowerCase();
  if (key === "escape" || key === "esc") {
    return eventKey === "escape" || eventKey === "esc";
  }
  if (key === "enter" || key === "return" || key === "↵") {
    return eventKey === "enter";
  }
  if (key === "tab") return eventKey === "tab";
  if (key === "space") {
    return eventKey === " " || eventKey === "space" || event.code === "Space";
  }

  return eventKey === key;
}

export function formatShortcutDisplay(
  combo: string,
  options?: { preferMacSymbols?: boolean },
): string {
  if (!combo) return "";
  if (["↑↓", "↵", "Esc"].includes(combo)) return combo;

  const isMac =
    options?.preferMacSymbols !== undefined
      ? options.preferMacSymbols
      : isMacPlatform() || true;

  return combo
    .split("+")
    .map((part) => {
      const lower = part.trim().toLowerCase();
      if (lower === "mod" || lower === "cmd" || lower === "meta") {
        return isMac ? "⌘" : "Ctrl+";
      }
      if (lower === "ctrl" || lower === "control") {
        return isMac ? "⌃" : "Ctrl+";
      }
      if (lower === "alt" || lower === "option") {
        return isMac ? "⌥" : "Alt+";
      }
      if (lower === "shift") {
        return isMac ? "⇧" : "Shift+";
      }
      if (lower === "enter" || lower === "return") return "↵";
      if (lower === "escape") return "Esc";
      if (lower === "tab") return "Tab";
      return part.toUpperCase();
    })
    .join("");
}

export function eventToShortcutCombo(event: KeyboardEvent): string | null {
  if (["Control", "Shift", "Alt", "Meta"].includes(event.key)) return null;

  const parts: string[] = [];
  if (event.metaKey || event.ctrlKey) parts.push("Mod");
  if (event.altKey) parts.push("Alt");
  if (event.shiftKey) parts.push("Shift");

  let key = event.key;
  if (key === " ") key = "Space";
  else if (key === "Escape") key = "Escape";
  else if (key.length === 1) key = key.toUpperCase();

  parts.push(key);
  return parts.join("+");
}

export function isEditableElement(target: EventTarget | null): boolean {
  if (!target || !(target instanceof HTMLElement)) return false;
  const tag = target.tagName.toLowerCase();
  return Boolean(
    tag === "input" ||
      tag === "textarea" ||
      tag === "select" ||
      target.isContentEditable,
  );
}
