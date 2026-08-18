import { describe, it, expect, vi, beforeEach, type Mock } from "vitest";
import type { QueryClient } from "@tanstack/react-query";
import type { ActiveMediaSession } from "@/app/activeMediaContext";
import {
  handleMediaEvent,
  handleAgentEvent,
  handleJobEvent,
} from "../use-websocket-handlers";
import type { MediaEvent, AgentEvent, JobEvent } from "../types";

describe("use-websocket-handlers", () => {
  let mockQueryClient: QueryClient;
  let mockNotify: {
    info: Mock;
    loading: Mock;
    updateBySourceId: Mock;
  };
  let mockSetActiveSession: Mock<(session: ActiveMediaSession | null) => void>;

  beforeEach(() => {
    mockQueryClient = {
      invalidateQueries: vi.fn(),
    } as unknown as QueryClient;

    mockNotify = {
      info: vi.fn(),
      loading: vi.fn(),
      updateBySourceId: vi.fn(),
    };

    mockSetActiveSession = vi.fn<(session: ActiveMediaSession | null) => void>();
  });

  describe("handleMediaEvent", () => {
    it("handles Launched event correctly", () => {
      const launchedEvent: MediaEvent = {
        Launched: {
          media_id: "game-101",
          launch_id: "launch-202",
          agent_id: "agent-303",
        },
      };

      handleMediaEvent(
        launchedEvent,
        mockQueryClient,
        mockNotify as never,
        mockSetActiveSession,
      );

      expect(mockSetActiveSession).toHaveBeenCalledWith(
        expect.objectContaining({
          mediaId: "game-101",
          launchId: "launch-202",
          agentId: "agent-303",
        }),
      );

      expect(mockQueryClient.invalidateQueries).toHaveBeenCalledWith({
        queryKey: ["media", "game-101"],
      });
      expect(mockQueryClient.invalidateQueries).toHaveBeenCalledWith({
        queryKey: ["media"],
      });

      expect(mockNotify.info).not.toHaveBeenCalled();
    });

    it("handles Stopped event with duration formatting", () => {
      const stoppedEvent: MediaEvent = {
        Stopped: {
          media_id: "game-101",
          launch_id: "launch-202",
          agent_id: "agent-303",
          duration_seconds: 125,
        },
      };

      handleMediaEvent(
        stoppedEvent,
        mockQueryClient,
        mockNotify as never,
        mockSetActiveSession,
      );

      expect(mockSetActiveSession).toHaveBeenCalledWith(null);

      expect(mockQueryClient.invalidateQueries).toHaveBeenCalledWith({
        queryKey: ["media", "game-101"],
      });
      expect(mockQueryClient.invalidateQueries).toHaveBeenCalledWith({
        queryKey: ["media"],
      });

      expect(mockNotify.info).toHaveBeenCalledWith({
        title: "Game Stopped",
        description: "Session duration: 2m 5s",
      });
    });
  });

  describe("handleAgentEvent", () => {
    it("handles Connected event", () => {
      const agentEvent: AgentEvent = {
        Connected: {
          agent_id: "12345678-90ab-cdef-1234-567890abcdef",
        },
      };

      handleAgentEvent(agentEvent, mockQueryClient, mockNotify as never);

      expect(mockQueryClient.invalidateQueries).toHaveBeenCalled();
      expect(mockNotify.info).toHaveBeenCalledWith({
        title: "Agent Connected",
        description: "Agent connected (12345678)",
      });
    });

    it("handles Disconnected event", () => {
      const agentEvent: AgentEvent = {
        Disconnected: {
          agent_id: "87654321-fedc-ba09-4321-fedcba098765",
        },
      };

      handleAgentEvent(agentEvent, mockQueryClient, mockNotify as never);

      expect(mockQueryClient.invalidateQueries).toHaveBeenCalled();
      expect(mockNotify.info).toHaveBeenCalledWith({
        title: "Agent Disconnected",
        description: "Agent disconnected (87654321)",
      });
    });
  });

  describe("handleJobEvent", () => {
    it("handles Sync Started event", () => {
      const jobEvent: JobEvent = {
        job_id: "job-1",
        job_type: { type: "Sync" },
        event: {
          type: "Sync",
          payload: {
            type: "Started",
          },
        },
      };

      handleJobEvent(jobEvent, mockQueryClient, mockNotify as never);

      expect(mockNotify.loading).toHaveBeenCalledWith({
        sourceId: "job-1",
        title: "Syncing Library",
        description: "Scanning and updating media...",
      });
    });

    it("handles Sync MediaSynced event", () => {
      const jobEvent: JobEvent = {
        job_id: "job-2",
        job_type: { type: "Sync" },
        event: {
          type: "Sync",
          payload: {
            type: "MediaSynced",
            payload: {
              media_id: "media-99",
            },
          },
        },
      };

      handleJobEvent(jobEvent, mockQueryClient, mockNotify as never);

      expect(mockQueryClient.invalidateQueries).toHaveBeenCalledWith({
        queryKey: ["media"],
      });
      expect(mockNotify.updateBySourceId).toHaveBeenCalledWith("job-2", {
        sourceId: "job-2",
        description: "Syncing media-99...",
      });
    });

    it("handles Sync Completed event", () => {
      const jobEvent: JobEvent = {
        job_id: "job-3",
        job_type: { type: "Sync" },
        event: {
          type: "Sync",
          payload: {
            type: "Completed",
          },
        },
      };

      handleJobEvent(jobEvent, mockQueryClient, mockNotify as never);

      expect(mockNotify.updateBySourceId).toHaveBeenCalledWith("job-3", {
        severity: "success",
        title: "Sync Completed",
        description: "Library synchronization finished",
        duration: 5000,
      });
    });

    it("handles Sync MediaSyncFailed event", () => {
      const jobEvent: JobEvent = {
        job_id: "job-4",
        job_type: { type: "Sync" },
        event: {
          type: "Sync",
          payload: {
            type: "MediaSyncFailed",
            payload: {
              media_id: "media-99",
              error: "Connection timeout",
            },
          },
        },
      };

      handleJobEvent(jobEvent, mockQueryClient, mockNotify as never);

      expect(mockNotify.updateBySourceId).toHaveBeenCalledWith("job-4", {
        severity: "error",
        title: "Sync Failed",
        description: "Failed to sync media-99: Connection timeout",
        duration: 8000,
      });
    });
  });
});
