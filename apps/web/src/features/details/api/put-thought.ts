import {
  useMutation,
  useQueryClient,
  type UseMutationOptions,
} from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";

export type UpdateThoughtParams = {
  mediaId: string;
  thoughtId: string;
  payload: components["schemas"]["UpdateThoughtPayload"];
};

export const updateThought = ({
  mediaId,
  thoughtId,
  payload,
}: UpdateThoughtParams): Promise<components["schemas"]["GameThought"]> => {
  return api.put(`/media/${mediaId}/thoughts/${thoughtId}`, payload);
};

export type UseUpdateThoughtOptions = {
  mediaId: string;
  mutationConfig?: UseMutationOptions<
    components["schemas"]["GameThought"],
    Error,
    { thoughtId: string; payload: components["schemas"]["UpdateThoughtPayload"] }
  >;
};

export const useUpdateThought = ({
  mediaId,
  mutationConfig,
}: UseUpdateThoughtOptions) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...restConfig } = mutationConfig || {};

  return useMutation({
    ...restConfig,
    mutationFn: ({
      thoughtId,
      payload,
    }: {
      thoughtId: string;
      payload: components["schemas"]["UpdateThoughtPayload"];
    }) => updateThought({ mediaId, thoughtId, payload }),
    onSuccess: (...args) => {
      queryClient.invalidateQueries({
        queryKey: ["media", mediaId, "review"],
      });
      onSuccess?.(...args);
    },
  });
};
