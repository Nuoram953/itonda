import { useState } from "react";
import { Sparkles, ThumbsUp, Minus, ThumbsDown, Edit3, Trash2, Award } from "lucide-react";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import type { GameReview, ReviewVerdict } from "../../types/reviews";
import { useVerdictActions } from "../../hooks/use-verdict-actions";
import { VerdictDialog } from "./VerdictDialog";
import { ConfirmDialog } from "@/components/feedback/ConfirmDialog";
import { formatDate } from "@/utils/datetime";

type GameVerdictCardProps = {
  mediaId: string;
  review: GameReview | null;
  onSaveVerdict?: (data: { verdict: ReviewVerdict; summary: string }) => Promise<void>;
  onDeleteVerdict?: () => Promise<void>;
};

const VERDICT_CONFIG: Record<
  ReviewVerdict,
  {
    label: string;
    icon: typeof Sparkles;
    badgeStyle: string;
    iconStyle: string;
    headline: string;
  }
> = {
  masterpiece: {
    label: "Masterpiece",
    icon: Sparkles,
    badgeStyle:
      "bg-surface-raised/90 text-accent-gold border border-accent-gold/40 shadow-xs",
    iconStyle: "text-accent-gold",
    headline: "Exceptional Experience",
  },
  recommended: {
    label: "Recommended",
    icon: ThumbsUp,
    badgeStyle:
      "bg-surface-raised/90 text-emerald-300 border border-emerald-500/30 shadow-xs",
    iconStyle: "text-emerald-400",
    headline: "Worth Playing",
  },
  neutral: {
    label: "Neutral",
    icon: Minus,
    badgeStyle:
      "bg-surface-raised/90 text-amber-200/90 border border-amber-400/30 shadow-xs",
    iconStyle: "text-amber-300",
    headline: "Mixed Experience",
  },
  do_not_recommend: {
    label: "Do Not Recommend",
    icon: ThumbsDown,
    badgeStyle:
      "bg-surface-raised/90 text-rose-300 border border-rose-500/30 shadow-xs",
    iconStyle: "text-rose-400",
    headline: "Not Recommended",
  },
};

export function GameVerdictCard({
  mediaId,
  review,
  onSaveVerdict,
  onDeleteVerdict,
}: GameVerdictCardProps) {
  const [dialogOpen, setDialogOpen] = useState(false);
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
  const [isDeletingLocal, setIsDeletingLocal] = useState(false);
  const actions = useVerdictActions({ mediaId });

  const saveVerdict = onSaveVerdict ?? actions.saveVerdict;
  const deleteVerdict = onDeleteVerdict ?? actions.deleteVerdict;
  const isDeleting = isDeletingLocal || actions.isDeleting;

  const handleDelete = async () => {
    try {
      setIsDeletingLocal(true);
      await deleteVerdict();
    } finally {
      setIsDeletingLocal(false);
    }
  };

  if (!review) {
    return (
      <>
        <div className="relative overflow-hidden rounded-2xl bg-surface/70 border border-white/10 p-6 sm:p-8">
          <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-5">
            <div className="flex items-start gap-4">
              <div className="p-3 rounded-2xl bg-surface-raised/80 border border-white/10 text-text-muted shrink-0">
                <Award className="w-6 h-6" />
              </div>
              <div className="space-y-1">
                <h3 className="text-lg font-bold text-foreground">
                  Overall Game Verdict & Score
                </h3>
                <p className="text-sm text-text-muted max-w-xl">
                  You haven't added a review verdict yet. Rate this game as a Masterpiece, Recommended, Neutral, or Do Not Recommend, and optionally attach a review summary.
                </p>
              </div>
            </div>

            <Button
              onClick={() => setDialogOpen(true)}
              className="shrink-0 cursor-pointer font-semibold shadow-sm"
            >
              + Add Verdict & Review
            </Button>
          </div>
        </div>

        <VerdictDialog
          open={dialogOpen}
          onOpenChange={setDialogOpen}
          onSave={saveVerdict}
        />
      </>
    );
  }

  const config = VERDICT_CONFIG[review.verdict];
  const Icon = config.icon;

  const formattedDate = formatDate(review.updated_at);

  return (
    <>
      <div className="relative overflow-hidden rounded-2xl bg-surface/70 border border-white/10 p-6 sm:p-8 space-y-5">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
          <div className="flex items-center gap-3.5">
            <div
              className={cn(
                "inline-flex items-center gap-2 px-4 py-1.5 rounded-full text-sm font-bold tracking-wide uppercase shadow-sm",
                config.badgeStyle,
              )}
            >
              <Icon className={cn("w-4 h-4", config.iconStyle)} />
              <span>{config.label}</span>
            </div>

            <span className="text-xs text-text-muted">
              Updated {formattedDate}
            </span>
          </div>

          <div className="flex items-center gap-2">
            <Button
              variant="outline"
              size="sm"
              onClick={() => setDialogOpen(true)}
              className="h-8 gap-1.5 text-xs text-text-muted hover:text-white border-white/10 cursor-pointer"
            >
              <Edit3 className="w-3.5 h-3.5" />
              <span>Edit</span>
            </Button>
            <Button
              variant="ghost"
              size="sm"
              onClick={() => setDeleteDialogOpen(true)}
              disabled={isDeleting}
              className="h-8 gap-1.5 text-xs text-rose-400 hover:text-rose-300 hover:bg-rose-500/10 cursor-pointer"
            >
              <Trash2 className="w-3.5 h-3.5" />
              <span>Remove</span>
            </Button>
          </div>
        </div>

        {review.summary && (
          <div className="rounded-xl bg-background/50 border border-white/5 p-4 sm:p-5">
            <p className="text-sm sm:text-base text-white/90 leading-relaxed italic whitespace-pre-line">
              "{review.summary}"
            </p>
          </div>
        )}
      </div>

      <VerdictDialog
        open={dialogOpen}
        onOpenChange={setDialogOpen}
        currentVerdict={review.verdict}
        currentSummary={review.summary}
        onSave={saveVerdict}
      />

      <ConfirmDialog
        open={deleteDialogOpen}
        onOpenChange={setDeleteDialogOpen}
        title="Remove Verdict & Review"
        description="Are you sure you want to remove your game review and verdict? This action cannot be undone."
        confirmLabel="Remove Review"
        variant="destructive"
        isLoading={isDeleting}
        onConfirm={handleDelete}
      />
    </>
  );
}
