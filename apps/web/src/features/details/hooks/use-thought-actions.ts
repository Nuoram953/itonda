import { useCreateThought } from "../api/post-thought";
import { useUpdateThought } from "../api/put-thought";
import { useDeleteThought } from "../api/delete-thought";

type UseThoughtActionsOptions = {
  mediaId: string;
};

export function useThoughtActions({ mediaId }: UseThoughtActionsOptions) {
  const createThoughtMutation = useCreateThought({ mediaId });
  const updateThoughtMutation = useUpdateThought({ mediaId });
  const deleteThoughtMutation = useDeleteThought({ mediaId });

  const createThought = async (data: {
    title: string;
    content: string;
    category?: string;
    playtime_minutes?: number;
  }) => {
    await createThoughtMutation.mutateAsync(data);
  };

  const updateThought = async (
    thoughtId: string,
    data: {
      title: string;
      content: string;
      category?: string;
    },
  ) => {
    await updateThoughtMutation.mutateAsync({
      thoughtId,
      payload: data,
    });
  };

  const deleteThought = async (thoughtId: string) => {
    await deleteThoughtMutation.mutateAsync(thoughtId);
  };

  return {
    createThought,
    updateThought,
    deleteThought,
    isCreating: createThoughtMutation.isPending,
    isUpdating: updateThoughtMutation.isPending,
    isDeleting: deleteThoughtMutation.isPending,
  };
}
