import { useEffect, useRef } from "react";
import type { QueryClient } from "@tanstack/react-query";

import type { AppWebSocket } from "./client";
import type { AgentEvent, JobEvent, MediaEvent } from "./types";

import type { NotificationContextValue } from "@/app/notificationContext";
import type { ActiveMediaSession } from "@/app/activeMediaContext";
import { formatDurationText } from "@/utils/datetime";
import { getAgentsQueryOptions } from "@/api/get-agents";


type Notify = NotificationContextValue["notify"];

export function useWebSocketHandlers(
  websocket: AppWebSocket,
  queryClient: QueryClient,
  notify: Notify,
  setActiveSession?: (session: ActiveMediaSession | null) => void,
) {
  const notifyRef = useRef(notify);
  const queryClientRef = useRef(queryClient);
  const setActiveSessionRef = useRef(setActiveSession);

  useEffect(() => {
    notifyRef.current = notify;
    queryClientRef.current = queryClient;
    setActiveSessionRef.current = setActiveSession;
  }, [notify, queryClient, setActiveSession]);

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

      if ("Media" in event) {
        handleMediaEvent(
          event.Media,
          queryClientRef.current,
          notifyRef.current,
          setActiveSessionRef.current,
        );
      }
    });
  }, [websocket]);
}

export function handleJobEvent(
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
    return;
  }

  if (event.type === "Sync" && event.payload.type === "MediaSynced") {
    queryClient.invalidateQueries({
      queryKey: ["media"],
    });

    notify.updateBySourceId(job.job_id, {
      sourceId: job.job_id,
      description: event.payload.payload.media_id,
    });
    return;
  }

  if (event.type === "Sync" && event.payload.type === "Completed") {
    notify.updateBySourceId(job.job_id, {
      duration: 8000,
    });
  }
}

export function handleAgentEvent(
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
    return;
  }

  if ("Disconnected" in agentEvent) {
    const id = agentEvent.Disconnected.agent_id;
    notify.info({
      title: "Agent Disconnected",
      description: `Agent disconnected (${id.slice(0, 8)})`,
    });
  }
}

export function handleMediaEvent(
  mediaEvent: MediaEvent,
  queryClient: QueryClient,
  notify: Notify,
  setActiveSession?: (session: ActiveMediaSession | null) => void,
) {
  if ("Launched" in mediaEvent) {
    const { media_id, launch_id, agent_id } = mediaEvent.Launched;

    setActiveSession?.({
      mediaId: media_id,
      launchId: launch_id,
      agentId: agent_id,
      startedAt: Date.now(),
    });

    invalidateMediaQuery(queryClient, media_id);

    notify.info({
      title: "Game Launched",
      description: "Game session started",
    });
    return;
  }

  if ("Stopped" in mediaEvent) {
    const { media_id, duration_seconds } = mediaEvent.Stopped;

    setActiveSession?.(null);

    invalidateMediaQuery(queryClient, media_id);

    notify.info({
      title: "Game Stopped",
      description: `Session duration: ${formatDurationText(duration_seconds)}`,
    });
  }
}

function invalidateMediaQuery(queryClient: QueryClient, mediaId?: string) {
  if (mediaId) {
    queryClient.invalidateQueries({ queryKey: ["media", mediaId] });
  }
  queryClient.invalidateQueries({ queryKey: ["media"] });
}
