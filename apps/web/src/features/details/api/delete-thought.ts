import {
  useMutation,
  useQueryClient,
  type UseMutationOptions,
} from "@tanstack/react-query";

import { api } from "@/lib/api-client";

export type DeleteThoughtParams = {
  mediaId: string;
  thoughtId: string;
};

export const deleteThought = ({
  mediaId,
  thoughtId,
}: DeleteThoughtParams): Promise<void> => {
  return api.delete(`/media/${mediaId}/thoughts/${thoughtId}`);
};

export type UseDeleteThoughtOptions = {
  mediaId: string;
  mutationConfig?: UseMutationOptions<void, Error, string>;
};

export const useDeleteThought = ({
  mediaId,
  mutationConfig,
}: UseDeleteThoughtOptions) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...restConfig } = mutationConfig || {};

  return useMutation({
    ...restConfig,
    mutationFn: (thoughtId: string) =>
      deleteThought({ mediaId, thoughtId }),
    onSuccess: (...args) => {
      queryClient.invalidateQueries({
        queryKey: ["media", mediaId, "review"],
      });
      onSuccess?.(...args);
    },
  });
};
