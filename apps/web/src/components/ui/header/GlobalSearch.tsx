import { useState } from "react";
import { useNavigate } from "@tanstack/react-router";
import { Loader2, Search, X } from "lucide-react";
import type { components } from "@/api/generated.d";
import { useLaunchMedia } from "@/features/details/api/post-media-launch";
import { useActiveMedia } from "@/hooks/use-active-media";
import { useAgents } from "@/api/get-agents";
import { useWebSocketStatus } from "@/hooks/use-websocket-status";
import { useNotification } from "@/hooks/use-notification";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import {
  useGlobalSearch,
  type MediaItem,
  type SearchAction,
} from "./use-global-search";
import { GlobalSearchResultRow } from "./GlobalSearchResultRow";

export function GlobalSearch() {
  const navigate = useNavigate();
  const { notify } = useNotification();
  const { session, isPlaying } = useActiveMedia();
  const wsStatus = useWebSocketStatus();
  const { data: agentsData } = useAgents({});
  const launchMediaMutation = useLaunchMedia({});

  const [multiLaunchMedia, setMultiLaunchMedia] = useState<MediaItem | null>(
    null,
  );

  const agents = agentsData?.agents ?? [];
  const connectedAgents = agents.filter((agent) => agent.is_connected);
  const isServerConnected = wsStatus === "connected";
  const hasConnectedAgent = isServerConnected && connectedAgents.length > 0;

  const isProfileAvailable = (profile: components["schemas"]["Launch"]) => {
    if (!isServerConnected) return false;
    if (profile.agent_id) {
      return connectedAgents.some((agent) => agent.id === profile.agent_id);
    }
    return hasConnectedAgent;
  };

  const executeLaunch = (profileId: string, mediaTitle: string) => {
    launchMediaMutation.mutate(profileId, {
      onSuccess: () => {
        setMultiLaunchMedia(null);
        notify.success({
          title: "Launching game",
          description: `Starting ${mediaTitle}...`,
        });
      },
      onError: (err) => {
        notify.error({
          title: "Launch failed",
          description:
            err instanceof Error ? err.message : "Failed to launch game.",
        });
      },
    });
  };

  const handleStartGame = (item: MediaItem) => {
    const profiles = item.launches ?? [];
    const availableProfiles = profiles.filter(isProfileAvailable);

    if (availableProfiles.length === 0) {
      notify.error({
        title: "Cannot launch game",
        description: "No connected agent available for this game.",
      });
      return;
    }

    if (profiles.length === 1) {
      executeLaunch(profiles[0].id, item.title);
      setIsOpen(false);
      return;
    }

    // Multiple profiles: open selection dialog
    setMultiLaunchMedia(item);
    setIsOpen(false);
  };

  const handleGoToDetails = (item: MediaItem) => {
    setIsOpen(false);
    navigate({
      to: "/media/$mediaId",
      params: { mediaId: item.id },
    });
  };

  const handleExecuteAction = (item: MediaItem, action: SearchAction) => {
    if (action === "start") {
      handleStartGame(item);
    } else {
      handleGoToDetails(item);
    }
  };

  const {
    query,
    setQuery,
    clearQuery,
    isOpen,
    setIsOpen,
    debouncedQuery,
    activeIndex,
    setActiveIndex,
    activeAction,
    setActiveAction,
    results,
    isLoading,
    handleKeyDown,
    inputRef,
    containerRef,
  } = useGlobalSearch({
    onExecuteAction: handleExecuteAction,
  });

  return (
    <>
      <div ref={containerRef} className="relative w-72 lg:w-96">
        {/* Search input container */}
        <div className="flex items-center gap-2.5 bg-surface-raised/50 border border-white/10 focus-within:border-primary/50 focus-within:ring-1 focus-within:ring-primary/40 rounded-xl px-3.5 py-1.5 w-full text-sm text-text-muted transition-all duration-200">
          {isLoading ? (
            <Loader2 className="w-4 h-4 text-primary shrink-0 animate-spin" />
          ) : (
            <Search className="w-4 h-4 text-text-muted shrink-0" />
          )}

          <input
            ref={inputRef}
            type="text"
            role="combobox"
            aria-expanded={isOpen && Boolean(query.trim())}
            aria-autocomplete="list"
            aria-controls="global-search-results"
            placeholder="Search media, games, movies..."
            value={query}
            onChange={(e) => {
              setQuery(e.target.value);
              if (!isOpen) setIsOpen(true);
            }}
            onFocus={() => {
              if (query.trim().length > 0) setIsOpen(true);
            }}
            onKeyDown={handleKeyDown}
            className="w-full bg-transparent border-none outline-none text-foreground placeholder:text-text-muted/60 text-xs"
          />

          {query ? (
            <button
              type="button"
              onClick={clearQuery}
              className="text-text-muted hover:text-foreground cursor-pointer"
              aria-label="Clear search"
            >
              <X className="w-3.5 h-3.5" />
            </button>
          ) : (
            <kbd className="hidden lg:inline-flex items-center gap-0.5 rounded bg-surface border border-white/10 px-1.5 text-xs text-text-muted font-mono">
              ⌘K
            </kbd>
          )}
        </div>

        {/* Dropdown Popover */}
        {isOpen && query.trim().length > 0 && (
          <div
            id="global-search-results"
            role="listbox"
            className="absolute top-full mt-2 left-0 w-full sm:w-[480px] lg:w-[540px] z-50 rounded-2xl bg-surface/95 border border-white/10 shadow-2xl backdrop-blur-xl overflow-hidden animate-in fade-in-0 zoom-in-95"
          >
            {isLoading && results.length === 0 ? (
              <div className="flex items-center justify-center gap-2 p-8 text-xs text-text-muted">
                <Loader2 className="w-4 h-4 animate-spin text-primary" />
                <span>Searching media...</span>
              </div>
            ) : results.length === 0 ? (
              <div className="p-8 text-center text-xs text-text-muted">
                No media found matching &ldquo;{debouncedQuery || query}&rdquo;
              </div>
            ) : (
              <>
                <div className="p-2 space-y-1.5 max-h-[60vh] overflow-y-auto">
                  {results.map((item, idx) => {
                    const isSelected = activeIndex === idx;
                    const isCurrentPlaying = Boolean(
                      isPlaying && session?.mediaId === item.id,
                    );

                    return (
                      <GlobalSearchResultRow
                        key={item.id}
                        item={item}
                        isSelected={isSelected}
                        activeAction={isSelected ? activeAction : "detail"}
                        onSelect={(action) => handleExecuteAction(item, action)}
                        onHover={() => setActiveIndex(idx)}
                        onHoverAction={(action) => {
                          setActiveIndex(idx);
                          setActiveAction(action);
                        }}
                        isProfileAvailable={isProfileAvailable}
                        isCurrentMediaPlaying={isCurrentPlaying}
                        isServerConnected={isServerConnected}
                        hasConnectedAgent={hasConnectedAgent}
                      />
                    );
                  })}
                </div>

                {/* Footer hints */}
                <div className="flex items-center justify-between border-t border-white/10 px-3 py-2 text-[11px] text-text-muted bg-surface/50">
                  <div className="flex items-center gap-3">
                    <span className="flex items-center gap-1">
                      <kbd className="rounded bg-surface border border-white/10 px-1 py-0.5 font-mono text-[10px]">
                        ↑↓
                      </kbd>
                      <span>Navigate</span>
                    </span>
                    <span className="flex items-center gap-1">
                      <kbd className="rounded bg-surface border border-white/10 px-1 py-0.5 font-mono text-[10px]">
                        Tab
                      </kbd>
                      <span>Switch action</span>
                    </span>
                    <span className="flex items-center gap-1">
                      <kbd className="rounded bg-surface border border-white/10 px-1 py-0.5 font-mono text-[10px]">
                        ↵
                      </kbd>
                      <span>Select</span>
                    </span>
                  </div>

                  <span className="flex items-center gap-1">
                    <kbd className="rounded bg-surface border border-white/10 px-1 py-0.5 font-mono text-[10px]">
                      Esc
                    </kbd>
                    <span>Close</span>
                  </span>
                </div>
              </>
            )}
          </div>
        )}
      </div>

      {/* Multi-profile selection modal */}
      {multiLaunchMedia && (
        <Dialog
          open={Boolean(multiLaunchMedia)}
          onOpenChange={(open) => !open && setMultiLaunchMedia(null)}
        >
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Select launch profile</DialogTitle>
              <DialogDescription>
                Choose how you want to launch {multiLaunchMedia.title}.
              </DialogDescription>
            </DialogHeader>

            <div className="flex flex-col gap-2">
              {multiLaunchMedia.launches.map((profile) => {
                const available = isProfileAvailable(profile);
                return (
                  <Button
                    key={profile.id}
                    variant="outline"
                    disabled={!available}
                    onClick={() =>
                      executeLaunch(profile.id, multiLaunchMedia.title)
                    }
                    title={!available ? "Agent not connected" : undefined}
                  >
                    {profile.name}
                  </Button>
                );
              })}
            </div>

            <DialogFooter>
              <Button
                variant="ghost"
                onClick={() => setMultiLaunchMedia(null)}
              >
                Cancel
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      )}
    </>
  );
}
