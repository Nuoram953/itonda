import {
  useQuery,
  useInfiniteQuery,
  queryOptions,
  infiniteQueryOptions,
} from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";
import { type QueryConfig } from "@/lib/react-query";

export type GetMediaParams = {
  type?: components["schemas"]["MediaType"] | string;
  search?: string;
  status?: components["schemas"]["MediaStatus"] | string;
  storefront?: string;
  sort_by?: components["schemas"]["MediaSortField"];
  sort_order?: components["schemas"]["SortOrder"];
  page?: number;
  limit?: number;
};

export const getMedia = (
  params?: GetMediaParams,
): Promise<components["schemas"]["MediaResponse"]> => {
  return api.get(`/media`, {
    params,
  });
};

export const getMediaQueryOptions = (params?: GetMediaParams) => {
  return queryOptions({
    queryKey: ["media", params],
    queryFn: () => getMedia(params),
  });
};

export const getInfiniteMediaQueryOptions = (
  params: Omit<GetMediaParams, "page"> = {},
) => {
  return infiniteQueryOptions({
    queryKey: ["media", "infinite", params],
    queryFn: ({ pageParam = 1 }) =>
      getMedia({ ...params, page: pageParam as number }),
    initialPageParam: 1,
    getNextPageParam: (lastPage) =>
      lastPage.has_next ? lastPage.page + 1 : undefined,
  });
};

type UseMediaOptions = {
  type?: string;
  queryConfig?: QueryConfig<typeof getMediaQueryOptions>;
};

export const useMedia = ({ type, queryConfig }: UseMediaOptions = {}) => {
  return useQuery({
    ...getMediaQueryOptions({ type }),
    ...queryConfig,
  });
};

export const useInfiniteMedia = (
  params: Omit<GetMediaParams, "page"> = {},
) => {
  return useInfiniteQuery(getInfiniteMediaQueryOptions(params));
};
