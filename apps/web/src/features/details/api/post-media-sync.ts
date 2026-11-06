import { useMutation, useQueryClient } from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";
import { type MutationConfig } from "@/lib/react-query";

export const syncMedia = (
  mediaId: string,
): Promise<components["schemas"]["JobResponse"]> => {
  return api.post(`/media/${mediaId}/sync`);
};

type UseSyncMediaOptions = {
  mutationConfig?: MutationConfig<typeof syncMedia>;
};

export const useSyncMedia = ({ mutationConfig }: UseSyncMediaOptions = {}) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...restConfig } = mutationConfig || {};

  return useMutation({
    onSuccess: (...args) => {
      queryClient.invalidateQueries({ queryKey: ["media"] });
      onSuccess?.(...args);
    },
    ...restConfig,
    mutationFn: syncMedia,
  });
};
