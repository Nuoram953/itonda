import type { AppEvent } from "./types";

type EventHandler = (event: AppEvent) => void;

export class AppWebSocket {
  private readonly url: string;

  private socket?: WebSocket;

  private listeners = new Set<EventHandler>();

  constructor(url: string) {
    this.url = url;
  }

  connect() {
    if (this.socket?.readyState === WebSocket.OPEN) {
      return;
    }

    this.socket = new WebSocket(this.url);

    this.socket.onopen = () => {
      console.log("WebSocket connected");
    };

    this.socket.onmessage = (message) => {
      this.handleMessage(message.data);
    };

    this.socket.onerror = (error) => {
      console.error("WebSocket error", error);
    };

    this.socket.onclose = () => {
      console.log("WebSocket disconnected");

      this.socket = undefined;
    };
  }

  disconnect() {
    this.socket?.close();
    this.socket = undefined;
  }

  send<T>(message: T) {
    if (this.socket?.readyState !== WebSocket.OPEN) {
      console.warn("Cannot send message. WebSocket is not connected.");

      return;
    }

    this.socket.send(JSON.stringify(message));
  }

  on(handler: EventHandler) {
    this.listeners.add(handler);

    return () => {
      this.listeners.delete(handler);
    };
  }

  private emit(event: AppEvent) {
    this.listeners.forEach((handler) => {
      handler(event);
    });
  }

  private handleMessage(raw: string) {
    try {
      const event = JSON.parse(raw) as AppEvent;

      this.emit(event);
    } catch (error) {
      console.error("Failed to parse websocket message", error);
    }
  }
}
