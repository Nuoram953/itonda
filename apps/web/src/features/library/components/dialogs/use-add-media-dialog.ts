import { useState } from "react";
import { useSearchMedia } from "../../api/search-media";
import { useCreateMedia } from "../../api/post-media";
import { useNotification } from "@/hooks/use-notification";
import { useDebounce } from "@/hooks/use-debounce";
import type { components } from "@/api/generated.d";

type MediaType = components["schemas"]["MediaType"];
type MediaSearchResult = components["schemas"]["MediaSearchResult"];

type UseAddMediaDialogOptions = {
  onSuccess: () => void;
};

export function useAddMediaDialog({ onSuccess }: UseAddMediaDialogOptions) {
  const [selectedType, setSelectedType] = useState<MediaType | null>(null);
  const [searchQuery, setSearchQuery] = useState("");
  const [inputValue, setInputValue] = useState("");
  const [selectedResult, setSelectedResult] =
    useState<MediaSearchResult | null>(null);

  const debouncedQuery = useDebounce(searchQuery, 400);
  const { notify } = useNotification();
  const createMediaMutation = useCreateMedia();

  // Determine if a search is actively debouncing
  const isDebouncing =
    searchQuery.trim().length >= 2 &&
    searchQuery.trim() !== debouncedQuery.trim();

  const shouldSearch = Boolean(
    selectedType && debouncedQuery.trim().length >= 2,
  );

  const { data: searchResults, isFetching: isSearching } = useSearchMedia({
    query: debouncedQuery,
    type: selectedType || "game",
    queryConfig: {
      enabled: shouldSearch,
    },
  });

  const isLoading =
    (isSearching && shouldSearch) ||
    (isDebouncing && Boolean(selectedType && searchQuery.trim().length >= 2));

  const handleSelectType = (type: MediaType) => {
    setSelectedType(type);
    setSearchQuery("");
    setInputValue("");
    setSelectedResult(null);
  };

  const handleQueryChange = (value: string) => {
    setInputValue(value);
    setSearchQuery(value);
    setSelectedResult(null);
  };

  const handleSelectResult = (result: MediaSearchResult) => {
    if (selectedResult?.external_id === result.external_id) {
      setSelectedResult(null);
      setInputValue(searchQuery);
    } else {
      setSelectedResult(result);
      setInputValue(result.title);
    }
  };

  const handleClearSearch = () => {
    setSearchQuery("");
    setInputValue("");
    setSelectedResult(null);
  };

  const titleToAdd = inputValue.trim();
  const isAdding = createMediaMutation.isPending;
  const canSubmit = Boolean(selectedType && titleToAdd.length > 0 && !isAdding);

  const handleSubmit = async () => {
    if (!selectedType || !titleToAdd) return;

    try {
      await createMediaMutation.mutateAsync({
        title: titleToAdd,
        media_type: selectedType,
        external_id: selectedResult?.external_id ?? null,
      });

      notify.success({
        title: "Media Added",
        description: `Added "${titleToAdd}" to your library.`,
      });

      onSuccess();
    } catch {
      notify.error({
        title: "Failed to Add Media",
        description: "An error occurred while adding the media item.",
      });
    }
  };

  return {
    selectedType,
    searchQuery,
    inputValue,
    displayTitle: inputValue,
    debouncedQuery,
    selectedResult,
    searchResults,
    isLoading,
    shouldSearch,
    canSubmit,
    isAdding,
    selectType: handleSelectType,
    setQuery: handleQueryChange,
    selectResult: handleSelectResult,
    clearSearch: handleClearSearch,
    submit: handleSubmit,
  };
}
