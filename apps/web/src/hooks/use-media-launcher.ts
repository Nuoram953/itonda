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
  const [activeDialogMedia, setActiveDialogMedia] = useState<MediaItem | null>(null);

  const { notify } = useNotification();
  const launchMutation = useLaunchMedia({});
  const { session, isPlaying, formattedElapsed } = useActiveMedia();
  const wsStatus = useWebSocketStatus();
  const { data: agentsData } = useAgents({});

  const isServerConnected = wsStatus === "connected";
  const connectedAgents = agentsData?.agents?.filter((a) => a.is_connected) ?? [];
  const hasConnectedAgent = isServerConnected && connectedAgents.length > 0;

  const isProfileAvailable = (profile: LaunchProfile) => {
    if (!isServerConnected) return false;
    return profile.agent_id
      ? connectedAgents.some((a) => a.id === profile.agent_id)
      : hasConnectedAgent;
  };

  const canLaunch = (target?: MediaItem | LaunchProfile[] | null) => {
    if (!target) return false;
    if ("media_type" in target) {
      return target.media_type === "game" && Boolean(target.launches?.some(isProfileAvailable));
    }
    return target.length > 0 && target.some(isProfileAvailable);
  };

  const getDisabledReason = (target?: MediaItem | LaunchProfile[] | null) => {
    if (!target) return undefined;
    const list = "media_type" in target ? (target.launches ?? []) : target;
    if (!list.length) return "No launch profiles available";
    if (!isServerConnected) return "Server disconnected";
    if (!hasConnectedAgent) return "No agent connected";
    if (!list.some(isProfileAvailable)) return "Required agent not connected";
    return undefined;
  };

  const isMediaPlaying = (id?: string) => Boolean(isPlaying && id && session?.mediaId === id);
  const isCurrentPlaying = Boolean(mediaId && isMediaPlaying(mediaId));
  if (isLaunching && isCurrentPlaying) setIsLaunching(false);

  const launch = (profileId: string, options?: { mediaTitle?: string; onSuccess?: () => void }) => {
    setIsLaunching(true);
    launchMutation.mutate(profileId, {
      onSuccess: () => {
        setOpen(false);
        setActiveDialogMedia(null);
        if (options?.mediaTitle) {
          notify.success({ title: "Launching game", description: `Starting ${options.mediaTitle}...` });
        }
        options?.onSuccess?.();
        onLaunchSuccess?.(profileId);
      },
      onError: (err) => {
        setIsLaunching(false);
        if (options?.mediaTitle) {
          notify.error({
            title: "Launch failed",
            description: err instanceof Error ? err.message : "Failed to launch game.",
          });
        }
      },
    });
  };

  const handleLaunchClick = () => {
    if (!canLaunch(initialProfiles)) return;
    if (initialProfiles.length === 1) return launch(initialProfiles[0].id);
    setOpen(true);
  };

  const handleLaunchMedia = (media: MediaItem, callbacks?: { onSuccess?: () => void }) => {
    if (!canLaunch(media)) {
      notify.error({
        title: "Cannot launch game",
        description: getDisabledReason(media) ?? "No available agent profile found.",
      });
      return;
    }
    if ((media.launches?.length ?? 0) === 1) {
      return launch(media.launches![0].id, { mediaTitle: media.title, onSuccess: callbacks?.onSuccess });
    }
    setActiveDialogMedia(media);
  };

  const isLoading = isLaunching || Boolean(launchMutation.isPending);

  const dialogProps = activeDialogMedia
    ? {
        open: true,
        onOpenChange: (nextOpen: boolean) => {
          if (!nextOpen) setActiveDialogMedia(null);
        },
        title: activeDialogMedia.title,
        profiles: activeDialogMedia.launches ?? [],
        isProfileAvailable,
        onSelectProfile: (profileId: string) =>
          launch(profileId, { mediaTitle: activeDialogMedia.title }),
        isLoading,
      }
    : null;

  return {
    isServerConnected,
    hasConnectedAgent,
    isProfileAvailable,
    canLaunchProfiles: (p?: LaunchProfile[]) => canLaunch(p),
    canLaunchMedia: (m?: MediaItem | null) => canLaunch(m),
    getDisabledReason,
    isMediaPlaying,
    launch,
    handleLaunchMedia,
    activeDialogMedia,
    setActiveDialogMedia,
    open,
    setOpen,
    isLaunching,
    isLoading,
    isPlaying: isCurrentPlaying,
    formattedElapsed,
    canLaunch: canLaunch(initialProfiles),
    disabledReason: getDisabledReason(initialProfiles),
    profiles: initialProfiles,
    handleLaunchClick,
    dialogProps,
  };
}
