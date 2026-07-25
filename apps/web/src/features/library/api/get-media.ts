import { useQuery, queryOptions } from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";
import { type QueryConfig } from "@/lib/react-query";

export const getMedia = (): Promise<components["schemas"]["MediaResponse"]> => {
  return api.get(`/media`);
};

export const getMediaQueryOptions = () => {
  return queryOptions({
    queryKey: ["media"],
    queryFn: () => getMedia(),
  });
};

type UseMediaOptions = {
  queryConfig?: QueryConfig<typeof getMediaQueryOptions>;
};

export const useMedia = ({ queryConfig }: UseMediaOptions) => {
  return useQuery({
    ...getMediaQueryOptions(),
    ...queryConfig,
  });
};
