import { useQuery, queryOptions } from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";
import { type QueryConfig } from "@/lib/react-query";

export const getConfig = (): Promise<
  components["schemas"]["CombinedConfig"]
> => {
  return api.get(`/config`);
};

export const getConfigQueryOptions = () => {
  return queryOptions({
    queryKey: ["config"],
    queryFn: () => getConfig(),
  });
};

type UseConfigOptions = {
  queryConfig?: QueryConfig<typeof getConfigQueryOptions>;
};

export const useConfig = ({ queryConfig }: UseConfigOptions = {}) => {
  return useQuery({
    ...getConfigQueryOptions(),
    ...queryConfig,
  });
};
