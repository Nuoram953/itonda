import { useEffect } from "react";
import type { QueryClient } from "@tanstack/react-query";

import type { AppWebSocket } from "./client";
import type { AppEvent } from "./types";

import type { NotificationContextValue } from "@/app/notificationContext";

type Notify = NotificationContextValue["notify"];

export function useWebSocketHandlers(
  websocket: AppWebSocket,
  queryClient: QueryClient,
  notify: Notify,
) {
  useEffect(() => {
    return websocket.on((event) => {
      if ("Job" in event) {
        handleJobEvent(event.Job, queryClient, notify);
      }
    });
  }, [websocket, queryClient, notify]);
}

function handleJobEvent(
  job: AppEvent["Job"],
  queryClient: QueryClient,
  notify: Notify,
) {
  const event = job.event;

  if (event.type === "Sync" && event.payload.type === "MediaSynced") {
    queryClient.invalidateQueries({
      queryKey: ["media"],
    });

    notify.success({
      title: "Game Synced",
      description: "Import was completed successfully",
      duration: 9000,
    });
  }
}
