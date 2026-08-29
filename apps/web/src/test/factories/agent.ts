import type { components } from "@/api/generated.d";

export type Agent = components["schemas"]["Agent"];

export function createAgent(overrides?: Partial<Agent>): Agent {
  return {
    id: "agent-1",
    name: "Desktop-Agent",
    hostname: "desktop-pc",
    platform: "linux",
    is_connected: true,
    created_at: 1000,
    agent_version: null,
    connected_at: null,
    ip_address: null,
    last_seen_at: null,
    ...overrides,
  };
}
