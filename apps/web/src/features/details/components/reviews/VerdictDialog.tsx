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
import { cn } from "@/lib/utils";
import type { ReviewVerdict } from "../../types/reviews";
import { VERDICT_OPTIONS } from "../../constants/reviews";

type VerdictDialogProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  currentVerdict?: ReviewVerdict;
  currentSummary?: string | null;
  onSave: (data: { verdict: ReviewVerdict; summary: string }) => Promise<void>;
};

function VerdictForm({
  currentVerdict,
  currentSummary,
  onCancel,
  onSave,
}: {
  currentVerdict?: ReviewVerdict;
  currentSummary?: string | null;
  onCancel: () => void;
  onSave: (data: { verdict: ReviewVerdict; summary: string }) => Promise<void>;
}) {
  const [verdict, setVerdict] = useState<ReviewVerdict>(
    currentVerdict || "recommended",
  );
  const [summary, setSummary] = useState(currentSummary || "");
  const [isSaving, setIsSaving] = useState(false);

  const handleSave = async () => {
    try {
      setIsSaving(true);
      await onSave({ verdict, summary });
      onCancel();
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <>
      <div className="space-y-4 py-2">
        <div className="space-y-2">
          <label className="text-xs font-semibold uppercase tracking-wider text-text-muted">
            Select Verdict
          </label>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
            {VERDICT_OPTIONS.map((opt) => {
              const Icon = opt.icon;
              const isSelected = verdict === opt.id;
              return (
                <button
                  key={opt.id}
                  type="button"
                  onClick={() => setVerdict(opt.id)}
                  className={cn(
                    "flex items-start gap-3 p-3 rounded-xl border text-left transition-all cursor-pointer",
                    isSelected
                      ? cn(opt.bgActive, opt.borderColor)
                      : "border-white/10 bg-surface/50 hover:bg-white/5 text-text-muted",
                  )}
                >
                  <Icon
                    className={cn(
                      "w-5 h-5 shrink-0 mt-0.5 transition-colors",
                      isSelected ? opt.activeColor : "text-text-muted",
                    )}
                  />
                  <div className="min-w-0">
                    <div
                      className={cn(
                        "text-sm font-semibold",
                        isSelected ? "text-foreground" : "text-white/80",
                      )}
                    >
                      {opt.label}
                    </div>
                    <div className="text-xs text-text-muted line-clamp-2 mt-0.5">
                      {opt.description}
                    </div>
                  </div>
                </button>
              );
            })}
          </div>
        </div>

        <div className="space-y-2">
          <label className="text-xs font-semibold uppercase tracking-wider text-text-muted">
            Review Summary (Optional)
          </label>
          <textarea
            value={summary}
            onChange={(e) => setSummary(e.target.value)}
            placeholder="What did you love or dislike about this game? Key highlights, story impressions, replayability..."
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
          disabled={isSaving}
          className="bg-accent-gold text-background hover:bg-accent-gold/90 font-semibold cursor-pointer"
        >
          {isSaving ? "Saving..." : "Save Verdict"}
        </Button>
      </DialogFooter>
    </>
  );
}

export function VerdictDialog({
  open,
  onOpenChange,
  currentVerdict,
  currentSummary,
  onSave,
}: VerdictDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-lg">
        <DialogHeader>
          <DialogTitle>Game Verdict & Overall Review</DialogTitle>
          <DialogDescription>
            Choose your overall score and share your final thoughts on the game.
          </DialogDescription>
        </DialogHeader>

        {open && (
          <VerdictForm
            currentVerdict={currentVerdict}
            currentSummary={currentSummary}
            onCancel={() => onOpenChange(false)}
            onSave={onSave}
          />
        )}
      </DialogContent>
    </Dialog>
  );
}
