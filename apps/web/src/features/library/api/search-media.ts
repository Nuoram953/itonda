import { useQuery, queryOptions } from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";
import { type QueryConfig } from "@/lib/react-query";

export type SearchMediaParams = {
  query: string;
  type: components["schemas"]["MediaType"];
};

export const searchMedia = (
  params: SearchMediaParams,
): Promise<components["schemas"]["MediaSearchResult"][]> => {
  return api.get("/media/search", {
    params,
  });
};

export const searchMediaQueryOptions = (params: SearchMediaParams) => {
  return queryOptions({
    queryKey: ["media", "search", params],
    queryFn: () => searchMedia(params),
    enabled: Boolean(params.query.trim().length >= 2 && params.type),
  });
};

type UseSearchMediaOptions = {
  query: string;
  type: components["schemas"]["MediaType"];
  queryConfig?: QueryConfig<typeof searchMediaQueryOptions>;
};

export const useSearchMedia = ({
  query,
  type,
  queryConfig,
}: UseSearchMediaOptions) => {
  return useQuery({
    ...searchMediaQueryOptions({ query, type }),
    ...queryConfig,
  });
};
