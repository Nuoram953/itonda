import type { components } from "@/api/generated.d";
import { LoadingState } from "@/components/feedback/LoadingState";
import { ErrorState } from "@/components/feedback/ErrorState";
import { useReviewOverview } from "../../api/get-review-overview";
import { GameVerdictCard } from "../reviews/GameVerdictCard";
import { SessionThoughtsSection } from "../reviews/SessionThoughtsSection";

type ReviewsTabProps = {
  media: components["schemas"]["Media"];
};

export function ReviewsTab({ media }: ReviewsTabProps) {
  const { data: overview, isPending, isError } = useReviewOverview({
    mediaId: media.id,
  });

  if (isPending) {
    return <LoadingState message="Loading game reviews and session notes..." />;
  }

  if (isError) {
    return (
      <ErrorState
        title="Unable to load reviews"
        message="An error occurred while loading reviews and session notes. Please try again."
      />
    );
  }

  return (
    <div className="space-y-8 animate-in fade-in duration-300 pb-16">
      <div className="border-b border-white/10 pb-4">
        <h2 className="text-2xl font-extrabold text-foreground tracking-tight">
          Reviews & Session Notes
        </h2>
        <p className="text-sm text-text-muted mt-1">
          Your overall recommendation score and session highlights for {media.title}.
        </p>
      </div>

      <GameVerdictCard
        mediaId={media.id}
        review={overview?.review ?? null}
      />

      <SessionThoughtsSection
        mediaId={media.id}
        thoughts={overview?.thoughts ?? []}
        defaultPlaytimeMinutes={media.details?.playtime_minutes ?? undefined}
      />
    </div>
  );
}
