import { useState } from "react";
import { Clock, Calendar, Edit2, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import type { GameThought } from "../../types/reviews";
import { ConfirmDialog } from "@/components/feedback/ConfirmDialog";
import { formatDate, formatDuration } from "@/utils/datetime";

type ThoughtCardProps = {
  thought: GameThought;
  onEdit: (thought: GameThought) => void;
  onDelete: (thoughtId: string) => Promise<void>;
};

export function ThoughtCard({ thought, onEdit, onDelete }: ThoughtCardProps) {
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);

  const formattedDate = formatDate(thought.created_at);

  const playtimeText =
    thought.playtime_minutes != null
      ? formatDuration(thought.playtime_minutes)
      : null;

  const handleDelete = async () => {
    try {
      setIsDeleting(true);
      await onDelete(thought.id);
    } finally {
      setIsDeleting(false);
    }
  };

  return (
    <>
      <div className="group relative rounded-2xl bg-surface/60 border border-white/10 hover:border-white/20 p-5 flex flex-col justify-between transition-all duration-200 shadow-md hover:shadow-lg">
        <div className="space-y-3">
          <div className="flex items-center justify-between gap-2">
            <h4 className="text-base font-bold text-foreground tracking-tight group-hover:text-accent-gold transition-colors">
              {thought.title}
            </h4>

            <div className="flex items-center gap-1 opacity-80 group-hover:opacity-100 transition-opacity">
              <Button
                variant="ghost"
                size="sm"
                onClick={() => onEdit(thought)}
                className="h-7 w-7 p-0 text-text-muted hover:text-white hover:bg-white/10 rounded-lg cursor-pointer"
                title="Edit thought"
              >
                <Edit2 className="w-3.5 h-3.5" />
              </Button>
              <Button
                variant="ghost"
                size="sm"
                onClick={() => setDeleteDialogOpen(true)}
                disabled={isDeleting}
                className="h-7 w-7 p-0 text-rose-400/80 hover:text-rose-300 hover:bg-rose-500/10 rounded-lg cursor-pointer"
                title="Delete thought"
              >
                <Trash2 className="w-3.5 h-3.5" />
              </Button>
            </div>
          </div>

          <p className="text-sm text-white/80 leading-relaxed whitespace-pre-line">
            {thought.content}
          </p>
        </div>

        <div className="pt-4 mt-3 border-t border-white/5 flex items-center justify-between text-xs text-text-muted">
          <div className="flex items-center gap-1.5">
            <Calendar className="w-3.5 h-3.5" />
            <span>{formattedDate}</span>
          </div>

          {playtimeText && (
            <div className="flex items-center gap-1.5">
              <Clock className="w-3.5 h-3.5" />
              <span>{playtimeText}</span>
            </div>
          )}
        </div>
      </div>

      <ConfirmDialog
        open={deleteDialogOpen}
        onOpenChange={setDeleteDialogOpen}
        title="Delete Session Note"
        description={`Are you sure you want to delete "${thought.title}"? This note will be permanently removed.`}
        confirmLabel="Delete Note"
        variant="destructive"
        isLoading={isDeleting}
        onConfirm={handleDelete}
      />
    </>
  );
}
