import { useQuery, queryOptions } from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";
import { type QueryConfig } from "@/lib/react-query";

export const getMediaById = (
  mediaId: string,
): Promise<components["schemas"]["Media"]> => {
  return api.get(`/media/${mediaId}`);
};

export const getMediaByIdQueryOptions = (mediaId: string) => {
  return queryOptions({
    queryKey: ["media", mediaId],
    queryFn: () => getMediaById(mediaId),
  });
};

type UseMediaByIdOptions = {
  mediaId: string;
  queryConfig?: QueryConfig<typeof getMediaByIdQueryOptions>;
};

export const useMediaById = ({ mediaId, queryConfig }: UseMediaByIdOptions) => {
  return useQuery({
    ...getMediaByIdQueryOptions(mediaId),
    ...queryConfig,
  });
};
