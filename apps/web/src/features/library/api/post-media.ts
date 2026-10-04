import { useMutation, useQueryClient } from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";
import { type MutationConfig } from "@/lib/react-query";

export type CreateMediaInput = components["schemas"]["CreateMediaPayload"];

export const createMedia = (
  data: CreateMediaInput,
): Promise<components["schemas"]["Media"]> => {
  return api.post("/media", data);
};

type UseCreateMediaOptions = {
  mutationConfig?: MutationConfig<typeof createMedia>;
};

export const useCreateMedia = ({ mutationConfig }: UseCreateMediaOptions = {}) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...restConfig } = mutationConfig || {};

  return useMutation({
    onSuccess: (...args) => {
      queryClient.invalidateQueries({
        queryKey: ["media"],
      });
      onSuccess?.(...args);
    },
    ...restConfig,
    mutationFn: createMedia,
  });
};
