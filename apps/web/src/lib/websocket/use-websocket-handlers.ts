import { useEffect, useRef } from "react";
import type { QueryClient } from "@tanstack/react-query";

import type { AppWebSocket } from "./client";
import type { AgentEvent, JobEvent } from "./types";

import type { NotificationContextValue } from "@/app/notificationContext";
import { getAgentsQueryOptions } from "@/api/get-agents";

type Notify = NotificationContextValue["notify"];

export function useWebSocketHandlers(
  websocket: AppWebSocket,
  queryClient: QueryClient,
  notify: Notify,
) {
  const notifyRef = useRef(notify);
  const queryClientRef = useRef(queryClient);

  useEffect(() => {
    notifyRef.current = notify;
    queryClientRef.current = queryClient;
  }, [notify, queryClient]);

  useEffect(() => {
    return websocket.on((event) => {
      if ("Job" in event) {
        handleJobEvent(event.Job, queryClientRef.current, notifyRef.current);
      }

      if ("Agent" in event) {
        handleAgentEvent(
          event.Agent,
          queryClientRef.current,
          notifyRef.current,
        );
      }
    });
  }, [websocket]);
}

function handleJobEvent(
  job: JobEvent,
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

function handleAgentEvent(
  agentEvent: AgentEvent,
  queryClient: QueryClient,
  notify: Notify,
) {
  queryClient.invalidateQueries({
    queryKey: getAgentsQueryOptions().queryKey,
  });

  if ("Connected" in agentEvent) {
    const id = agentEvent.Connected.agent_id;

    notify.info({
      title: "Agent Connected",
      description: `Agent connected (${id.slice(0, 8)})`,
    });
  }

  if ("Disconnected" in agentEvent) {
    const id = agentEvent.Disconnected.agent_id;

    notify.info({
      title: "Agent Disconnected",
      description: `Agent disconnected (${id.slice(0, 8)})`,
    });
  }
}
