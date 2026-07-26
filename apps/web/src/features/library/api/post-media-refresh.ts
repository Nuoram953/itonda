import { useMutation } from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";
import { type MutationConfig } from "@/lib/react-query";

type RefreshMediaInput = components["schemas"]["MediaRefreshPayload"];

export const refreshMedia = (
  data: RefreshMediaInput,
): Promise<components["schemas"]["MediaRefreshPayload"]> => {
  return api.post("/media/refresh", data);
};

type UseRefreshMediaOptions = {
  mutationConfig?: MutationConfig<typeof refreshMedia>;
};

export const useRefreshMedia = ({ mutationConfig }: UseRefreshMediaOptions) => {
  const { onSuccess, ...restConfig } = mutationConfig || {};

  return useMutation({
    onSuccess: (...args) => {
      onSuccess?.(...args);
    },
    ...restConfig,
    mutationFn: refreshMedia,
  });
};
