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
import { Form } from "@/components/ui/form";
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
        <Form.Field>
          <Form.Label>Select Verdict</Form.Label>
          <Form.SelectionGroup className="flex gap-2.5">
            {VERDICT_OPTIONS.map((opt) => {
              const Icon = opt.icon;
              const isSelected = verdict === opt.id;
              return (
                <button
                  key={opt.id}
                  type="button"
                  onClick={() => setVerdict(opt.id)}
                  className={cn(
                    "flex w-1/4 items-start gap-3 p-3 rounded-xl border text-left transition-all cursor-pointer",
                    isSelected
                      ? cn(opt.bgActive, opt.borderColor)
                      : "border-white/10 bg-surface/50 hover:bg-surface-raised/80 hover:border-white/15 text-text-muted",
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
                    <div className="text-xs text-text-muted mt-0.5">
                      {opt.description}
                    </div>
                  </div>
                </button>
              );
            })}
          </Form.SelectionGroup>
        </Form.Field>

        <Form.Field>
          <Form.Label>Review Summary (Optional)</Form.Label>
          <Form.Textarea
            value={summary}
            onChange={(e) => setSummary(e.target.value)}
            placeholder="What did you love or dislike about this game? Key highlights, story impressions, replayability..."
            rows={4}
          />
        </Form.Field>
      </div>

      <DialogFooter className="gap-2">
        <Button variant="ghost" onClick={onCancel} disabled={isSaving}>
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
      <DialogContent className="max-w-1/2">
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
