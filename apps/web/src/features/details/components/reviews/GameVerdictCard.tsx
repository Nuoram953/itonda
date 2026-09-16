import { useState } from "react";
import { Sparkles, ThumbsUp, Minus, ThumbsDown, Edit3, Trash2, Award } from "lucide-react";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import type { GameReview, ReviewVerdict } from "../../types/reviews";
import { VerdictDialog } from "./VerdictDialog";

type GameVerdictCardProps = {
  review: GameReview | null;
  onSaveVerdict: (data: { verdict: ReviewVerdict; summary: string }) => Promise<void>;
  onDeleteVerdict: () => Promise<void>;
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
      "bg-amber-500/20 text-amber-300 border border-amber-400/40 shadow-sm shadow-amber-500/10",
    iconStyle: "text-amber-300",
    headline: "Exceptional Experience",
  },
  recommended: {
    label: "Recommended",
    icon: ThumbsUp,
    badgeStyle:
      "bg-emerald-500/20 text-emerald-300 border border-emerald-400/40 shadow-sm shadow-emerald-500/10",
    iconStyle: "text-emerald-300",
    headline: "Worth Playing",
  },
  neutral: {
    label: "Neutral",
    icon: Minus,
    badgeStyle:
      "bg-amber-400/15 text-amber-200 border border-amber-400/30 shadow-sm shadow-amber-400/10",
    iconStyle: "text-amber-200",
    headline: "Mixed Experience",
  },
  do_not_recommend: {
    label: "Do Not Recommend",
    icon: ThumbsDown,
    badgeStyle:
      "bg-rose-500/20 text-rose-300 border border-rose-400/40 shadow-sm shadow-rose-500/10",
    iconStyle: "text-rose-300",
    headline: "Not Recommended",
  },
};

export function GameVerdictCard({
  review,
  onSaveVerdict,
  onDeleteVerdict,
}: GameVerdictCardProps) {
  const [dialogOpen, setDialogOpen] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);

  const handleDelete = async () => {
    if (!window.confirm("Are you sure you want to remove your game review and verdict?")) {
      return;
    }
    try {
      setIsDeleting(true);
      await onDeleteVerdict();
    } finally {
      setIsDeleting(false);
    }
  };

  if (!review) {
    return (
      <>
        <div className="relative overflow-hidden rounded-2xl bg-surface/70 border border-white/10 p-6 sm:p-8">
          <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-5">
            <div className="flex items-start gap-4">
              <div className="p-3 rounded-2xl bg-accent-gold/10 border border-accent-gold/20 text-accent-gold shrink-0">
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
              className="bg-accent-gold text-background hover:bg-accent-gold/90 font-semibold shrink-0 cursor-pointer shadow-md"
            >
              + Add Verdict & Review
            </Button>
          </div>
        </div>

        <VerdictDialog
          open={dialogOpen}
          onOpenChange={setDialogOpen}
          onSave={onSaveVerdict}
        />
      </>
    );
  }

  const config = VERDICT_CONFIG[review.verdict];
  const Icon = config.icon;

  const formattedDate = new Date(review.updated_at).toLocaleDateString(undefined, {
    year: "numeric",
    month: "short",
    day: "numeric",
  });

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
              onClick={handleDelete}
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
        onSave={onSaveVerdict}
      />
    </>
  );
}
