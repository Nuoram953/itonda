import { useQuery, queryOptions } from "@tanstack/react-query";
import { api } from "@/lib/api-client";

export interface DirectoryEntry {
  name: string;
  path: string;
  is_dir: boolean;
}

export interface AgentDirectoryListingResponse {
  path: string;
  parent: string | null;
  entries: DirectoryEntry[];
  error: string | null;
}

export const listAgentFiles = (params: {
  agentId?: string;
  path?: string;
}): Promise<AgentDirectoryListingResponse> => {
  const search = new URLSearchParams();
  if (params.agentId) search.set("agent_id", params.agentId);
  if (params.path) search.set("path", params.path);
  const qs = search.toString();
  return api.get(`/agents/fs${qs ? `?${qs}` : ""}`);
};

export const useAgentFiles = (
  params: { agentId?: string; path?: string },
  enabled = true,
) => {
  return useQuery(
    queryOptions({
      queryKey: ["agent-fs", params.agentId ?? null, params.path ?? null],
      queryFn: () => listAgentFiles(params),
      enabled,
      retry: false,
    }),
  );
};
