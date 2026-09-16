import { useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { cn } from "@/lib/utils";
import type { GameThought } from "../../types/reviews";
import { THOUGHT_CATEGORIES } from "../../constants/reviews";

type ThoughtDialogProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  initialThought?: GameThought | null;
  defaultPlaytimeMinutes?: number;
  onSave: (data: {
    title: string;
    content: string;
    category: string;
    playtime_minutes?: number;
  }) => Promise<void>;
};

function ThoughtForm({
  initialThought,
  defaultPlaytimeMinutes,
  onCancel,
  onSave,
}: {
  initialThought?: GameThought | null;
  defaultPlaytimeMinutes?: number;
  onCancel: () => void;
  onSave: (data: {
    title: string;
    content: string;
    category: string;
    playtime_minutes?: number;
  }) => Promise<void>;
}) {
  const [title, setTitle] = useState(initialThought?.title || "");
  const [content, setContent] = useState(initialThought?.content || "");
  const [category, setCategory] = useState(
    initialThought?.category || "audio",
  );
  const [playtimeHours, setPlaytimeHours] = useState(
    initialThought?.playtime_minutes != null
      ? (initialThought.playtime_minutes / 60).toFixed(1)
      : defaultPlaytimeMinutes != null && defaultPlaytimeMinutes > 0
        ? (defaultPlaytimeMinutes / 60).toFixed(1)
        : "",
  );
  const [isSaving, setIsSaving] = useState(false);

  const isEdit = Boolean(initialThought);

  const handleSave = async () => {
    if (!title.trim() || !content.trim()) return;

    try {
      setIsSaving(true);
      const parsedHours = parseFloat(playtimeHours);
      const playtime_minutes =
        !isNaN(parsedHours) && parsedHours >= 0
          ? Math.round(parsedHours * 60)
          : undefined;

      await onSave({
        title: title.trim(),
        content: content.trim(),
        category,
        playtime_minutes,
      });
      onCancel();
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <>
      <div className="space-y-4 py-2">
        <div className="space-y-1.5">
          <label className="text-xs font-semibold uppercase tracking-wider text-text-muted">
            Topic / Title
          </label>
          <Input
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            placeholder="e.g. Music during Act 2 combat, Boss fight phase 3..."
            className="bg-surface/50 border-input text-sm"
            autoFocus
          />
        </div>

        <div className="space-y-1.5">
          <label className="text-xs font-semibold uppercase tracking-wider text-text-muted">
            Category
          </label>
          <div className="flex flex-wrap gap-2">
            {THOUGHT_CATEGORIES.map((cat) => {
              const Icon = cat.icon;
              const isSelected = category === cat.id;
              return (
                <button
                  key={cat.id}
                  type="button"
                  onClick={() => setCategory(cat.id)}
                  className={cn(
                    "inline-flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-medium border transition-all cursor-pointer",
                    isSelected
                      ? cn(cat.badgeStyle, "ring-1 ring-white/20")
                      : "border-white/10 bg-surface/40 text-text-muted hover:text-white hover:bg-white/5",
                  )}
                >
                  <Icon className={cn("w-3.5 h-3.5", isSelected ? cat.color : "")} />
                  <span>{cat.label}</span>
                </button>
              );
            })}
          </div>
        </div>

        <div className="space-y-1.5">
          <label className="text-xs font-semibold uppercase tracking-wider text-text-muted">
            Playtime at Session (Hours)
          </label>
          <Input
            type="number"
            step="0.1"
            min="0"
            value={playtimeHours}
            onChange={(e) => setPlaytimeHours(e.target.value)}
            placeholder="e.g. 14.5"
            className="bg-surface/50 border-input text-sm w-36"
          />
        </div>

        <div className="space-y-1.5">
          <label className="text-xs font-semibold uppercase tracking-wider text-text-muted">
            Thought / Notes
          </label>
          <textarea
            value={content}
            onChange={(e) => setContent(e.target.value)}
            placeholder="What surprised, challenged, or delighted you in this session? (e.g. The music really surprised me with its dramatic choral swells during the bridge battle...)"
            rows={4}
            className="w-full rounded-xl border border-input bg-surface/50 px-3.5 py-2.5 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-accent-gold/50 transition-all resize-none"
          />
        </div>
      </div>

      <DialogFooter className="gap-2">
        <Button
          variant="ghost"
          onClick={onCancel}
          disabled={isSaving}
        >
          Cancel
        </Button>
        <Button
          onClick={handleSave}
          disabled={isSaving || !title.trim() || !content.trim()}
          className="bg-accent-gold text-background hover:bg-accent-gold/90 font-semibold cursor-pointer"
        >
          {isSaving ? "Saving..." : isEdit ? "Update Thought" : "Save Thought"}
        </Button>
      </DialogFooter>
    </>
  );
}

export function ThoughtDialog({
  open,
  onOpenChange,
  initialThought,
  defaultPlaytimeMinutes,
  onSave,
}: ThoughtDialogProps) {
  const isEdit = Boolean(initialThought);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-lg">
        <DialogHeader>
          <DialogTitle>
            {isEdit ? "Edit Session Thought" : "Add Session Thought"}
          </DialogTitle>
          <DialogDescription>
            Jot down a quick thought, highlight, or observation from your gaming session.
          </DialogDescription>
        </DialogHeader>

        {open && (
          <ThoughtForm
            initialThought={initialThought}
            defaultPlaytimeMinutes={defaultPlaytimeMinutes}
            onCancel={() => onOpenChange(false)}
            onSave={onSave}
          />
        )}
      </DialogContent>
    </Dialog>
  );
}
