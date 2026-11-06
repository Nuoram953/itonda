import { useMutation, useQueryClient } from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";
import { type MutationConfig } from "@/lib/react-query";

type RefreshMediaInput = components["schemas"]["MediaSyncPayload"];

export const refreshMedia = (
  data?: RefreshMediaInput,
): Promise<components["schemas"]["JobResponse"]> => {
  return api.post("/media/sync", data ?? {});
};

type UseRefreshMediaOptions = {
  mutationConfig?: MutationConfig<typeof refreshMedia>;
};

export const useRefreshMedia = ({ mutationConfig }: UseRefreshMediaOptions = {}) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...restConfig } = mutationConfig || {};

  return useMutation({
    onSuccess: (...args) => {
      queryClient.invalidateQueries({ queryKey: ["media"] });
      onSuccess?.(...args);
    },
    ...restConfig,
    mutationFn: refreshMedia,
  });
};
