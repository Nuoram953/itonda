import {
  useMutation,
  useQueryClient,
  type UseMutationOptions,
} from "@tanstack/react-query";

import { api } from "@/lib/api-client";

export type DeleteReviewParams = {
  mediaId: string;
};

export const deleteReview = ({
  mediaId,
}: DeleteReviewParams): Promise<void> => {
  return api.delete(`/media/${mediaId}/review`);
};

export type UseDeleteReviewOptions = {
  mediaId: string;
  mutationConfig?: UseMutationOptions<void, Error, void>;
};

export const useDeleteReview = ({
  mediaId,
  mutationConfig,
}: UseDeleteReviewOptions) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...restConfig } = mutationConfig || {};

  return useMutation({
    ...restConfig,
    mutationFn: () => deleteReview({ mediaId }),
    onSuccess: (...args) => {
      queryClient.invalidateQueries({
        queryKey: ["media", mediaId, "review"],
      });
      onSuccess?.(...args);
    },
  });
};
