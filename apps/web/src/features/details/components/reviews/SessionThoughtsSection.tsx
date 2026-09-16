import { useState } from "react";
import { Plus, MessageSquareDashed } from "lucide-react";
import { Button } from "@/components/ui/button";
import type { GameThought } from "../../types/reviews";
import { useThoughtActions } from "../../hooks/use-thought-actions";
import { ThoughtCard } from "./ThoughtCard";
import { ThoughtDialog } from "./ThoughtDialog";

type SessionThoughtsSectionProps = {
  mediaId: string;
  thoughts: GameThought[];
  defaultPlaytimeMinutes?: number;
  onCreateThought?: (data: {
    title: string;
    content: string;
    category?: string;
    playtime_minutes?: number;
  }) => Promise<void>;
  onUpdateThought?: (
    thoughtId: string,
    data: {
      title: string;
      content: string;
      category?: string;
    },
  ) => Promise<void>;
  onDeleteThought?: (thoughtId: string) => Promise<void>;
};

export function SessionThoughtsSection({
  mediaId,
  thoughts,
  defaultPlaytimeMinutes,
  onCreateThought,
  onUpdateThought,
  onDeleteThought,
}: SessionThoughtsSectionProps) {
  const [dialogOpen, setDialogOpen] = useState(false);
  const [editingThought, setEditingThought] = useState<GameThought | null>(null);

  const actions = useThoughtActions({ mediaId });
  const createThought = onCreateThought ?? actions.createThought;
  const updateThought = onUpdateThought ?? actions.updateThought;
  const deleteThought = onDeleteThought ?? actions.deleteThought;

  const handleOpenCreate = () => {
    setEditingThought(null);
    setDialogOpen(true);
  };

  const handleOpenEdit = (thought: GameThought) => {
    setEditingThought(thought);
    setDialogOpen(true);
  };

  const handleSaveThought = async (data: {
    title: string;
    content: string;
    playtime_minutes?: number;
  }) => {
    if (editingThought) {
      await updateThought(editingThought.id, {
        title: data.title,
        content: data.content,
      });
    } else {
      await createThought(data);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-white/10 pb-4">
        <div>
          <h3 className="text-xl font-bold text-foreground tracking-tight flex items-center gap-2.5">
            <span>Session Thoughts & Notes</span>
            <span className="text-xs font-semibold px-2.5 py-0.5 rounded-full bg-surface/80 border border-white/10 text-text-muted">
              {thoughts.length}
            </span>
          </h3>
          <p className="text-xs sm:text-sm text-text-muted mt-1">
            Jot down thoughts and highlights after sessions so you can easily find them later.
          </p>
        </div>

        {thoughts.length > 0 && (
          <Button
            variant="outline"
            size="sm"
            onClick={handleOpenCreate}
            className="gap-1.5 text-xs text-foreground border-white/10 hover:border-white/20 hover:bg-white/5 cursor-pointer self-start sm:self-auto"
          >
            <Plus className="w-3.5 h-3.5" />
            <span>Add Thought</span>
          </Button>
        )}
      </div>

      {thoughts.length === 0 ? (
        <div className="rounded-2xl border border-dashed border-white/15 bg-surface/30 p-10 text-center flex flex-col items-center justify-center space-y-4">
          <div className="p-4 rounded-2xl bg-surface-raised/80 border border-white/10 text-text-muted">
            <MessageSquareDashed className="w-8 h-8" />
          </div>
          <div className="space-y-1 max-w-sm">
            <h4 className="text-base font-bold text-foreground">
              No session thoughts yet
            </h4>
            <p className="text-xs sm:text-sm text-text-muted">
              Did something catch your ear or challenge your playstyle? Record quick impressions and find them whenever you want.
            </p>
          </div>
          <Button
            variant="outline"
            onClick={handleOpenCreate}
            className="font-medium gap-1.5 border-white/15 hover:border-white/30 hover:bg-white/5 cursor-pointer"
          >
            <Plus className="w-4 h-4" />
            <span>Add Your First Thought</span>
          </Button>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {thoughts.map((thought) => (
            <ThoughtCard
              key={thought.id}
              thought={thought}
              onEdit={handleOpenEdit}
              onDelete={deleteThought}
            />
          ))}
        </div>
      )}

      <ThoughtDialog
        open={dialogOpen}
        onOpenChange={setDialogOpen}
        initialThought={editingThought}
        defaultPlaytimeMinutes={defaultPlaytimeMinutes}
        onSave={handleSaveThought}
      />
    </div>
  );
}
