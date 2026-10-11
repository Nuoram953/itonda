import {
  useQuery,
  useInfiniteQuery,
  queryOptions,
  infiniteQueryOptions,
} from "@tanstack/react-query";

import type { components } from "@/api/generated.d";
import { api } from "@/lib/api-client";
import { type QueryConfig } from "@/lib/react-query";

export type GetMediaParams = components["schemas"]["MediaQueryParams"] & {
  type?: components["schemas"]["MediaType"] | string | null;
  status?: components["schemas"]["MediaStatus"] | string | null;
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

export type UseMediaOptions = {
  params?: GetMediaParams;
  queryConfig?: QueryConfig<typeof getMediaQueryOptions>;
} & GetMediaParams;

export const useMedia = (options: UseMediaOptions = {}) => {
  const { queryConfig, params, ...directParams } = options;
  const hasDirectParams = Object.keys(directParams).length > 0;
  const resolvedParams: GetMediaParams | undefined =
    params ?? (hasDirectParams ? (directParams as GetMediaParams) : undefined);

  return useQuery({
    ...getMediaQueryOptions(resolvedParams),
    ...queryConfig,
  });
};

export const useInfiniteMedia = (params: Omit<GetMediaParams, "page"> = {}) => {
  return useInfiniteQuery(getInfiniteMediaQueryOptions(params));
};
