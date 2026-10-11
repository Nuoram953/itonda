import { useState, useEffect, useRef, useMemo } from "react";
import { useNavigate } from "@tanstack/react-router";
import { Loader2, Search, X } from "lucide-react";
import {
  Command,
  CommandList,
  CommandEmpty,
  CommandGroup,
} from "@/components/ui/command";
import { Command as CommandPrimitive } from "cmdk";
import { useMedia } from "@/api/get-media";
import { useDebounce } from "@/hooks/use-debounce";
import { useMediaLauncher, type MediaItem } from "@/hooks/use-media-launcher";
import { LaunchProfileDialog } from "@/features/details/components/action/LaunchProfileDialog";
import { GlobalSearchResultRow } from "./GlobalSearchResultRow";

export function GlobalSearch() {
  const [isOpen, setIsOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [selectedId, setSelectedId] = useState("");
  const [actionOverride, setActionOverride] = useState<
    "start" | "detail" | null
  >(null);

  const navigate = useNavigate();
  const launcher = useMediaLauncher();
  const inputRef = useRef<HTMLInputElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);

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
      enabled: isQueryActive,
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

  // Click outside listener
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        containerRef.current &&
        !containerRef.current.contains(event.target as Node)
      ) {
        setIsOpen(false);
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, []);

  // Global shortcut: ⌘K or Ctrl+K to focus input
  useEffect(() => {
    const handleGlobalKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && (e.key === "k" || e.key === "K")) {
        e.preventDefault();
        inputRef.current?.focus();
        if (query.trim().length > 0) {
          setIsOpen(true);
        }
      }
    };

    window.addEventListener("keydown", handleGlobalKeyDown);
    return () => window.removeEventListener("keydown", handleGlobalKeyDown);
  }, [query]);

  const handleExecuteAction = (item: MediaItem, action: "start" | "detail") => {
    if (action === "start") {
      launcher.handleLaunchMedia(item, {
        onSuccess: () => setIsOpen(false),
      });
      if ((item.launches?.length ?? 0) > 1) {
        setIsOpen(false);
      }
    } else {
      setIsOpen(false);
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
    } else if (e.key === "Escape") {
      e.preventDefault();
      setIsOpen(false);
    }
  };

  const clearQuery = () => {
    setQuery("");
    setSelectedId("");
    setActionOverride(null);
    setIsOpen(false);
  };

  return (
    <>
      <div ref={containerRef} className="relative w-72 lg:w-96">
        <Command
          shouldFilter={false}
          value={effectiveSelectedId}
          onValueChange={(val) => {
            setSelectedId(val);
            setActionOverride(null);
          }}
          onKeyDown={handleKeyDown}
          className="overflow-visible bg-transparent text-foreground"
        >
          {/* Search input container */}
          <div className="flex items-center gap-2.5 bg-surface-raised/50 border border-white/10 focus-within:border-primary/50 focus-within:ring-1 focus-within:ring-primary/40 rounded-xl px-3.5 py-1.5 w-full text-sm text-text-muted transition-all duration-200">
            {isLoading ? (
              <Loader2 className="w-4 h-4 text-primary shrink-0 animate-spin" />
            ) : (
              <Search className="w-4 h-4 text-text-muted shrink-0" />
            )}

            <CommandPrimitive.Input
              ref={inputRef}
              placeholder="Search media, games, movies..."
              value={query}
              onValueChange={(val) => {
                setQuery(val);
                if (!isOpen) setIsOpen(true);
              }}
              onFocus={() => {
                if (query.trim().length > 0) setIsOpen(true);
              }}
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

          {/* Results Popover using CommandList */}
          {isOpen && query.trim().length > 0 && (
            <div className="absolute top-full mt-2 left-0 w-full sm:w-[480px] lg:w-[540px] z-50 rounded-2xl bg-surface/95 border border-white/10 shadow-2xl backdrop-blur-xl overflow-hidden animate-in fade-in-0 zoom-in-95">
              <CommandList className="max-h-[60vh] p-2 overflow-y-auto">
                {isLoading && results.length === 0 ? (
                  <div className="flex items-center justify-center gap-2 p-8 text-xs text-text-muted">
                    <Loader2 className="w-4 h-4 animate-spin text-primary" />
                    <span>Searching media...</span>
                  </div>
                ) : results.length === 0 ? (
                  <CommandEmpty className="p-8 text-center text-xs text-text-muted">
                    No media found matching &ldquo;{debouncedQuery || query}
                    &rdquo;
                  </CommandEmpty>
                ) : (
                  <CommandGroup>
                    {results.map((item) => {
                      const isSelected = item.id === activeItem?.id;
                      return (
                        <GlobalSearchResultRow
                          key={item.id}
                          item={item}
                          isSelected={isSelected}
                          activeAction={isSelected ? activeAction : "detail"}
                          onSelect={(action) =>
                            handleExecuteAction(item, action)
                          }
                          onHover={() => setSelectedId(item.id)}
                          onHoverAction={(action) => {
                            setSelectedId(item.id);
                            setActionOverride(action);
                          }}
                          canLaunch={launcher.canLaunchMedia(item)}
                          disabledReason={launcher.getDisabledReason(item)}
                          isCurrentMediaPlaying={launcher.isMediaPlaying(
                            item.id,
                          )}
                        />
                      );
                    })}
                  </CommandGroup>
                )}

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
            </div>
          )}
        </Command>
      </div>

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
