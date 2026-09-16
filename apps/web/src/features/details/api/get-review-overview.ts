import {
  queryOptions,
  useQuery,
} from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";
import { type QueryConfig } from "@/lib/react-query";

export const getReviewOverview = (
  mediaId: string,
): Promise<components["schemas"]["GameReviewOverview"]> => {
  return api.get(`/media/${mediaId}/review`);
};

export const getReviewOverviewQueryOptions = (mediaId: string) => {
  return queryOptions({
    queryKey: ["media", mediaId, "review"],
    queryFn: () => getReviewOverview(mediaId),
  });
};

type UseReviewOverviewOptions = {
  mediaId: string;
  queryConfig?: QueryConfig<typeof getReviewOverviewQueryOptions>;
};

export const useReviewOverview = ({
  mediaId,
  queryConfig,
}: UseReviewOverviewOptions) => {
  return useQuery({
    ...getReviewOverviewQueryOptions(mediaId),
    ...queryConfig,
  });
};
