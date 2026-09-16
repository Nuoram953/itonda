import {
  useMutation,
  useQueryClient,
  type UseMutationOptions,
} from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";

export type UpsertReviewParams = {
  mediaId: string;
  payload: components["schemas"]["UpsertReviewPayload"];
};

export const upsertReview = ({
  mediaId,
  payload,
}: UpsertReviewParams): Promise<components["schemas"]["GameReview"]> => {
  return api.put(`/media/${mediaId}/review`, payload);
};

export type UseUpsertReviewOptions = {
  mediaId: string;
  mutationConfig?: UseMutationOptions<
    components["schemas"]["GameReview"],
    Error,
    components["schemas"]["UpsertReviewPayload"]
  >;
};

export const useUpsertReview = ({
  mediaId,
  mutationConfig,
}: UseUpsertReviewOptions) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...restConfig } = mutationConfig || {};

  return useMutation({
    ...restConfig,
    mutationFn: (payload: components["schemas"]["UpsertReviewPayload"]) =>
      upsertReview({ mediaId, payload }),
    onSuccess: (...args) => {
      queryClient.invalidateQueries({
        queryKey: ["media", mediaId, "review"],
      });
      onSuccess?.(...args);
    },
  });
};
