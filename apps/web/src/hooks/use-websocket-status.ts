import { useState, useEffect } from "react";
import { useWebSocket } from "./use-websocket";
import type { ConnectionStatus } from "@/lib/websocket/client";

export function useWebSocketStatus(): ConnectionStatus {
  const websocket = useWebSocket();
  const [status, setStatus] = useState<ConnectionStatus>(
    websocket.getStatus(),
  );

  useEffect(() => {
    return websocket.onStatusChange((newStatus) => {
      setStatus(newStatus);
    });
  }, [websocket, queryClient]);

  return status;
}
