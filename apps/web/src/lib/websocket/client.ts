import type { AppEvent } from "./types";

export type ConnectionStatus = "connecting" | "connected" | "disconnected";
type EventHandler = (event: AppEvent) => void;
type StatusHandler = (status: ConnectionStatus) => void;

export class AppWebSocket {
  private readonly url: string;

  private socket?: WebSocket;

  private listeners = new Set<EventHandler>();

  private statusListeners = new Set<StatusHandler>();

  private currentStatus: ConnectionStatus = "disconnected";

  private reconnectTimer?: ReturnType<typeof setTimeout>;

  private isExplicitDisconnect = false;

  constructor(url: string) {
    this.url = url;
  }

  getStatus(): ConnectionStatus {
    return this.currentStatus;
  }

  onStatusChange(handler: StatusHandler) {
    this.statusListeners.add(handler);

    handler(this.currentStatus);

    return () => {
      this.statusListeners.delete(handler);
    };
  }

  private setStatus(status: ConnectionStatus) {
    if (this.currentStatus === status) {
      return;
    }

    this.currentStatus = status;

    this.statusListeners.forEach((handler) => {
      handler(status);
    });
  }

  connect() {
    if (
      this.socket?.readyState === WebSocket.OPEN ||
      this.socket?.readyState === WebSocket.CONNECTING
    ) {
      return;
    }

    this.isExplicitDisconnect = false;

    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = undefined;
    }

    this.setStatus("connecting");

    try {
      this.socket = new WebSocket(this.url);

      this.socket.onopen = () => {
        console.log("WebSocket connected");

        this.setStatus("connected");
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

        this.setStatus("disconnected");

        if (!this.isExplicitDisconnect) {
          this.scheduleReconnect();
        }
      };
    } catch (err) {
      console.error("Failed to establish WebSocket connection", err);

      this.setStatus("disconnected");

      if (!this.isExplicitDisconnect) {
        this.scheduleReconnect();
      }
    }
  }

  private scheduleReconnect() {
    if (this.reconnectTimer) {
      return;
    }

    this.reconnectTimer = setTimeout(() => {
      this.reconnectTimer = undefined;

      this.connect();
    }, 3000);
  }

  disconnect() {
    this.isExplicitDisconnect = true;

    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = undefined;
    }

    this.socket?.close();

    this.socket = undefined;

    this.setStatus("disconnected");
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
