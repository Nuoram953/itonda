import type { components } from "@/api/generated.d";
import { LoadingState } from "@/components/feedback/LoadingState";
import { ErrorState } from "@/components/feedback/ErrorState";
import {
  useCreateThought,
  useDeleteReview,
  useDeleteThought,
  useReviewOverview,
  useUpdateThought,
  useUpsertReview,
} from "../../api/reviews";
import { GameVerdictCard } from "../reviews/GameVerdictCard";
import { SessionThoughtsSection } from "../reviews/SessionThoughtsSection";
import type { ReviewVerdict } from "../../types/reviews";

type ReviewsTabProps = {
  media: components["schemas"]["Media"];
};

export function ReviewsTab({ media }: ReviewsTabProps) {
  const { data: overview, isPending, isError } = useReviewOverview({
    mediaId: media.id,
  });

  const upsertReviewMutation = useUpsertReview({ mediaId: media.id });
  const deleteReviewMutation = useDeleteReview({ mediaId: media.id });
  const createThoughtMutation = useCreateThought({ mediaId: media.id });
  const updateThoughtMutation = useUpdateThought({ mediaId: media.id });
  const deleteThoughtMutation = useDeleteThought({ mediaId: media.id });

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

  const handleSaveVerdict = async (data: {
    verdict: ReviewVerdict;
    summary: string;
  }) => {
    await upsertReviewMutation.mutateAsync({
      verdict: data.verdict,
      summary: data.summary,
    });
  };

  const handleDeleteVerdict = async () => {
    await deleteReviewMutation.mutateAsync();
  };

  const handleCreateThought = async (data: {
    title: string;
    content: string;
    category: string;
    playtime_minutes?: number;
  }) => {
    await createThoughtMutation.mutateAsync(data);
  };

  const handleUpdateThought = async (
    thoughtId: string,
    data: {
      title: string;
      content: string;
      category: string;
    },
  ) => {
    await updateThoughtMutation.mutateAsync({
      thoughtId,
      payload: data,
    });
  };

  const handleDeleteThought = async (thoughtId: string) => {
    await deleteThoughtMutation.mutateAsync(thoughtId);
  };

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
        review={overview?.review ?? null}
        onSaveVerdict={handleSaveVerdict}
        onDeleteVerdict={handleDeleteVerdict}
      />

      <SessionThoughtsSection
        thoughts={overview?.thoughts ?? []}
        defaultPlaytimeMinutes={media.details?.playtime_minutes ?? undefined}
        onCreateThought={handleCreateThought}
        onUpdateThought={handleUpdateThought}
        onDeleteThought={handleDeleteThought}
      />
    </div>
  );
}
