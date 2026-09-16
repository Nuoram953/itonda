import {
  useMutation,
  useQuery,
  useQueryClient,
  queryOptions,
  type UseMutationOptions,
} from "@tanstack/react-query";
import { api } from "@/lib/api-client";
import { type QueryConfig } from "@/lib/react-query";
import type {
  CreateThoughtPayload,
  GameReview,
  GameReviewOverview,
  GameThought,
  UpdateThoughtPayload,
  UpsertReviewPayload,
} from "../types/reviews";

export const getReviewOverview = (
  mediaId: string,
): Promise<GameReviewOverview> => {
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

export const upsertReview = ({
  mediaId,
  payload,
}: {
  mediaId: string;
  payload: UpsertReviewPayload;
}): Promise<GameReview> => {
  return api.put(`/media/${mediaId}/review`, payload);
};

export const useUpsertReview = ({
  mediaId,
  mutationConfig,
}: {
  mediaId: string;
  mutationConfig?: UseMutationOptions<GameReview, Error, UpsertReviewPayload>;
}) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...restConfig } = mutationConfig || {};

  return useMutation({
    ...restConfig,
    mutationFn: (payload: UpsertReviewPayload) =>
      upsertReview({ mediaId, payload }),
    onSuccess: (...args) => {
      queryClient.invalidateQueries({
        queryKey: ["media", mediaId, "review"],
      });
      onSuccess?.(...args);
    },
  });
};

export const deleteReview = ({
  mediaId,
}: {
  mediaId: string;
}): Promise<void> => {
  return api.delete(`/media/${mediaId}/review`);
};

export const useDeleteReview = ({
  mediaId,
  mutationConfig,
}: {
  mediaId: string;
  mutationConfig?: UseMutationOptions<void, Error, void>;
}) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...restConfig } = mutationConfig || {};

  return useMutation({
    ...restConfig,
    mutationFn: () => deleteReview({ mediaId }),
    onSuccess: (...args) => {
      queryClient.invalidateQueries({
        queryKey: ["media", mediaId, "review"],
      });
      onSuccess?.(...args);
    },
  });
};

export const createThought = ({
  mediaId,
  payload,
}: {
  mediaId: string;
  payload: CreateThoughtPayload;
}): Promise<GameThought> => {
  return api.post(`/media/${mediaId}/thoughts`, payload);
};

export const useCreateThought = ({
  mediaId,
  mutationConfig,
}: {
  mediaId: string;
  mutationConfig?: UseMutationOptions<GameThought, Error, CreateThoughtPayload>;
}) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...restConfig } = mutationConfig || {};

  return useMutation({
    ...restConfig,
    mutationFn: (payload: CreateThoughtPayload) =>
      createThought({ mediaId, payload }),
    onSuccess: (...args) => {
      queryClient.invalidateQueries({
        queryKey: ["media", mediaId, "review"],
      });
      onSuccess?.(...args);
    },
  });
};

export const updateThought = ({
  mediaId,
  thoughtId,
  payload,
}: {
  mediaId: string;
  thoughtId: string;
  payload: UpdateThoughtPayload;
}): Promise<GameThought> => {
  return api.put(`/media/${mediaId}/thoughts/${thoughtId}`, payload);
};

export const useUpdateThought = ({
  mediaId,
  mutationConfig,
}: {
  mediaId: string;
  mutationConfig?: UseMutationOptions<
    GameThought,
    Error,
    { thoughtId: string; payload: UpdateThoughtPayload }
  >;
}) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...restConfig } = mutationConfig || {};

  return useMutation({
    ...restConfig,
    mutationFn: ({
      thoughtId,
      payload,
    }: {
      thoughtId: string;
      payload: UpdateThoughtPayload;
    }) => updateThought({ mediaId, thoughtId, payload }),
    onSuccess: (...args) => {
      queryClient.invalidateQueries({
        queryKey: ["media", mediaId, "review"],
      });
      onSuccess?.(...args);
    },
  });
};

export const deleteThought = ({
  mediaId,
  thoughtId,
}: {
  mediaId: string;
  thoughtId: string;
}): Promise<void> => {
  return api.delete(`/media/${mediaId}/thoughts/${thoughtId}`);
};

export const useDeleteThought = ({
  mediaId,
  mutationConfig,
}: {
  mediaId: string;
  mutationConfig?: UseMutationOptions<void, Error, string>;
}) => {
  const queryClient = useQueryClient();
  const { onSuccess, ...restConfig } = mutationConfig || {};

  return useMutation({
    ...restConfig,
    mutationFn: (thoughtId: string) =>
      deleteThought({ mediaId, thoughtId }),
    onSuccess: (...args) => {
      queryClient.invalidateQueries({
        queryKey: ["media", mediaId, "review"],
      });
      onSuccess?.(...args);
    },
  });
};
