import { useMutation, useQueryClient } from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";
import { type MutationConfig } from "@/lib/react-query";

export type RefreshSingleMediaParams = {
  mediaId: string;
  force?: boolean;
};

export const refreshSingleMedia = ({
  mediaId,
  force = false,
}: RefreshSingleMediaParams): Promise<components["schemas"]["JobResponse"]> => {
  return api.post(`/media/refresh/${mediaId}`, { force });
};

type UseRefreshSingleMediaOptions = {
  mutationConfig?: MutationConfig<typeof refreshSingleMedia>;
};

export const useRefreshSingleMedia = ({
  mutationConfig,
}: UseRefreshSingleMediaOptions = {}) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...restConfig } = mutationConfig || {};

  return useMutation({
    onSuccess: (data, variables, context) => {
      queryClient.invalidateQueries({
        queryKey: ["media", variables.mediaId],
      });
      queryClient.invalidateQueries({
        queryKey: ["media"],
      });
      onSuccess?.(data, variables, context);
    },
    ...restConfig,
    mutationFn: refreshSingleMedia,
  });
};
