import { useState } from "react";
import type { components } from "@/api/generated.d";
import { useLaunchMedia } from "@/features/details/api/post-media-launch";
import { useActiveMedia } from "@/hooks/use-active-media";
import { useAgents } from "@/api/get-agents";
import { useWebSocketStatus } from "@/hooks/use-websocket-status";

export type UseMediaLauncherOptions = {
  profiles?: components["schemas"]["Media"]["launches"];
  mediaId?: string;
  onLaunchSuccess?: (launchId: string) => void;
};

export function useMediaLauncher({
  profiles = [],
  mediaId,
  onLaunchSuccess,
}: UseMediaLauncherOptions = {}) {
  const [open, setOpen] = useState(false);
  const [isLaunching, setIsLaunching] = useState(false);
  const launchMediaMutation = useLaunchMedia({});
  const { session, isPlaying, formattedElapsed } = useActiveMedia();
  const wsStatus = useWebSocketStatus();
  const { data: agentsData } = useAgents({});

  const agents = agentsData?.agents ?? [];
  const connectedAgents = agents.filter((agent) => agent.is_connected);
  const isServerConnected = wsStatus === "connected";
  const hasConnectedAgent = isServerConnected && connectedAgents.length > 0;

  const isProfileAvailable = (profile: components["schemas"]["Launch"]) => {
    if (!isServerConnected) {
      return false;
    }
    if (profile.agent_id) {
      return connectedAgents.some((agent) => agent.id === profile.agent_id);
    }
    return hasConnectedAgent;
  };

  const canLaunch = profiles.length > 0 && profiles.some(isProfileAvailable);

  const isCurrentMediaPlaying = Boolean(
    isPlaying && mediaId && session?.mediaId === mediaId,
  );

  if (isLaunching && isCurrentMediaPlaying) {
    setIsLaunching(false);
  }

  const isLoading = isLaunching || launchMediaMutation.isPending;

  function launch(id: string) {
    const profile = profiles.find((p) => p.id === id);
    if (profile && !isProfileAvailable(profile)) {
      return;
    }

    setIsLaunching(true);
    launchMediaMutation.mutate(id, {
      onSuccess: () => {
        setOpen(false);
        onLaunchSuccess?.(id);
      },
      onError: () => setIsLaunching(false),
    });
  }

  function handleLaunchClick() {
    if (!canLaunch || profiles.length === 0) {
      return;
    }

    if (profiles.length === 1) {
      launch(profiles[0].id);
      return;
    }

    setOpen(true);
  }

  const disabledReason = !profiles.length
    ? "No launch profiles available"
    : !isServerConnected
      ? "Server disconnected"
      : !hasConnectedAgent
        ? "No agent connected"
        : !canLaunch
          ? "Required agent not connected"
          : undefined;

  return {
    open,
    setOpen,
    isLaunching,
    isLoading,
    isPlaying: isCurrentMediaPlaying,
    formattedElapsed,
    canLaunch,
    disabledReason,
    profiles,
    isProfileAvailable,
    launch,
    handleLaunchClick,
  };
}
