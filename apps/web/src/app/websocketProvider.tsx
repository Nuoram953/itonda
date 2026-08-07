import { useNotification } from "@/hooks/use-notification";
import { AppWebSocket } from "@/lib/websocket/client";
import { useWebSocketHandlers } from "@/lib/websocket/use-websocket-handlers";
import { useQueryClient } from "@tanstack/react-query";
import { createContext, useEffect, type PropsWithChildren } from "react";

const WebSocketContext = createContext<AppWebSocket | null>(null);

function getWebSocketUrl(): string {
  const serverUrl =
    import.meta.env.VITE_SERVER_URL ||
    `${window.location.protocol}//${window.location.hostname}:3005`;

  const wsProtocol = serverUrl.startsWith("https") ? "wss:" : "ws:";
  // const host = serverUrl.replace(/^https?:\/\//, "");

  return `${wsProtocol}//localhost:3005/ws`;
}

let singletonWebSocket: AppWebSocket | null = null;

function getAppWebSocket(): AppWebSocket {
  if (!singletonWebSocket) {
    singletonWebSocket = new AppWebSocket(getWebSocketUrl());
  }

  return singletonWebSocket;
}

export function WebSocketProvider({ children }: PropsWithChildren) {
  const queryClient = useQueryClient();
  const { notify } = useNotification();
  const websocket = getAppWebSocket();

  useWebSocketHandlers(websocket, queryClient, notify);

  useEffect(() => {
    websocket.connect();
  }, [websocket]);

  return (
    <WebSocketContext.Provider value={websocket}>
      {children}
    </WebSocketContext.Provider>
  );
}

export { WebSocketContext };
