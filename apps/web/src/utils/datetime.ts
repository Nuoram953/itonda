export function parseBackendTimestamp(
  timestamp?: number | string | Date | null,
): Date | null {
  if (timestamp === null || timestamp === undefined || timestamp === "") {
    return null;
  }

  if (timestamp instanceof Date) {
    return isNaN(timestamp.getTime()) ? null : timestamp;
  }

  if (typeof timestamp === "string") {
    const num = Number(timestamp);
    if (!isNaN(num)) {
      return parseBackendTimestamp(num);
    }

    const parsedDate = new Date(timestamp);
    return isNaN(parsedDate.getTime()) ? null : parsedDate;
  }

  if (typeof timestamp === "number") {
    if (isNaN(timestamp) || timestamp <= 0) {
      return null;
    }

    const ms = timestamp < 1e11 ? timestamp * 1000 : timestamp;
    const parsedDate = new Date(ms);
    return isNaN(parsedDate.getTime()) ? null : parsedDate;
  }

  return null;
}

export type DateFormatOptions = Intl.DateTimeFormatOptions;

export function formatDate(
  timestamp?: number | string | Date | null,
  options: DateFormatOptions = {
    year: "numeric",
    month: "short",
    day: "numeric",
  },
  fallback = "",
): string {
  const date = parseBackendTimestamp(timestamp);
  if (!date) return fallback;
  return date.toLocaleDateString(undefined, options);
}

export function formatDateTime(
  timestamp?: number | string | Date | null,
  options: DateFormatOptions = {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "numeric",
    minute: "numeric",
  },
  fallback = "",
): string {
  const date = parseBackendTimestamp(timestamp);
  if (!date) return fallback;
  return date.toLocaleString(undefined, options);
}

export function formatTime(
  timestamp?: number | string | Date | null,
  options: DateFormatOptions = {
    hour: "numeric",
    minute: "numeric",
  },
  fallback = "",
): string {
  const date = parseBackendTimestamp(timestamp);
  if (!date) return fallback;
  return date.toLocaleTimeString(undefined, options);
}

export function formatLastPlayedDate(
  lastPlayedAt?: number | string | Date | null,
  fallback = "Never played",
  options: DateFormatOptions = {
    year: "numeric",
    month: "short",
    day: "numeric",
  },
): string {
  return formatDate(lastPlayedAt, options, fallback);
}

export function formatRelativeTime(
  timestamp?: number | string | Date | null,
  fallback: string | null = null,
): string | null {
  const date = parseBackendTimestamp(timestamp);
  if (!date) return fallback;

  const diffSec = Math.floor((Date.now() - date.getTime()) / 1000);
  if (diffSec < 0 || diffSec < 60) return "just now";

  const diffMin = Math.floor(diffSec / 60);
  if (diffMin < 60) return `${diffMin}m ago`;

  const diffHours = Math.floor(diffMin / 60);
  if (diffHours < 24) return `${diffHours}h ago`;

  const diffDays = Math.floor(diffHours / 24);
  if (diffDays < 30) return `${diffDays}d ago`;

  const diffMonths = Math.floor(diffDays / 30);
  if (diffMonths < 12) return `${diffMonths}mo ago`;

  const diffYears = Math.floor(diffDays / 365);
  return `${diffYears}y ago`;
}

export function formatPlaytimeHours(playtimeMinutes?: number | null): number {
  if (!playtimeMinutes || playtimeMinutes <= 0 || isNaN(playtimeMinutes)) {
    return 0;
  }
  return Math.floor(playtimeMinutes / 60);
}

export type PlaytimeFormatMode = "hours" | "compact" | "detailed" | "approx";

export type PlaytimeFormatOptions = {
  mode?: PlaytimeFormatMode;
  fallback?: string;
};

export function formatPlaytime(
  playtimeMinutes?: number | null,
  options: PlaytimeFormatOptions = {},
): string {
  const { mode = "hours", fallback } = options;

  if (
    playtimeMinutes === null ||
    playtimeMinutes === undefined ||
    isNaN(playtimeMinutes) ||
    playtimeMinutes < 0
  ) {
    if (fallback !== undefined) return fallback;
    switch (mode) {
      case "compact":
        return "0m";
      case "detailed":
        return "0 mins";
      case "approx":
        return "0 hours";
      case "hours":
      default:
        return "0 Hours";
    }
  }

  const totalMinutes = Math.floor(playtimeMinutes);
  const hours = Math.floor(totalMinutes / 60);
  const minutes = totalMinutes % 60;

  switch (mode) {
    case "compact": {
      if (hours === 0 && minutes === 0) return "0m";
      if (hours === 0) return `${minutes}m`;
      if (minutes === 0) return `${hours}h`;
      return `${hours}h ${minutes}m`;
    }

    case "detailed": {
      if (hours === 0 && minutes === 0) return "0 mins";
      if (hours === 0) return `${minutes} mins`;
      if (minutes === 0) return `${hours} ${hours === 1 ? "hr" : "hrs"}`;
      return `${hours} ${hours === 1 ? "hr" : "hrs"} ${minutes} mins`;
    }

    case "approx": {
      if (totalMinutes === 0) return "0 hours";
      if (hours === 0) return "< 1 hour";
      return `${hours} ${hours === 1 ? "hour" : "hours"}`;
    }

    case "hours":
    default: {
      return `${hours} ${hours === 1 ? "Hour" : "Hours"}`;
    }
  }
}

export function formatDuration(playtimeMinutes?: number | null): string {
  return formatPlaytime(playtimeMinutes, { mode: "compact" });
}

export function formatElapsedSeconds(totalSeconds: number): string {
  const safeSeconds = Math.max(0, Math.floor(totalSeconds));
  const hours = Math.floor(safeSeconds / 3600);
  const minutes = Math.floor((safeSeconds % 3600) / 60);
  const seconds = safeSeconds % 60;

  const pad = (n: number) => n.toString().padStart(2, "0");

  if (hours > 0) {
    return `${hours}:${pad(minutes)}:${pad(seconds)}`;
  }
  return `${pad(minutes)}:${pad(seconds)}`;
}

export function formatDurationText(totalSeconds: number): string {
  const safeSeconds = Math.max(0, Math.floor(totalSeconds));
  const hours = Math.floor(safeSeconds / 3600);
  const minutes = Math.floor((safeSeconds % 3600) / 60);
  const seconds = safeSeconds % 60;

  if (hours > 0) {
    return minutes > 0 ? `${hours}h ${minutes}m` : `${hours}h`;
  }
  if (minutes > 0) {
    return `${minutes}m ${seconds}s`;
  }
  return `${seconds}s`;
}

