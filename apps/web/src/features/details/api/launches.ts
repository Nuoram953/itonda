import { useMutation, useQueryClient } from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";
import { type MutationConfig } from "@/lib/react-query";

export type Launch = components["schemas"]["Launch"];
export type CreateMediaLaunchPayload =
  components["schemas"]["CreateMediaLaunchPayload"];
export type UpdateMediaLaunchPayload =
  components["schemas"]["UpdateMediaLaunchPayload"];

export const createLaunchProfile = ({
  mediaId,
  data,
}: {
  mediaId: string;
  data: CreateMediaLaunchPayload;
}): Promise<Launch> => {
  return api.post(`/media/${mediaId}/launches`, data);
};

export const useCreateLaunchProfile = (
  mutationConfig?: MutationConfig<typeof createLaunchProfile>,
) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...rest } = mutationConfig || {};

  return useMutation({
    onSuccess: (data, variables, ...restArgs) => {
      queryClient.invalidateQueries({
        queryKey: ["media", variables.mediaId],
      });
      onSuccess?.(data, variables, ...restArgs);
    },
    ...rest,
    mutationFn: createLaunchProfile,
  });
};

export const updateLaunchProfile = ({
  mediaId,
  launchId,
  data,
}: {
  mediaId: string;
  launchId: string;
  data: UpdateMediaLaunchPayload;
}): Promise<Launch> => {
  return api.patch(`/media/${mediaId}/launches/${launchId}`, data);
};

export const useUpdateLaunchProfile = (
  mutationConfig?: MutationConfig<typeof updateLaunchProfile>,
) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...rest } = mutationConfig || {};

  return useMutation({
    onSuccess: (data, variables, ...restArgs) => {
      queryClient.invalidateQueries({
        queryKey: ["media", variables.mediaId],
      });
      onSuccess?.(data, variables, ...restArgs);
    },
    ...rest,
    mutationFn: updateLaunchProfile,
  });
};

export const deleteLaunchProfile = ({
  mediaId,
  launchId,
}: {
  mediaId: string;
  launchId: string;
}): Promise<void> => {
  return api.delete(`/media/${mediaId}/launches/${launchId}`);
};

export const useDeleteLaunchProfile = (
  mutationConfig?: MutationConfig<typeof deleteLaunchProfile>,
) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...rest } = mutationConfig || {};

  return useMutation({
    onSuccess: (data, variables, ...restArgs) => {
      queryClient.invalidateQueries({
        queryKey: ["media", variables.mediaId],
      });
      onSuccess?.(data, variables, ...restArgs);
    },
    ...rest,
    mutationFn: deleteLaunchProfile,
  });
};
