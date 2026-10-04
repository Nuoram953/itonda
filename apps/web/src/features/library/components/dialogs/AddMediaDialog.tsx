import { Dialog } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Form } from "@/components/ui/form";
import { Search, Loader2, X, Plus } from "lucide-react";
import { MediaTypeSelector, MEDIA_TYPES } from "./MediaTypeSelector";
import { SearchResultRow } from "./SearchResultRow";
import { useAddMediaDialog } from "./use-add-media-dialog";

type AddMediaDialogProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
};

function AddMediaForm({ onCancel }: { onCancel: () => void }) {
  const {
    selectedType,
    displayTitle,
    debouncedQuery,
    selectedResult,
    searchResults,
    isLoading,
    shouldSearch,
    canSubmit,
    isAdding,
    selectType,
    setQuery,
    selectResult,
    clearSearch,
    submit,
  } = useAddMediaDialog({ onSuccess: onCancel });

  const typeLabel =
    MEDIA_TYPES.find((m) => m.type === selectedType)?.label.toLowerCase() ||
    "media";

  return (
    <>
      <Dialog.Header icon={Plus} className="pb-4">
        <Dialog.Title>Add Media</Dialog.Title>
        <Dialog.Description>
          Choose the media type and search for a title to add to your library.
        </Dialog.Description>
      </Dialog.Header>

      <div className="pb-4 shrink-0">
        <MediaTypeSelector selected={selectedType} onSelect={selectType} />
      </div>

      <div className="flex-1 min-h-0 flex flex-col">
        {selectedType ? (
          <div className="flex-1 min-h-0 flex flex-col space-y-3">
            <Form.Field className="shrink-0">
              <Form.Label>2. Search Title</Form.Label>
              <Form.Control>
                <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-text-muted pointer-events-none" />
                <Form.Input
                  value={displayTitle}
                  onChange={(e) => setQuery(e.target.value)}
                  placeholder={`Type a few letters to search for ${typeLabel}...`}
                  className="pl-9 pr-8"
                  autoFocus
                />
                {displayTitle && (
                  <button
                    type="button"
                    onClick={clearSearch}
                    className="absolute right-2.5 top-1/2 -translate-y-1/2 text-text-muted hover:text-foreground p-1 rounded cursor-pointer"
                  >
                    <X className="w-3.5 h-3.5" />
                  </button>
                )}
              </Form.Control>
            </Form.Field>

            <div className="flex-1 min-h-0 overflow-hidden relative rounded-xl border border-white/5 bg-surface/20 p-2">
              {isLoading ? (
                <div className="h-full flex flex-col items-center justify-center gap-2 text-xs text-text-muted">
                  <Loader2 className="w-5 h-5 animate-spin text-primary" />
                  <span>Searching catalog...</span>
                </div>
              ) : shouldSearch && searchResults ? (
                searchResults.length === 0 ? (
                  <div className="h-full flex flex-col items-center justify-center text-center p-6 text-text-muted">
                    <p className="text-xs">
                      No results found for &ldquo;{debouncedQuery}&rdquo;.
                    </p>
                    <p className="text-[11px] text-text-muted/70 mt-1">
                      You can still add this title manually using the button
                      below.
                    </p>
                  </div>
                ) : (
                  <div className="h-full overflow-y-auto space-y-1.5 pr-1">
                    {searchResults.map((result) => (
                      <SearchResultRow
                        key={result.external_id}
                        result={result}
                        isSelected={
                          selectedResult?.external_id === result.external_id
                        }
                        onSelect={() => selectResult(result)}
                      />
                    ))}
                  </div>
                )
              ) : (
                <div className="h-full flex flex-col items-center justify-center text-center p-6 text-text-muted/60">
                  <Search className="w-6 h-6 mb-1.5 opacity-30" />
                  <p className="text-xs">
                    Type at least 2 characters to search catalog
                  </p>
                  <p className="text-[11px] text-text-muted/40 mt-0.5">
                    Or enter a custom title and add it directly
                  </p>
                </div>
              )}
            </div>
          </div>
        ) : (
          <div className="flex-1 min-h-0 flex flex-col items-center justify-center text-center p-6 text-text-muted/60 border border-dashed border-white/5 rounded-xl bg-surface/10">
            <Search className="w-8 h-8 mb-2 opacity-30" />
            <p className="text-xs">
              Choose a media type above to begin searching.
            </p>
          </div>
        )}
      </div>

      <Dialog.Footer className="pt-4 mt-auto shrink-0 border-t border-border/40 flex items-center justify-end gap-3">
        <Button
          type="button"
          variant="ghost"
          onClick={onCancel}
          disabled={isAdding}
        >
          Cancel
        </Button>
        <Button
          type="button"
          onClick={submit}
          disabled={!canSubmit}
          loading={isAdding}
          loadingText="Adding..."
        >
          <Plus className="w-4 h-4" />
          <span>Add to Library</span>
        </Button>
      </Dialog.Footer>
    </>
  );
}

export function AddMediaDialog({ open, onOpenChange }: AddMediaDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <Dialog.Content className="w-full max-w-[calc(100%-2rem)] sm:max-w-xl md:max-w-2xl h-[560px] max-h-[88vh] flex flex-col p-4 sm:p-6 gap-0">
        {open && <AddMediaForm onCancel={() => onOpenChange(false)} />}
      </Dialog.Content>
    </Dialog>
  );
}
