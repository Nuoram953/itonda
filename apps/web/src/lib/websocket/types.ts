export type AppEvent = {
  Job: JobEvent;
};

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
