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

  if (event.type === "Sync" && event.payload.type === "Started") {
    notify.loading({
      sourceId: job.job_id,
      title: "Syncing",
      description: "Currently syncing",
    });
  }

  if (event.type === "Sync" && event.payload.type === "MediaSynced") {
    queryClient.invalidateQueries({
      queryKey: ["media"],
    });

    notify.updateBySourceId(job.job_id, {
      sourceId: job.job_id,
      description: event.payload.payload.media_id,
    });
  }

  if (event.type === "Sync" && event.payload.type === "Completed") {
    notify.updateBySourceId(job.job_id, {
      duration: 8000,
    });
  }
}
