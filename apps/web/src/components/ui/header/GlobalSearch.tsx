import { useState, useEffect, useMemo, useCallback } from "react";
import { useNavigate } from "@tanstack/react-router";
import { Loader2, Search } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Command,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandList,
} from "@/components/ui/command";
import { useMedia } from "@/api/get-media";
import { useDebounce } from "@/hooks/use-debounce";
import { useMediaLauncher, type MediaItem } from "@/hooks/use-media-launcher";
import { LaunchProfileDialog } from "@/features/details/components/action/LaunchProfileDialog";
import { GlobalSearchResultRow } from "./GlobalSearchResultRow";

export type GlobalSearchProps = {
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
  defaultOpen?: boolean;
};

export function GlobalSearch({
  open: controlledOpen,
  onOpenChange: controlledOnOpenChange,
  defaultOpen = false,
}: GlobalSearchProps = {}) {
  const [internalOpen, setInternalOpen] = useState(defaultOpen);
  const open = controlledOpen !== undefined ? controlledOpen : internalOpen;

  const setOpen = useCallback(
    (nextOpen: boolean | ((prev: boolean) => boolean)) => {
      setInternalOpen((currentInternal) => {
        const currentEffective =
          controlledOpen !== undefined ? controlledOpen : currentInternal;
        const resolved =
          typeof nextOpen === "function"
            ? nextOpen(currentEffective)
            : nextOpen;
        controlledOnOpenChange?.(resolved);
        return resolved;
      });
    },
    [controlledOpen, controlledOnOpenChange],
  );

  const [query, setQuery] = useState("");
  const [selectedId, setSelectedId] = useState("");
  const [actionOverride, setActionOverride] = useState<
    "start" | "detail" | null
  >(null);

  const navigate = useNavigate();
  const launcher = useMediaLauncher();

  const debouncedQuery = useDebounce(query, 250);
  const isQueryActive = debouncedQuery.trim().length > 0;
  const isDebouncing =
    query.trim().length > 0 && query.trim() !== debouncedQuery.trim();

  const { data, isFetching } = useMedia({
    params: {
      search: debouncedQuery.trim(),
      limit: 8,
      sort_by: "last_played_at",
      sort_order: "desc",
    },
    queryConfig: {
      enabled: isQueryActive && open,
    },
  });

  const results: MediaItem[] = useMemo(
    () => (isQueryActive ? (data?.items ?? []) : []),
    [isQueryActive, data?.items],
  );
  const isLoading = (isDebouncing || isFetching) && query.trim().length > 0;

  // Active item based on effectiveSelectedId
  const effectiveSelectedId =
    selectedId && results.some((item) => item.id === selectedId)
      ? selectedId
      : (results[0]?.id ?? "");

  const activeItem =
    results.find((item) => item.id === effectiveSelectedId) ?? results[0];

  const canLaunchActiveItem = Boolean(
    activeItem && launcher.canLaunchMedia(activeItem),
  );

  const defaultActionForActiveItem: "start" | "detail" = canLaunchActiveItem
    ? "start"
    : "detail";

  const activeAction =
    actionOverride !== null && canLaunchActiveItem
      ? actionOverride
      : defaultActionForActiveItem;

  // Global shortcut: ⌘K or Ctrl+K to toggle dialog
  useEffect(() => {
    const handleGlobalKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && (e.key === "k" || e.key === "K")) {
        e.preventDefault();
        setOpen((prev) => !prev);
      }
    };

    window.addEventListener("keydown", handleGlobalKeyDown);
    return () => window.removeEventListener("keydown", handleGlobalKeyDown);
  }, [setOpen]);

  const handleExecuteAction = (item: MediaItem, action: "start" | "detail") => {
    if (action === "start") {
      launcher.handleLaunchMedia(item, {
        onSuccess: () => setOpen(false),
      });
      if ((item.launches?.length ?? 0) > 1) {
        setOpen(false);
      }
    } else {
      setOpen(false);
      navigate({
        to: "/media/$mediaId",
        params: { mediaId: item.id },
      });
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Tab") {
      e.preventDefault();
      // Option A: Strictly toggle between 'start' and 'detail' on the currently active item
      if (canLaunchActiveItem) {
        setActionOverride((prev) => {
          const current = prev !== null ? prev : defaultActionForActiveItem;
          return current === "start" ? "detail" : "start";
        });
      }
    }
  };

  const handleOpenChange = (nextOpen: boolean) => {
    setOpen(nextOpen);
    if (!nextOpen) {
      setQuery("");
      setSelectedId("");
      setActionOverride(null);
    }
  };

  return (
    <>
      {/* Header Search Trigger Button */}
      <Button
        variant="outline"
        onClick={() => setOpen(true)}
        className="hidden md:flex items-center justify-between gap-2.5 bg-surface-raised/50 border border-white/10 hover:border-primary/50 text-xs text-text-muted hover:text-foreground rounded-xl px-3.5 py-1.5 w-72 lg:w-96 cursor-pointer font-normal h-9 shadow-none"
      >
        <div className="flex items-center gap-2">
          <Search className="w-4 h-4 text-text-muted shrink-0" />
          <span className="text-text-muted/70">
            Search media, games, movies...
          </span>
        </div>
        <kbd className="hidden lg:inline-flex items-center gap-0.5 rounded bg-surface border border-white/10 px-1.5 text-xs text-text-muted font-mono">
          ⌘K
        </kbd>
      </Button>

      {/* shadcn CommandDialog */}
      <CommandDialog
        open={open}
        onOpenChange={handleOpenChange}
        title="Search Media"
        description="Search games, movies, and TV series in your library"
        className="sm:max-w-2xl bg-surface/95 border border-white/10 shadow-2xl backdrop-blur-xl text-foreground"
      >
        <Command
          shouldFilter={false}
          value={effectiveSelectedId}
          onValueChange={(val) => {
            setSelectedId(val);
            setActionOverride(null);
          }}
          onKeyDown={handleKeyDown}
          className="bg-transparent text-foreground"
        >
          <CommandInput
            placeholder="Search media, games, movies..."
            value={query}
            onValueChange={setQuery}
          />

          <CommandList className="max-h-[60vh] p-2">
            {isLoading && results.length === 0 ? (
              <div className="flex items-center justify-center gap-2 p-8 text-xs text-text-muted">
                <Loader2 className="w-4 h-4 animate-spin text-primary" />
                <span>Searching media...</span>
              </div>
            ) : results.length === 0 && query.trim() ? (
              <CommandEmpty className="p-8 text-center text-xs text-text-muted">
                No media found matching &ldquo;{debouncedQuery || query}&rdquo;
              </CommandEmpty>
            ) : results.length > 0 ? (
              <CommandGroup heading="Media">
                {results.map((item) => {
                  const isSelected = item.id === activeItem?.id;
                  return (
                    <GlobalSearchResultRow
                      key={item.id}
                      item={item}
                      isSelected={isSelected}
                      activeAction={isSelected ? activeAction : "detail"}
                      onSelect={(action) => handleExecuteAction(item, action)}
                      onHover={() => setSelectedId(item.id)}
                      onHoverAction={(action) => {
                        setSelectedId(item.id);
                        setActionOverride(action);
                      }}
                      canLaunch={launcher.canLaunchMedia(item)}
                      disabledReason={launcher.getDisabledReason(item)}
                      isCurrentMediaPlaying={launcher.isMediaPlaying(item.id)}
                    />
                  );
                })}
              </CommandGroup>
            ) : null}

            {/* Footer keyboard hints */}
            <div className="flex items-center justify-between border-t border-white/10 px-3 py-2 text-[11px] text-text-muted bg-surface/50 -mx-2 -mb-2 mt-2">
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
          </CommandList>
        </Command>
      </CommandDialog>

      {/* Shared multi-profile selection modal */}
      {launcher.activeDialogMedia && (
        <LaunchProfileDialog
          open={Boolean(launcher.activeDialogMedia)}
          onOpenChange={(nextOpen) =>
            !nextOpen && launcher.setActiveDialogMedia(null)
          }
          title={launcher.activeDialogMedia.title}
          profiles={launcher.activeDialogMedia.launches}
          isProfileAvailable={launcher.isProfileAvailable}
          onSelectProfile={(profileId) =>
            launcher.launch(profileId, {
              mediaTitle: launcher.activeDialogMedia?.title,
            })
          }
          isLoading={launcher.isLoading}
        />
      )}
    </>
  );
}
