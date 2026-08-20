import { useMutation, useQueryClient } from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";
import { type MutationConfig } from "@/lib/react-query";

export type PatchConfigData = Partial<
  components["schemas"]["PatchConfigPayload"]
>;
export type CombinedConfigData = components["schemas"]["CombinedConfig"];

export const patchConfig = (
  data: PatchConfigData,
): Promise<CombinedConfigData> => {
  return api.patch(`/config`, data);
};

type UsePatchConfigOptions = {
  mutationConfig?: MutationConfig<typeof patchConfig>;
};

export const usePatchConfig = ({
  mutationConfig,
}: UsePatchConfigOptions = {}) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...restConfig } = mutationConfig || {};

  return useMutation({
    onSuccess: (...args) => {
      queryClient.invalidateQueries({ queryKey: ["config"] });
      onSuccess?.(...args);
    },
    ...restConfig,
    mutationFn: patchConfig,
  });
};
