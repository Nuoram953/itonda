import {
  useMutation,
  useQueryClient,
  type UseMutationOptions,
} from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";

export type CreateThoughtParams = {
  mediaId: string;
  payload: components["schemas"]["CreateThoughtPayload"];
};

export const createThought = ({
  mediaId,
  payload,
}: CreateThoughtParams): Promise<components["schemas"]["GameThought"]> => {
  return api.post(`/media/${mediaId}/thoughts`, payload);
};

export type UseCreateThoughtOptions = {
  mediaId: string;
  mutationConfig?: UseMutationOptions<
    components["schemas"]["GameThought"],
    Error,
    components["schemas"]["CreateThoughtPayload"]
  >;
};

export const useCreateThought = ({
  mediaId,
  mutationConfig,
}: UseCreateThoughtOptions) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...restConfig } = mutationConfig || {};

  return useMutation({
    ...restConfig,
    mutationFn: (payload: components["schemas"]["CreateThoughtPayload"]) =>
      createThought({ mediaId, payload }),
    onSuccess: (...args) => {
      queryClient.invalidateQueries({
        queryKey: ["media", mediaId, "review"],
      });
      onSuccess?.(...args);
    },
  });
};
