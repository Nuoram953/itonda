import { useUpsertReview } from "../api/put-review";
import { useDeleteReview } from "../api/delete-review";
import type { ReviewVerdict } from "../types/reviews";

type UseVerdictActionsOptions = {
  mediaId: string;
};

export function useVerdictActions({ mediaId }: UseVerdictActionsOptions) {
  const upsertReviewMutation = useUpsertReview({ mediaId });
  const deleteReviewMutation = useDeleteReview({ mediaId });

  const saveVerdict = async (data: {
    verdict: ReviewVerdict;
    summary: string;
  }) => {
    await upsertReviewMutation.mutateAsync({
      verdict: data.verdict,
      summary: data.summary,
    });
  };

  const deleteVerdict = async () => {
    await deleteReviewMutation.mutateAsync();
  };

  return {
    saveVerdict,
    deleteVerdict,
    isSaving: upsertReviewMutation.isPending,
    isDeleting: deleteReviewMutation.isPending,
  };
}
