import { useEffect, useMemo, useState } from "react";
import {
  ArrowUp,
  File as FileIcon,
  Folder,
  Loader2,
  Search,
} from "lucide-react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useAgentFiles } from "@/api/list-agent-files";

type AgentFileBrowserDialogProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  agentId?: string;
  initialPath?: string;
  onSelect: (path: string) => void;
};

function getInitialBrowsePath(initialPath?: string): string | undefined {
  const trimmed = initialPath?.trim();
  if (!trimmed) return undefined;
  const lastSlash = Math.max(
    trimmed.lastIndexOf("/"),
    trimmed.lastIndexOf("\\"),
  );
  if (lastSlash > 0) {
    return trimmed.slice(0, lastSlash);
  }
  return trimmed;
}

export function AgentFileBrowserDialog({
  open,
  onOpenChange,
  agentId,
  initialPath,
  onSelect,
}: AgentFileBrowserDialogProps) {
  const [browsePath, setBrowsePath] = useState<string | undefined>(() =>
    getInitialBrowsePath(initialPath),
  );
  const [pathInput, setPathInput] = useState("");
  const [filterText, setFilterText] = useState("");
  const [selectedPath, setSelectedPath] = useState<string | null>(null);

  useEffect(() => {
    if (open) {
      setBrowsePath(getInitialBrowsePath(initialPath));
      setFilterText("");
      setSelectedPath(null);
    }
  }, [open, initialPath]);

  const { data, isLoading, error } = useAgentFiles(
    { agentId, path: browsePath },
    open,
  );

  useEffect(() => {
    if (data?.path) {
      setPathInput(data.path);
    }
  }, [data?.path]);

  const filteredEntries = useMemo(() => {
    const entries = data?.entries ?? [];
    const q = filterText.trim().toLowerCase();
    if (!q) return entries;
    return entries.filter((entry) => entry.name.toLowerCase().includes(q));
  }, [data?.entries, filterText]);

  const navigateTo = (targetPath: string) => {
    setSelectedPath(null);
    setFilterText("");
    setBrowsePath(targetPath);
  };

  const handleGoToPath = (e: React.FormEvent) => {
    e.preventDefault();
    const trimmed = pathInput.trim();
    if (trimmed) {
      navigateTo(trimmed);
    }
  };

  const handleConfirm = () => {
    if (selectedPath) {
      onSelect(selectedPath);
      onOpenChange(false);
    }
  };

  const errorMessage =
    error instanceof Error
      ? error.message
      : data?.error
        ? data.error
        : null;

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[600px]">
        <DialogHeader>
          <DialogTitle>Select Game File</DialogTitle>
          <DialogDescription>
            Browse files on the connected agent machine.
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-3 py-1">
          <form onSubmit={handleGoToPath} className="flex items-center gap-2">
            <Button
              type="button"
              variant="outline"
              size="icon"
              disabled={!data?.parent || isLoading}
              onClick={() => data?.parent && navigateTo(data.parent)}
              title="Parent directory"
            >
              <ArrowUp className="h-4 w-4" />
            </Button>
            <Input
              value={pathInput}
              onChange={(e) => setPathInput(e.target.value)}
              placeholder="/path/to/roms"
              className="font-mono text-xs"
            />
            <Button type="submit" variant="outline" size="sm" disabled={isLoading}>
              Go
            </Button>
          </form>

          <div className="relative">
            <Search className="pointer-events-none absolute left-2.5 top-2.5 h-4 w-4 text-text-muted" />
            <Input
              value={filterText}
              onChange={(e) => setFilterText(e.target.value)}
              placeholder="Filter current folder..."
              className="pl-8 text-sm"
            />
          </div>

          <div className="h-72 overflow-y-auto rounded-lg border border-border/60 bg-surface-raised/20 p-1">
            {isLoading ? (
              <div className="flex h-full items-center justify-center gap-2 text-sm text-text-muted">
                <Loader2 className="h-4 w-4 animate-spin" />
                Loading directory...
              </div>
            ) : errorMessage ? (
              <div className="flex h-full items-center justify-center px-4 text-center text-sm text-destructive">
                {errorMessage}
              </div>
            ) : filteredEntries.length === 0 ? (
              <div className="flex h-full items-center justify-center text-sm text-text-muted">
                No matching files or folders.
              </div>
            ) : (
              <div className="space-y-0.5">
                {filteredEntries.map((entry) => {
                  const isSelected = !entry.is_dir && selectedPath === entry.path;
                  return (
                    <button
                      key={entry.path}
                      type="button"
                      onClick={() => {
                        if (entry.is_dir) {
                          navigateTo(entry.path);
                        } else {
                          setSelectedPath(entry.path);
                        }
                      }}
                      onDoubleClick={() => {
                        if (!entry.is_dir) {
                          onSelect(entry.path);
                          onOpenChange(false);
                        }
                      }}
                      className={`flex w-full items-center gap-2.5 rounded-md px-2.5 py-1.5 text-left text-sm transition-colors ${
                        isSelected
                          ? "bg-primary text-primary-foreground"
                          : "hover:bg-accent/60 text-foreground"
                      }`}
                    >
                      {entry.is_dir ? (
                        <Folder className="h-4 w-4 shrink-0 text-amber-400" />
                      ) : (
                        <FileIcon className="h-4 w-4 shrink-0 text-text-muted" />
                      )}
                      <span className="truncate font-mono text-xs">
                        {entry.name}
                      </span>
                    </button>
                  );
                })}
              </div>
            )}
          </div>

          {selectedPath && (
            <div className="rounded-md bg-surface-raised/40 px-2.5 py-1.5 font-mono text-xs text-text-muted truncate">
              Selected: <span className="text-foreground">{selectedPath}</span>
            </div>
          )}
        </div>

        <DialogFooter>
          <Button
            type="button"
            variant="ghost"
            onClick={() => onOpenChange(false)}
          >
            Cancel
          </Button>
          <Button
            type="button"
            disabled={!selectedPath}
            onClick={handleConfirm}
          >
            Select File
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
