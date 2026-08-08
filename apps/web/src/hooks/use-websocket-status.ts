import { useState, useEffect } from "react";
import { useWebSocket } from "./use-websocket";
import type { ConnectionStatus } from "@/lib/websocket/client";
import { useQueryClient } from "@tanstack/react-query";
import { getAgentsQueryOptions } from "@/api/get-agents";

export function useWebSocketStatus(): ConnectionStatus {
  const websocket = useWebSocket();
  const queryClient = useQueryClient();

  const [status, setStatus] = useState<ConnectionStatus>(websocket.getStatus());

  useEffect(() => {
    return websocket.onStatusChange((newStatus) => {
      if (newStatus == "disconnected") {
        queryClient.invalidateQueries({
          queryKey: getAgentsQueryOptions().queryKey,
        });
      }

      setStatus(newStatus);
    });
  }, [websocket, queryClient]);

  return status;
}
