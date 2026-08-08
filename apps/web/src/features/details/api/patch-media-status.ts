import { useMutation, useQueryClient } from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";
import { type MutationConfig } from "@/lib/react-query";

export type PatchMediaStatusParams = {
  mediaId: string;
  statusId: components["schemas"]["MediaStatus"];
};

export const patchMediaStatus = ({
  mediaId,
  statusId,
}: PatchMediaStatusParams): Promise<void> => {
  return api.patch(`/media/${mediaId}/status/${statusId}`);
};

type UsePatchMediaStatusOptions = {
  mutationConfig?: MutationConfig<typeof patchMediaStatus>;
};

export const usePatchMediaStatus = ({
  mutationConfig,
}: UsePatchMediaStatusOptions = {}) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...restConfig } = mutationConfig || {};

  return useMutation({
    onSuccess: (...args) => {
      queryClient.invalidateQueries({ queryKey: ["media"] });
      onSuccess?.(...args);
    },
    ...restConfig,
    mutationFn: patchMediaStatus,
  });
};
