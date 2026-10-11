import { useState, useEffect, useRef, useMemo } from "react";
import { useQuery } from "@tanstack/react-query";
import type { components } from "@/api/generated.d";
import { getMediaQueryOptions } from "@/features/library/api/get-media";
import { useDebounce } from "@/hooks/use-debounce";

export type MediaItem = components["schemas"]["Media"];
export type SearchAction = "start" | "detail";

export type UseGlobalSearchOptions = {
  onExecuteAction?: (item: MediaItem, action: SearchAction) => void;
};

export function useGlobalSearch({ onExecuteAction }: UseGlobalSearchOptions = {}) {
  const [query, setQuery] = useState("");
  const [isOpen, setIsOpen] = useState(false);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [actionOverride, setActionOverride] = useState<SearchAction | null>(null);

  const inputRef = useRef<HTMLInputElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  const debouncedQuery = useDebounce(query, 250);

  const isDebouncing =
    query.trim().length > 0 && query.trim() !== debouncedQuery.trim();

  const isQueryActive = debouncedQuery.trim().length > 0;

  const { data, isFetching } = useQuery({
    ...getMediaQueryOptions({
      search: debouncedQuery.trim(),
      limit: 8,
    }),
    enabled: isQueryActive,
  });

  const rawResults = data?.items;
  const results = useMemo(() => {
    return isQueryActive ? rawResults ?? [] : [];
  }, [isQueryActive, rawResults]);

  const isLoading = (isDebouncing || isFetching) && query.trim().length > 0;

  // Derive activeIndex and activeAction cleanly without an effect
  const activeIndex =
    results.length > 0
      ? selectedIndex >= 0 && selectedIndex < results.length
        ? selectedIndex
        : 0
      : -1;

  const currentItem = activeIndex >= 0 ? results[activeIndex] : undefined;
  const canLaunchCurrentItem = Boolean(
    currentItem?.media_type === "game" &&
      (currentItem?.launches?.length ?? 0) > 0,
  );

  const defaultActionForCurrentItem: SearchAction = canLaunchCurrentItem
    ? "start"
    : "detail";

  const activeAction: SearchAction =
    actionOverride !== null && canLaunchCurrentItem
      ? actionOverride
      : defaultActionForCurrentItem;

  // Global shortcut: ⌘K or Ctrl+K to focus search
  useEffect(() => {
    const handleGlobalKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && (e.key === "k" || e.key === "K")) {
        e.preventDefault();
        setIsOpen(true);
        inputRef.current?.focus();
        inputRef.current?.select();
      }
    };

    window.addEventListener("keydown", handleGlobalKeyDown);
    return () => window.removeEventListener("keydown", handleGlobalKeyDown);
  }, []);

  // Click outside to close dropdown
  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (
        containerRef.current &&
        !containerRef.current.contains(e.target as Node)
      ) {
        setIsOpen(false);
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (!isOpen || results.length === 0) {
      if (e.key === "ArrowDown" && query.trim().length > 0) {
        e.preventDefault();
        setIsOpen(true);
      }
      return;
    }

    if (e.key === "Tab") {
      e.preventDefault();
      // Option A: Strictly toggle between 'start' and 'detail' on the currently active item
      if (canLaunchCurrentItem) {
        setActionOverride(activeAction === "start" ? "detail" : "start");
      }
      return;
    }

    if (e.key === "ArrowDown") {
      e.preventDefault();
      const nextIndex = (activeIndex + 1) % results.length;
      setSelectedIndex(nextIndex);
      setActionOverride(null);
      return;
    }

    if (e.key === "ArrowUp") {
      e.preventDefault();
      const prevIndex = (activeIndex - 1 + results.length) % results.length;
      setSelectedIndex(prevIndex);
      setActionOverride(null);
      return;
    }

    if (e.key === "Enter") {
      e.preventDefault();
      if (currentItem) {
        onExecuteAction?.(currentItem, activeAction);
      }
      return;
    }

    if (e.key === "Escape") {
      e.preventDefault();
      setIsOpen(false);
      inputRef.current?.blur();
      return;
    }
  };

  const handleSetQuery = (val: string) => {
    setQuery(val);
    setSelectedIndex(0);
    setActionOverride(null);
  };

  const clearQuery = () => {
    setQuery("");
    setSelectedIndex(0);
    setActionOverride(null);
    setIsOpen(false);
    inputRef.current?.focus();
  };

  const setActiveIndex = (idx: number) => {
    setSelectedIndex(idx);
    setActionOverride(null);
  };

  const setActiveAction = (action: SearchAction) => {
    setActionOverride(action);
  };

  return {
    query,
    setQuery: handleSetQuery,
    clearQuery,
    isOpen,
    setIsOpen,
    debouncedQuery,
    activeIndex,
    setActiveIndex,
    activeAction,
    setActiveAction,
    results,
    isLoading,
    handleKeyDown,
    inputRef,
    containerRef,
  };
}
