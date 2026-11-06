import { renderHook, act } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import type { ReactNode } from "react";
import { NotificationProvider } from "../notificationProvider";
import { useNotification } from "../notificationContext";

function wrapper({ children }: { children: ReactNode }) {
  return <NotificationProvider>{children}</NotificationProvider>;
}

describe("NotificationProvider", () => {
  it("defaults loading notification duration to Infinity so it stays active", () => {
    const { result } = renderHook(() => useNotification(), { wrapper });

    act(() => {
      result.current.notify.loading({
        title: "Syncing Library",
        description: "Scanning and updating media...",
        sourceId: "job-123",
      });
    });

    expect(result.current.notifications).toHaveLength(1);
    expect(result.current.notifications[0].severity).toBe("loading");
    expect(result.current.notifications[0].duration).toBe(Infinity);
  });

  it("defaults non-loading notification duration to 5000", () => {
    const { result } = renderHook(() => useNotification(), { wrapper });

    act(() => {
      result.current.notify.success({
        title: "Saved",
      });
    });

    expect(result.current.notifications).toHaveLength(1);
    expect(result.current.notifications[0].severity).toBe("success");
    expect(result.current.notifications[0].duration).toBe(5000);
  });

  it("keeps duration Infinity when updating a loading notification with step progress", () => {
    const { result } = renderHook(() => useNotification(), { wrapper });

    act(() => {
      result.current.notify.loading({
        title: "Syncing Library",
        sourceId: "job-123",
      });
    });

    act(() => {
      result.current.notify.updateBySourceId("job-123", {
        title: "Syncing Hollow Knight",
        description: "Fetching metadata...",
      });
    });

    expect(result.current.notifications).toHaveLength(1);
    expect(result.current.notifications[0].title).toBe("Syncing Hollow Knight");
    expect(result.current.notifications[0].description).toBe("Fetching metadata...");
    expect(result.current.notifications[0].duration).toBe(Infinity);
  });

  it("sets finite duration when transitioning from loading to completed", () => {
    const { result } = renderHook(() => useNotification(), { wrapper });

    act(() => {
      result.current.notify.loading({
        title: "Syncing Library",
        sourceId: "job-123",
      });
    });

    act(() => {
      result.current.notify.updateBySourceId("job-123", {
        severity: "success",
        title: "Sync Completed",
        description: "Library synchronization finished",
        duration: 5000,
      });
    });

    expect(result.current.notifications).toHaveLength(1);
    expect(result.current.notifications[0].severity).toBe("success");
    expect(result.current.notifications[0].title).toBe("Sync Completed");
    expect(result.current.notifications[0].duration).toBe(5000);
  });

  it("upserts notification via updateBySourceId if it did not exist", () => {
    const { result } = renderHook(() => useNotification(), { wrapper });

    act(() => {
      result.current.notify.updateBySourceId("job-456", {
        title: "Syncing Game",
        description: "Downloading media...",
      });
    });

    expect(result.current.notifications).toHaveLength(1);
    expect(result.current.notifications[0].sourceId).toBe("job-456");
    expect(result.current.notifications[0].severity).toBe("loading");
    expect(result.current.notifications[0].duration).toBe(Infinity);
  });
});
