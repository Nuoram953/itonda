import { useNavigate } from "@tanstack/react-router";
import { Loader2, Search, X } from "lucide-react";
import { useMediaLauncher } from "@/hooks/use-media-launcher";
import { LaunchProfileDialog } from "@/features/details/components/action/LaunchProfileDialog";
import {
  useGlobalSearch,
  type MediaItem,
  type SearchAction,
} from "./use-global-search";
import { GlobalSearchResultRow } from "./GlobalSearchResultRow";

export function GlobalSearch() {
  const navigate = useNavigate();
  const launcher = useMediaLauncher();

  const handleStartGame = (item: MediaItem) => {
    launcher.handleLaunchMedia(item, {
      onSuccess: () => setIsOpen(false),
    });

    if ((item.launches?.length ?? 0) > 1) {
      setIsOpen(false);
    }
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
                        canLaunch={launcher.canLaunchMedia(item)}
                        disabledReason={launcher.getDisabledReason(item)}
                        isCurrentMediaPlaying={launcher.isMediaPlaying(item.id)}
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

      {/* Shared multi-profile selection modal */}
      {launcher.activeDialogMedia && (
        <LaunchProfileDialog
          open={Boolean(launcher.activeDialogMedia)}
          onOpenChange={(open) =>
            !open && launcher.setActiveDialogMedia(null)
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
