import { useMutation } from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";
import { type MutationConfig } from "@/lib/react-query";

export const launchMedia = (
  launchId: string,
): Promise<components["schemas"]["CommandResponse"]> => {
  return api.post(`/media/launch/${launchId}`);
};

type UseLaunchMediaOptions = {
  mutationConfig?: MutationConfig<typeof launchMedia>;
};

export const useLaunchMedia = ({ mutationConfig }: UseLaunchMediaOptions) => {
  const { onSuccess, ...restConfig } = mutationConfig || {};

  return useMutation({
    onSuccess: (...args) => {
      onSuccess?.(...args);
    },
    ...restConfig,
    mutationFn: launchMedia,
  });
};
