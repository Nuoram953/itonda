import { useState, useEffect } from "react";
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
import { GlobalSearchResultRow, type SearchAction } from "./GlobalSearchResultRow";

const KEY_HINTS = [
  { key: "↑↓", label: "Navigate" },
  { key: "Tab", label: "Switch action" },
  { key: "↵", label: "Select" },
];

export function GlobalSearch() {
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [selectedId, setSelectedId] = useState("");
  const [actionOverride, setActionOverride] = useState<SearchAction | null>(null);

  const navigate = useNavigate();
  const launcher = useMediaLauncher();
  const debouncedQuery = useDebounce(query, 250);
  const isQueryActive = debouncedQuery.trim().length > 0;

  const { data, isFetching } = useMedia({
    params: {
      search: debouncedQuery.trim(),
      limit: 8,
      sort_by: "last_played_at",
      sort_order: "desc",
    },
    queryConfig: { enabled: isQueryActive && open },
  });

  const results = isQueryActive ? (data?.items ?? []) : [];
  const isLoading = (isFetching || query.trim() !== debouncedQuery.trim()) && query.trim().length > 0;

  const activeItem = results.find((i) => i.id === selectedId) ?? results[0];
  const canLaunch = Boolean(activeItem && launcher.canLaunchMedia(activeItem));
  const activeAction: SearchAction = canLaunch && actionOverride !== "detail" ? "start" : "detail";

  useEffect(() => {
    const onGlobalKey = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && (e.key === "k" || e.key === "K")) {
        e.preventDefault();
        setOpen((prev) => !prev);
      }
    };
    window.addEventListener("keydown", onGlobalKey);
    return () => window.removeEventListener("keydown", onGlobalKey);
  }, []);

  const onExecuteAction = (item: MediaItem, action: SearchAction) => {
    setOpen(false);
    if (action === "start") {
      launcher.handleLaunchMedia(item);
    } else {
      navigate({ to: "/media/$mediaId", params: { mediaId: item.id } });
    }
  };

  const onKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Tab" && canLaunch) {
      e.preventDefault();
      setActionOverride((prev) => (prev === "detail" ? "start" : "detail"));
    }
  };

  const onDialogOpenChange = (nextOpen: boolean) => {
    setOpen(nextOpen);
    if (!nextOpen) {
      setQuery("");
      setSelectedId("");
      setActionOverride(null);
    }
  };

  return (
    <>
      <Button
        variant="outline"
        onClick={() => setOpen(true)}
        className="hidden md:flex items-center justify-between gap-2 w-72 lg:w-80 h-9 px-3 rounded-xl border-white/10 bg-surface-raised/50 text-xs text-text-muted hover:text-foreground hover:border-primary/50"
      >
        <span className="flex items-center gap-2">
          <Search className="w-4 h-4 shrink-0" />
          <span>Search media, games, movies...</span>
        </span>
        <kbd className="hidden lg:inline-flex px-1.5 py-0.5 rounded border border-white/10 bg-surface font-mono text-[10px]">
          ⌘K
        </kbd>
      </Button>

      <CommandDialog
        open={open}
        onOpenChange={onDialogOpenChange}
        title="Search Media"
        description="Search games, movies, and TV series in your library"
        className="sm:max-w-2xl bg-surface/95 backdrop-blur-xl"
      >
        <Command
          shouldFilter={false}
          value={activeItem?.id ?? ""}
          onValueChange={(val) => {
            setSelectedId(val);
            setActionOverride(null);
          }}
          onKeyDown={onKeyDown}
        >
          <CommandInput
            placeholder="Search media, games, movies..."
            value={query}
            onValueChange={setQuery}
          />

          <CommandList className="max-h-[60vh] p-2">
            {isLoading && (
              <div className="flex items-center justify-center gap-2 p-8 text-xs text-text-muted">
                <Loader2 className="w-4 h-4 animate-spin text-primary" />
                <span>Searching media...</span>
              </div>
            )}

            {!isLoading && query.trim() && results.length === 0 && (
              <CommandEmpty className="p-8 text-center text-xs text-text-muted">
                No media found matching &ldquo;{debouncedQuery || query}&rdquo;
              </CommandEmpty>
            )}

            {results.length > 0 && (
              <CommandGroup heading="Media">
                {results.map((item) => (
                  <GlobalSearchResultRow
                    key={item.id}
                    item={item}
                    isSelected={item.id === activeItem?.id}
                    activeAction={item.id === activeItem?.id ? activeAction : "detail"}
                    launcher={launcher}
                    onSelect={(action) => onExecuteAction(item, action)}
                    onHover={(action) => {
                      setSelectedId(item.id);
                      if (action) setActionOverride(action);
                    }}
                  />
                ))}
              </CommandGroup>
            )}

            <div className="flex items-center justify-between border-t border-white/10 px-3 py-2 text-[11px] text-text-muted">
              <div className="flex items-center gap-3">
                {KEY_HINTS.map(({ key, label }) => (
                  <span key={key} className="flex items-center gap-1">
                    <kbd className="rounded border border-white/10 bg-surface px-1 py-0.5 font-mono text-[10px]">{key}</kbd>
                    <span>{label}</span>
                  </span>
                ))}
              </div>
              <span className="flex items-center gap-1">
                <kbd className="rounded border border-white/10 bg-surface px-1 py-0.5 font-mono text-[10px]">Esc</kbd>
                <span>Close</span>
              </span>
            </div>
          </CommandList>
        </Command>
      </CommandDialog>

      {launcher.dialogProps && <LaunchProfileDialog {...launcher.dialogProps} />}
    </>
  );
}
