import { useNotification } from "@/hooks/use-notification";
import { AppWebSocket } from "@/lib/websocket/client";
import { useWebSocketHandlers } from "@/lib/websocket/use-websocket-handlers";
import { useQueryClient } from "@tanstack/react-query";
import {
  createContext,
  useEffect,
  useMemo,
  type PropsWithChildren,
} from "react";

const WebSocketContext = createContext<AppWebSocket | null>(null);

export function WebSocketProvider({ children }: PropsWithChildren) {
  const queryClient = useQueryClient();
  const { notify } = useNotification();

  const websocket = useMemo(
    () => new AppWebSocket("ws://localhost:3005/ws"),
    [],
  );

  useWebSocketHandlers(websocket, queryClient, notify);

  useEffect(() => {
    websocket.connect();

    return () => {
      websocket.disconnect();
    };
  }, [websocket]);

  return (
    <WebSocketContext.Provider value={websocket}>
      {children}
    </WebSocketContext.Provider>
  );
}

export { WebSocketContext };
