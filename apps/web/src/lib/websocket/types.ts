export type AppEvent =
  | { Job: JobEvent }
  | { Agent: AgentEvent }
  | { Media: MediaEvent };

export type MediaEvent =
  | {
      Launched: {
        media_id: string;
        launch_id: string;
        agent_id: string;
      };
    }
  | {
      Stopped: {
        media_id: string;
        launch_id: string;
        agent_id: string;
        duration_seconds: number;
      };
    };

export type AgentEvent =
  | { Connected: { agent_id: string } }
  | { Disconnected: { agent_id: string } }
  | { ScanStarted: { agent_id: string } }
  | { ScanCompleted: { agent_id: string } };

export type JobEvent = {
  job_id: string;
  job_type: JobType;
  event: JobEventType;
};

export type JobType = {
  type: "Sync";
};

export type JobEventType = {
  type: "Sync";
  payload: SyncEvent;
};

export type SyncEvent =
  | {
      type: "MediaSynced";
      payload: {
        media_id: string;
      };
    }
  | {
      type: "MediaSyncFailed";
      payload: {
        media_id: string;
        error: string;
      };
    }
  | {
      type: "Started";
    }
  | {
      type: "Completed";
    };
