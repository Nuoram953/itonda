import {
  queryOptions,
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";
import { type MutationConfig, type QueryConfig } from "@/lib/react-query";

export type Emulator = components["schemas"]["Emulator"];
export type GetEmulatorsResponse = components["schemas"]["GetEmulatorsResponse"];
export type CreateEmulatorPayload =
  components["schemas"]["CreateEmulatorPayload"];
export type UpdateEmulatorPayload =
  components["schemas"]["UpdateEmulatorPayload"];

export const getEmulators = (): Promise<GetEmulatorsResponse> => {
  return api.get("/emulators");
};

export const getEmulatorsQueryOptions = () => {
  return queryOptions({
    queryKey: ["emulators"],
    queryFn: () => getEmulators(),
  });
};

type UseEmulatorsOptions = {
  queryConfig?: QueryConfig<typeof getEmulatorsQueryOptions>;
};

export const useEmulators = ({ queryConfig }: UseEmulatorsOptions = {}) => {
  return useQuery({
    ...getEmulatorsQueryOptions(),
    ...queryConfig,
  });
};

export const createEmulator = (
  data: CreateEmulatorPayload,
): Promise<Emulator> => {
  return api.post("/emulators", data);
};

export const useCreateEmulator = (
  mutationConfig?: MutationConfig<typeof createEmulator>,
) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...rest } = mutationConfig || {};

  return useMutation({
    onSuccess: (...args) => {
      queryClient.invalidateQueries({ queryKey: ["emulators"] });
      onSuccess?.(...args);
    },
    ...rest,
    mutationFn: createEmulator,
  });
};

export const updateEmulator = ({
  id,
  data,
}: {
  id: string;
  data: UpdateEmulatorPayload;
}): Promise<Emulator> => {
  return api.patch(`/emulators/${id}`, data);
};

export const useUpdateEmulator = (
  mutationConfig?: MutationConfig<typeof updateEmulator>,
) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...rest } = mutationConfig || {};

  return useMutation({
    onSuccess: (...args) => {
      queryClient.invalidateQueries({ queryKey: ["emulators"] });
      onSuccess?.(...args);
    },
    ...rest,
    mutationFn: updateEmulator,
  });
};

export const deleteEmulator = (id: string): Promise<void> => {
  return api.delete(`/emulators/${id}`);
};

export const useDeleteEmulator = (
  mutationConfig?: MutationConfig<typeof deleteEmulator>,
) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...rest } = mutationConfig || {};

  return useMutation({
    onSuccess: (...args) => {
      queryClient.invalidateQueries({ queryKey: ["emulators"] });
      onSuccess?.(...args);
    },
    ...rest,
    mutationFn: deleteEmulator,
  });
};
