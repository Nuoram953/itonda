import { useState } from "react";
import { Clock, Calendar, Edit2, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import type { GameThought } from "../../types/reviews";
import { THOUGHT_CATEGORIES } from "../../constants/reviews";

type ThoughtCardProps = {
  thought: GameThought;
  onEdit: (thought: GameThought) => void;
  onDelete: (thoughtId: string) => Promise<void>;
};

export function ThoughtCard({ thought, onEdit, onDelete }: ThoughtCardProps) {
  const [isDeleting, setIsDeleting] = useState(false);

  const categoryConfig =
    THOUGHT_CATEGORIES.find((c) => c.id === thought.category) ||
    THOUGHT_CATEGORIES.find((c) => c.id === "general")!;

  const CategoryIcon = categoryConfig.icon;

  const formattedDate = new Date(thought.created_at).toLocaleDateString(
    undefined,
    {
      year: "numeric",
      month: "short",
      day: "numeric",
    },
  );

  const playtimeText =
    thought.playtime_minutes != null
      ? `${(thought.playtime_minutes / 60).toFixed(1)}h`
      : null;

  const handleDelete = async () => {
    if (!window.confirm("Are you sure you want to delete this session note?")) {
      return;
    }
    try {
      setIsDeleting(true);
      await onDelete(thought.id);
    } finally {
      setIsDeleting(false);
    }
  };

  return (
    <div className="group relative rounded-2xl bg-surface/60 border border-white/10 hover:border-white/20 p-5 flex flex-col justify-between transition-all duration-200 shadow-md hover:shadow-lg">
      <div className="space-y-3">
        <div className="flex items-center justify-between gap-2">
          <div className="flex flex-wrap items-center gap-2">
            <span
              className={cn(
                "inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-xs font-semibold border",
                categoryConfig.badgeStyle,
              )}
            >
              <CategoryIcon className={cn("w-3.5 h-3.5", categoryConfig.color)} />
              <span>{categoryConfig.label}</span>
            </span>

            {playtimeText && (
              <span className="inline-flex items-center gap-1 text-[11px] text-text-muted bg-white/5 px-2 py-0.5 rounded-md border border-white/5">
                <Clock className="w-3 h-3 text-accent-gold" />
                <span>{playtimeText}</span>
              </span>
            )}
          </div>

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
              onClick={handleDelete}
              disabled={isDeleting}
              className="h-7 w-7 p-0 text-rose-400/80 hover:text-rose-300 hover:bg-rose-500/10 rounded-lg cursor-pointer"
              title="Delete thought"
            >
              <Trash2 className="w-3.5 h-3.5" />
            </Button>
          </div>
        </div>

        <h4 className="text-base font-bold text-foreground tracking-tight group-hover:text-accent-gold transition-colors">
          {thought.title}
        </h4>

        <p className="text-sm text-white/80 leading-relaxed whitespace-pre-line">
          {thought.content}
        </p>
      </div>

      <div className="pt-4 mt-3 border-t border-white/5 flex items-center justify-between text-xs text-text-muted">
        <div className="flex items-center gap-1.5">
          <Calendar className="w-3.5 h-3.5" />
          <span>{formattedDate}</span>
        </div>
      </div>
    </div>
  );
}
