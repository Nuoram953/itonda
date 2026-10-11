import { useState, useCallback } from "react";
import { useNavigate } from "@tanstack/react-router";
import { useMedia } from "@/api/get-media";
import { useDebounce } from "@/hooks/use-debounce";
import { useMediaLauncher, type MediaItem } from "@/hooks/use-media-launcher";
import type { SearchAction } from "./GlobalSearchResultRow";

export function useGlobalSearch() {
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [selectedId, setSelectedId] = useState("");
  const [actionOverride, setActionOverride] = useState<SearchAction | null>(null);

  const navigate = useNavigate();
  const launcher = useMediaLauncher();
  const debouncedQuery = useDebounce(query, 250);
  const isQueryActive = debouncedQuery.trim().length > 0;

  const { data, isFetching } = useMedia({
    params: {
      search: debouncedQuery.trim(),
      limit: 8,
      sort_by: "last_played_at",
      sort_order: "desc",
    },
    queryConfig: { enabled: isQueryActive && open },
  });

  const results = isQueryActive ? (data?.items ?? []) : [];
  const isLoading =
    (isFetching || query.trim() !== debouncedQuery.trim()) &&
    query.trim().length > 0;

  const activeItem = results.find((i) => i.id === selectedId) ?? results[0];
  const canLaunch = Boolean(activeItem && launcher.canLaunchMedia(activeItem));
  const activeAction: SearchAction =
    canLaunch && actionOverride !== "detail" ? "start" : "detail";

  const executeAction = useCallback(
    (item: MediaItem, action: SearchAction) => {
      setOpen(false);
      if (action === "start") {
        launcher.handleLaunchMedia(item);
      } else {
        navigate({ to: "/media/$mediaId", params: { mediaId: item.id } });
      }
    },
    [launcher, navigate],
  );

  const toggleActionOverride = useCallback(() => {
    if (!canLaunch) return;
    setActionOverride((prev) => (prev === "detail" ? "start" : "detail"));
  }, [canLaunch]);

  const handleHover = useCallback(
    (itemId: string, action?: SearchAction) => {
      setSelectedId(itemId);
      if (action) setActionOverride(action);
    },
    [],
  );

  const handleDialogOpenChange = useCallback((nextOpen: boolean) => {
    setOpen(nextOpen);
    if (!nextOpen) {
      setQuery("");
      setSelectedId("");
      setActionOverride(null);
    }
  }, []);

  return {
    open,
    setOpen,
    query,
    setQuery,
    debouncedQuery,
    isQueryActive,
    results,
    isLoading,
    selectedId,
    setSelectedId,
    activeItem,
    canLaunch,
    activeAction,
    actionOverride,
    setActionOverride,
    launcher,
    executeAction,
    toggleActionOverride,
    handleHover,
    handleDialogOpenChange,
  };
}
