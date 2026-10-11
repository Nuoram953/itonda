import { useState } from "react";
import type { components } from "@/api/generated.d";
import { useLaunchMedia } from "@/features/details/api/post-media-launch";
import { useActiveMedia } from "@/hooks/use-active-media";
import { useAgents } from "@/api/get-agents";
import { useWebSocketStatus } from "@/hooks/use-websocket-status";
import { useNotification } from "@/hooks/use-notification";

export type MediaItem = components["schemas"]["Media"];
export type LaunchProfile = components["schemas"]["Launch"];

export type UseMediaLauncherOptions = {
  profiles?: LaunchProfile[];
  mediaId?: string;
  onLaunchSuccess?: (launchId: string) => void;
};

export function useMediaLauncher({
  profiles: initialProfiles = [],
  mediaId,
  onLaunchSuccess,
}: UseMediaLauncherOptions = {}) {
  const [open, setOpen] = useState(false);
  const [isLaunching, setIsLaunching] = useState(false);
  const [activeDialogMedia, setActiveDialogMedia] = useState<MediaItem | null>(
    null,
  );

  const { notify } = useNotification();
  const launchMediaMutation = useLaunchMedia({});
  const { session, isPlaying, formattedElapsed } = useActiveMedia();
  const wsStatus = useWebSocketStatus();
  const { data: agentsData } = useAgents({});

  const agents = agentsData?.agents ?? [];
  const connectedAgents = agents.filter((agent) => agent.is_connected);
  const isServerConnected = wsStatus === "connected";
  const hasConnectedAgent = isServerConnected && connectedAgents.length > 0;

  const isProfileAvailable = (profile: LaunchProfile) => {
    if (!isServerConnected) {
      return false;
    }
    if (profile.agent_id) {
      return connectedAgents.some((agent) => agent.id === profile.agent_id);
    }
    return hasConnectedAgent;
  };

  const canLaunchProfiles = (profilesList: LaunchProfile[] = []) => {
    return profilesList.length > 0 && profilesList.some(isProfileAvailable);
  };

  const canLaunchMedia = (media?: MediaItem | null) => {
    if (!media || media.media_type !== "game") return false;
    return canLaunchProfiles(media.launches);
  };

  const getDisabledReason = (
    target?: LaunchProfile[] | MediaItem | null,
  ): string | undefined => {
    if (!target) return undefined;
    const profilesList = Array.isArray(target)
      ? target
      : (target.launches ?? []);

    if (!profilesList.length) return "No launch profiles available";
    if (!isServerConnected) return "Server disconnected";
    if (!hasConnectedAgent) return "No agent connected";
    if (!profilesList.some(isProfileAvailable)) {
      return "Required agent not connected";
    }
    return undefined;
  };

  const isMediaPlaying = (targetMediaId?: string) => {
    return Boolean(
      isPlaying && targetMediaId && session?.mediaId === targetMediaId,
    );
  };

  const isCurrentMediaPlaying = Boolean(
    isPlaying && mediaId && session?.mediaId === mediaId,
  );

  if (isLaunching && isCurrentMediaPlaying) {
    setIsLaunching(false);
  }

  const isLoading = isLaunching || Boolean(launchMediaMutation.isPending);

  function launch(
    id: string,
    options?: {
      mediaTitle?: string;
      onSuccess?: () => void;
      onError?: (err: unknown) => void;
    },
  ) {
    const allProfiles = activeDialogMedia
      ? activeDialogMedia.launches
      : initialProfiles;
    const profile = allProfiles.find((p) => p.id === id);
    if (profile && !isProfileAvailable(profile)) {
      return;
    }

    setIsLaunching(true);
    launchMediaMutation.mutate(id, {
      onSuccess: () => {
        setOpen(false);
        setActiveDialogMedia(null);
        if (options?.mediaTitle) {
          notify.success({
            title: "Launching game",
            description: `Starting ${options.mediaTitle}...`,
          });
        }
        options?.onSuccess?.();
        onLaunchSuccess?.(id);
      },
      onError: (err) => {
        setIsLaunching(false);
        if (options?.mediaTitle) {
          notify.error({
            title: "Launch failed",
            description:
              err instanceof Error ? err.message : "Failed to launch game.",
          });
        }
        options?.onError?.(err);
      },
    });
  }

  function handleLaunchClick() {
    if (!canLaunchProfiles(initialProfiles)) {
      return;
    }

    if (initialProfiles.length === 1) {
      launch(initialProfiles[0].id);
      return;
    }

    setOpen(true);
  }

  function handleLaunchMedia(
    media: MediaItem,
    callbacks?: { onSuccess?: () => void },
  ) {
    const profilesList = media.launches ?? [];
    if (!canLaunchMedia(media)) {
      notify.error({
        title: "Cannot launch game",
        description:
          getDisabledReason(media) ?? "No available agent profile found.",
      });
      return;
    }

    if (profilesList.length === 1) {
      launch(profilesList[0].id, {
        mediaTitle: media.title,
        onSuccess: callbacks?.onSuccess,
      });
      return;
    }

    setActiveDialogMedia(media);
  }

  return {
    // General availability & helpers
    isServerConnected,
    hasConnectedAgent,
    isProfileAvailable,
    canLaunchProfiles,
    canLaunchMedia,
    getDisabledReason,
    isMediaPlaying,

    // Launch actions
    launch,
    handleLaunchMedia,
    activeDialogMedia,
    setActiveDialogMedia,

    // Backwards-compatible properties for single-target Launch.tsx
    open,
    setOpen,
    isLaunching,
    isLoading,
    isPlaying: isCurrentMediaPlaying,
    formattedElapsed,
    canLaunch: canLaunchProfiles(initialProfiles),
    disabledReason: getDisabledReason(initialProfiles),
    profiles: initialProfiles,
    handleLaunchClick,
  };
}
