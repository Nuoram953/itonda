import { useWebSocketStatus } from "@/hooks/use-websocket-status";
import { useAgents } from "@/api/get-agents";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import {
  Activity,
  Bot,
  ChevronRight,
  Laptop,
  Monitor,
  Server,
  Terminal,
} from "lucide-react";
import { cn } from "@/lib/utils";
import { formatRelativeTime } from "@/utils/datetime";

function getPlatformIcon(platform?: string | null) {
  const p = platform?.toLowerCase() ?? "";
  if (p.includes("win")) {
    return <Monitor className="h-3.5 w-3.5" />;
  }
  if (p.includes("mac") || p.includes("darwin")) {
    return <Laptop className="h-3.5 w-3.5" />;
  }
  if (p.includes("linux")) {
    return <Terminal className="h-3.5 w-3.5" />;
  }
  return <Laptop className="h-3.5 w-3.5" />;
}

export function ServerAgentStatus() {
  const wsStatus = useWebSocketStatus();
  const { data, isLoading } = useAgents({});

  const agents = data?.agents ?? [];
  const connectedAgents = agents.filter((agent) => agent.is_connected);
  const isServerConnected = wsStatus === "connected";
  const isServerConnecting = wsStatus === "connecting";

  const dotColor = isServerConnected
    ? connectedAgents.length > 0
      ? "bg-emerald-500"
      : "bg-amber-500"
    : isServerConnecting
      ? "bg-amber-500"
      : "bg-rose-500";

  const pingColor = isServerConnected
    ? connectedAgents.length > 0
      ? "bg-emerald-400"
      : "bg-amber-400"
    : isServerConnecting
      ? "bg-amber-400"
      : "bg-rose-400";

  return (
    <Popover>
      <PopoverTrigger
        aria-label="Server and Agent Status"
        title="Server and Agent Status"
        className={cn(
          "group flex h-8 w-full items-center justify-between rounded-md px-2.5 text-xs text-text-muted transition-colors hover:bg-white/5 hover:text-foreground outline-none cursor-pointer select-none",
          "group-data-[collapsible=icon]:h-8 group-data-[collapsible=icon]:w-8 group-data-[collapsible=icon]:p-0 group-data-[collapsible=icon]:justify-center",
        )}
      >
        <div className="relative hidden group-data-[collapsible=icon]:flex items-center justify-center">
          <Server className="h-4 w-4 text-text-muted group-hover:text-foreground transition-colors" />
          <span className="absolute -top-1 -right-1 flex h-2 w-2">
            <span
              className={cn(
                "absolute inline-flex h-full w-full animate-ping rounded-full opacity-75",
                pingColor,
              )}
            />
            <span
              className={cn(
                "relative inline-flex h-2 w-2 rounded-full",
                dotColor,
              )}
            />
          </span>
        </div>

        <div className="flex items-center gap-2 min-w-0 group-data-[collapsible=icon]:hidden">
          <span className="relative flex h-2 w-2 shrink-0">
            <span
              className={cn(
                "absolute inline-flex h-full w-full animate-ping rounded-full opacity-75",
                pingColor,
              )}
            />
            <span
              className={cn(
                "relative inline-flex h-2 w-2 rounded-full",
                dotColor,
              )}
            />
          </span>

          <div className="flex items-center gap-1.5 truncate text-xs">
            <span className="font-medium text-foreground">
              {isServerConnected
                ? "Server"
                : isServerConnecting
                  ? "Connecting"
                  : "Offline"}
            </span>

            <span className="text-text-muted/40">·</span>

            <span className="truncate text-text-muted">
              {isLoading
                ? "Checking..."
                : isServerConnected
                  ? connectedAgents.length === 0
                    ? "No agents"
                    : `${connectedAgents.length} ${connectedAgents.length === 1 ? "agent" : "agents"}`
                  : "Disconnected"}
            </span>
          </div>
        </div>

        <ChevronRight className="h-3.5 w-3.5 text-text-muted/40 group-hover:text-text-muted group-hover:translate-x-0.5 transition-all shrink-0 group-data-[collapsible=icon]:hidden" />
      </PopoverTrigger>

      <PopoverContent
        side="right"
        align="end"
        sideOffset={10}
        className="w-80 p-3.5 bg-surface/95 backdrop-blur-2xl border-white/10 shadow-2xl space-y-3.5 rounded-lg"
      >
        <div className="flex items-start justify-between border-b border-white/10 pb-2.5">
          <div className="space-y-0.5">
            <div className="flex items-center gap-2">
              <Server className="h-4 w-4 text-primary" />
              <h4 className="text-sm font-semibold text-foreground">
                itonda-server
              </h4>
            </div>
            <p className="text-xs text-text-muted font-mono pl-6">
              WebSocket: {wsStatus}
            </p>
          </div>

          <span
            className={cn(
              "rounded px-2 py-0.5 text-xs font-mono uppercase tracking-wider border",
              isServerConnected
                ? "bg-emerald-500/10 text-emerald-400 border-emerald-500/20"
                : isServerConnecting
                  ? "bg-amber-500/10 text-amber-400 border-amber-500/20"
                  : "bg-rose-500/10 text-rose-400 border-rose-500/20",
            )}
          >
            {isServerConnected ? "Online" : wsStatus}
          </span>
        </div>

        <div>
          <div className="mb-2 flex items-center justify-between">
            <div className="flex items-center gap-1.5">
              <Activity className="h-3.5 w-3.5 text-text-muted" />
              <span className="text-xs font-medium uppercase tracking-wider text-text-muted">
                Agent Nodes ({connectedAgents.length}/{agents.length})
              </span>
            </div>

            {connectedAgents.length > 0 && (
              <span className="text-xs text-emerald-400 font-mono">
                {connectedAgents.length} active
              </span>
            )}
          </div>

          {isLoading ? (
            <div className="py-6 text-center text-xs text-text-muted">
              <div className="inline-block animate-spin mr-2 h-3 w-3 border-2 border-primary border-t-transparent rounded-full" />
              Loading agents...
            </div>
          ) : agents.length === 0 ? (
            <div className="flex flex-col items-center justify-center rounded-md border border-dashed border-white/10 p-4 text-center bg-surface/30">
              <Bot className="mb-2 h-6 w-6 text-text-muted opacity-40" />
              <p className="text-xs font-medium text-foreground">
                No Agents Paired
              </p>
              <p className="text-xs text-text-muted mt-1 leading-relaxed">
                Run an{" "}
                <code className="rounded bg-white/5 px-1 py-0.5 font-mono text-xs text-primary">
                  itonda-agent
                </code>{" "}
                worker node to execute tasks and launches.
              </p>
            </div>
          ) : (
            <div className="max-h-60 overflow-y-auto space-y-2 pr-1">
              {agents.map((agent) => {
                const connectedAgo = formatRelativeTime(agent.connected_at);
                const lastSeenAgo = formatRelativeTime(agent.last_seen_at);

                return (
                  <div
                    key={agent.id}
                    className={cn(
                      "flex flex-col gap-1.5 rounded-md border p-2.5 transition-colors text-xs",
                      agent.is_connected
                        ? "border-emerald-500/20 bg-emerald-500/5 hover:border-emerald-500/30"
                        : "border-white/10 bg-surface/40 opacity-70 hover:opacity-100",
                    )}
                  >
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-2 min-w-0">
                        <div className="flex h-6 w-6 shrink-0 items-center justify-center rounded bg-surface-raised border border-white/10 text-foreground">
                          {getPlatformIcon(agent.platform)}
                        </div>

                        <div className="min-w-0 truncate">
                          <div className="flex items-center gap-1.5">
                            <span className="font-medium text-foreground truncate">
                              {agent.name}
                            </span>
                            {agent.agent_version && (
                              <span className="rounded bg-white/5 border border-white/10 px-1 py-0.5 text-xs font-mono text-text-muted shrink-0">
                                v{agent.agent_version}
                              </span>
                            )}
                          </div>
                          <p className="text-xs text-text-muted truncate font-mono">
                            {agent.hostname ||
                              agent.ip_address ||
                              agent.id.slice(0, 8)}
                          </p>
                        </div>
                      </div>

                      <div className="flex items-center gap-1.5 shrink-0 pl-2">
                        <span
                          className={cn(
                            "flex h-2 w-2 rounded-full",
                            agent.is_connected
                              ? "bg-emerald-500"
                              : "bg-zinc-500",
                          )}
                        />
                        <span className="text-xs font-medium text-text-muted">
                          {agent.is_connected ? "Online" : "Offline"}
                        </span>
                      </div>
                    </div>

                    {(connectedAgo || lastSeenAgo || agent.platform) && (
                      <div className="flex items-center justify-between pt-1 border-t border-white/5 text-xs text-text-muted">
                        <span>
                          {agent.is_connected
                            ? connectedAgo
                              ? `Active ${connectedAgo}`
                              : "Connected"
                            : lastSeenAgo
                              ? `Last seen ${lastSeenAgo}`
                              : "Inactive"}
                        </span>

                        {agent.platform && (
                          <span className="uppercase text-xs font-medium tracking-wider text-text-muted/80">
                            {agent.platform}
                          </span>
                        )}
                      </div>
                    )}
                  </div>
                );
              })}
            </div>
          )}
        </div>
      </PopoverContent>
    </Popover>
  );
}
