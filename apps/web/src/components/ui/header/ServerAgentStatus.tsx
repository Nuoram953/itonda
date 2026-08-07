import { useWebSocketStatus } from "@/hooks/use-websocket-status";
import { useAgents } from "@/api/get-agents";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { Bot, Server, Laptop } from "lucide-react";
import { cn } from "@/lib/utils";

export function ServerAgentStatus() {
  const wsStatus = useWebSocketStatus();
  const { data, isLoading } = useAgents({});

  const agents = data?.agents ?? [];
  const connectedAgents = agents.filter((agent) => agent.is_connected);
  const isServerConnected = wsStatus === "connected";

  const statusColor = isServerConnected
    ? connectedAgents.length > 0
      ? "emerald"
      : "amber"
    : "rose";

  return (
    <Popover>
      <PopoverTrigger
        aria-label="Server and Agent Status"
        className={cn(
          "relative flex h-9 w-9 items-center justify-center rounded-lg border transition-all duration-200 outline-none cursor-pointer text-foreground border-border hover:border-strong hover:bg-surface-hover",
        )}
      >
        <Server className="h-4 w-4" />

        <span className="absolute top-1.5 right-1.5 flex h-2 w-2">
          <span
            className={cn(
              "absolute inline-flex h-full w-full animate-ping rounded-full opacity-75",
              statusColor === "emerald"
                ? "bg-emerald-400"
                : statusColor === "amber"
                  ? "bg-amber-400"
                  : "bg-rose-400",
            )}
          />
          <span
            className={cn(
              "relative inline-flex h-2 w-2 rounded-full",
              statusColor === "emerald"
                ? "bg-emerald-500"
                : statusColor === "amber"
                  ? "bg-amber-500"
                  : "bg-rose-500",
            )}
          />
        </span>
      </PopoverTrigger>

      <PopoverContent align="end" sideOffset={8} className="w-80 p-4">
        <div className="space-y-4">
          <div className="flex items-center justify-between border-b border-border-strong/40 pb-3">
            <div className="flex items-center gap-2">
              <h4 className="text-sm font-semibold">itonda-server</h4>
            </div>
            <span
              className={cn(
                "rounded-full px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wider",
                isServerConnected
                  ? "bg-emerald-500/15 text-emerald-400"
                  : wsStatus === "connecting"
                    ? "bg-amber-500/15 text-amber-400"
                    : "bg-rose-500/15 text-rose-400",
              )}
            >
              {wsStatus}
            </span>
          </div>

          <div>
            <div className="mb-2 flex items-center justify-between">
              <span className="text-xs font-medium text-text-muted">
                Connected Agents ({connectedAgents.length})
              </span>
            </div>

            {isLoading ? (
              <div className="py-4 text-center text-xs text-text-muted">
                Loading agents...
              </div>
            ) : agents.length === 0 ? (
              <div className="flex flex-col items-center justify-center rounded-lg border border-dashed border-border-strong/50 py-6 text-center">
                <Bot className="mb-2 h-6 w-6 text-text-muted opacity-40" />
                <p className="text-xs font-medium text-text-muted">
                  No agents connected
                </p>
                <p className="text-[11px] text-text-muted/60">
                  Run an itonda-agent node to connect.
                </p>
              </div>
            ) : (
              <div className="max-h-60 overflow-y-auto space-y-2 pr-1">
                {agents.map((agent) => (
                  <div
                    key={agent.id}
                    className={cn(
                      "flex items-center justify-between rounded-lg border p-2.5 transition-colors text-xs",
                      agent.is_connected
                        ? "border-emerald-500/20 bg-emerald-500/5"
                        : "border-border-strong/40 bg-surface/50 opacity-60",
                    )}
                  >
                    <div className="flex items-center gap-2.5 overflow-hidden">
                      <div className="flex h-7 w-7 shrink-0 items-center justify-center rounded-md bg-surface-hover text-foreground">
                        <Laptop className="h-4 w-4" />
                      </div>
                      <div className="truncate">
                        <p className="font-medium truncate text-foreground">
                          {agent.name}
                        </p>
                        <p className="text-[10px] text-text-muted truncate">
                          {agent.hostname ||
                            agent.ip_address ||
                            agent.id.slice(0, 8)}
                        </p>
                      </div>
                    </div>

                    <div className="flex shrink-0 items-center gap-1.5 pl-2">
                      {agent.platform && (
                        <span className="rounded bg-surface px-1.5 py-0.5 text-[9px] uppercase text-text-muted">
                          {agent.platform}
                        </span>
                      )}
                      {agent.is_connected ? (
                        <CheckCircle2 className="h-4 w-4 text-emerald-400" />
                      ) : (
                        <XCircle className="h-4 w-4 text-text-muted" />
                      )}
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      </PopoverContent>
    </Popover>
  );
}
