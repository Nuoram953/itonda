import { useQuery, queryOptions } from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";
import { type QueryConfig } from "@/lib/react-query";

export const getAgents = (): Promise<
  components["schemas"]["GetAgentsResponse"]
> => {
  return api.get(`/agents`);
};

export const getAgentsQueryOptions = () => {
  return queryOptions({
    queryKey: ["agents"],
    queryFn: () => getAgents(),
  });
};

type UseAgentsOptions = {
  queryConfig?: QueryConfig<typeof getAgentsQueryOptions>;
};

export const useAgents = ({ queryConfig }: UseAgentsOptions) => {
  return useQuery({
    ...getAgentsQueryOptions(),
    ...queryConfig,
  });
};
