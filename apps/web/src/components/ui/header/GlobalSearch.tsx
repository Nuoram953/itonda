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
import { LaunchProfileDialog } from "@/features/details/components/action/LaunchProfileDialog";
import { useShortcut, useShortcuts } from "@/features/shortcuts";
import { GlobalSearchResultRow } from "./GlobalSearchResultRow";
import { useGlobalSearch } from "./use-global-search";

export function GlobalSearch() {
  const search = useGlobalSearch();
  const shortcuts = useShortcuts();

  // Register the global shortcut to toggle search dialog (works even when inputs are focused)
  useShortcut("search.open", () => search.setOpen((prev) => !prev), {
    ignoreInputs: false,
  });

  const onKeyDown = (e: React.KeyboardEvent) => {
    if (
      shortcuts.matches(e.nativeEvent, "search.switchAction") &&
      search.canLaunch
    ) {
      e.preventDefault();
      search.toggleActionOverride();
    }
  };

  const keyHints = [
    {
      key: shortcuts.getDisplayString("search.navigate"),
      label: "Navigate",
    },
    {
      key: shortcuts.getDisplayString("search.switchAction"),
      label: "Switch action",
    },
    {
      key: shortcuts.getDisplayString("search.select"),
      label: "Select",
    },
  ];

  return (
    <>
      <Button
        variant="outline"
        onClick={() => search.setOpen(true)}
        className="hidden md:flex items-center justify-between gap-2 w-72 lg:w-80 h-9 px-3 rounded-xl border-white/10 bg-surface-raised/50 text-xs text-text-muted hover:text-foreground hover:border-primary/50"
      >
        <span className="flex items-center gap-2">
          <Search className="w-4 h-4 shrink-0" />
          <span>Search media, games, movies...</span>
        </span>
        <kbd>{shortcuts.getDisplayString("search.open")}</kbd>
      </Button>

      <CommandDialog
        open={search.open}
        onOpenChange={search.handleDialogOpenChange}
        title="Search Media"
        description="Search games, movies, and TV series in your library"
        className="sm:max-w-2xl bg-surface/95 backdrop-blur-xl"
      >
        <Command
          shouldFilter={false}
          value={search.activeItem?.id ?? ""}
          onValueChange={(val) => {
            search.setSelectedId(val);
            search.setActionOverride(null);
          }}
          onKeyDown={onKeyDown}
        >
          <CommandInput
            placeholder="Search media, games, movies..."
            value={search.query}
            onValueChange={search.setQuery}
          />

          <CommandList className="max-h-[60vh] p-2">
            {search.isLoading && (
              <div className="flex items-center justify-center gap-2 p-8 text-xs text-text-muted">
                <Loader2 className="w-4 h-4 animate-spin text-primary" />
                <span>Searching media...</span>
              </div>
            )}

            {!search.isLoading &&
              search.query.trim() &&
              search.results.length === 0 && (
                <CommandEmpty className="p-8 text-center text-xs text-text-muted">
                  No media found matching &ldquo;
                  {search.debouncedQuery || search.query}&rdquo;
                </CommandEmpty>
              )}

            {search.results.length > 0 && (
              <CommandGroup heading="Media">
                {search.results.map((item) => (
                  <GlobalSearchResultRow
                    key={item.id}
                    item={item}
                    isSelected={item.id === search.activeItem?.id}
                    activeAction={
                      item.id === search.activeItem?.id
                        ? search.activeAction
                        : "detail"
                    }
                    launcher={search.launcher}
                    onSelect={(action) => search.executeAction(item, action)}
                    onHover={(action) => search.handleHover(item.id, action)}
                  />
                ))}
              </CommandGroup>
            )}

            <div className="flex items-center justify-between border-t border-white/10 px-3 py-2 text-[11px] text-text-muted">
              <div className="flex items-center gap-3">
                {keyHints.map(({ key, label }) => (
                  <span key={key} className="flex items-center gap-1">
                    <kbd className="rounded border border-white/10 bg-surface px-1 py-0.5 font-mono text-[10px]">
                      {key}
                    </kbd>
                    <span>{label}</span>
                  </span>
                ))}
              </div>
              <span className="flex items-center gap-1">
                <kbd className="rounded border border-white/10 bg-surface px-1 py-0.5 font-mono text-[10px]">
                  {shortcuts.getDisplayString("search.close")}
                </kbd>
                <span>Close</span>
              </span>
            </div>
          </CommandList>
        </Command>
      </CommandDialog>

      {search.launcher.dialogProps && (
        <LaunchProfileDialog {...search.launcher.dialogProps} />
      )}
    </>
  );
}
