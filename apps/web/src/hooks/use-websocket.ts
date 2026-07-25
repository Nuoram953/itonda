import { WebSocketContext } from "@/app/websocketProvider";
import { useContext } from "react";

export function useWebSocket() {
  const websocket = useContext(WebSocketContext);

  if (!websocket) {
    throw new Error("useWebSocket must be used inside WebSocketProvider");
  }

  return websocket;
}
