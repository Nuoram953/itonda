import type { ActiveMediaSession } from "@/app/activeMediaContext";

export function createActiveMediaSession(
  overrides?: Partial<ActiveMediaSession>,
): ActiveMediaSession {
  return {
    mediaId: "media-1",
    launchId: "launch-1",
    agentId: "agent-1",
    startedAt: 1700000000000,
    ...overrides,
  };
}
