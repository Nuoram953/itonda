import { useQuery, queryOptions } from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";
import { type QueryConfig } from "@/lib/react-query";

export const getMedia = (type?: string): Promise<components["schemas"]["MediaResponse"]> => {
  return api.get(`/media`, {
    params: type ? { type } : undefined,
  });
};

export const getMediaQueryOptions = (type?: string) => {
  return queryOptions({
    queryKey: ["media", { type }],
    queryFn: () => getMedia(type),
  });
};

type UseMediaOptions = {
  type?: string;
  queryConfig?: QueryConfig<typeof getMediaQueryOptions>;
};

export const useMedia = ({ type, queryConfig }: UseMediaOptions = {}) => {
  return useQuery({
    ...getMediaQueryOptions(type),
    ...queryConfig,
  });
};
