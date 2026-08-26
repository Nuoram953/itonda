import { useMemo, useRef, useEffect } from "react";
import { useSearch } from "@tanstack/react-router";
import { useInfiniteMedia } from "../api/get-media";
import { useLibrary } from "./useLibrary";
import type { components } from "@/api/generated.d";

const typeTitles: Record<string, string> = {
  game: "Games",
  movie: "Movies",
  tv_show: "TV Series",
};

export function useMediaGrid() {
  const searchParams = useSearch({ strict: false });
  const type =
    typeof searchParams?.type === "string" ? searchParams.type : undefined;

  const { search, filters, sort } = useLibrary();

  const mediaQuery = useInfiniteMedia({
    type: (type ?? filters.type) as
      | components["schemas"]["MediaType"]
      | undefined,
    search: search.trim() ? search : undefined,
    status: filters.status as components["schemas"]["MediaStatus"] | undefined,
    storefront: filters.storefront,
    sort_by: sort.field as components["schemas"]["MediaSortField"],
    sort_order: sort.direction as components["schemas"]["SortOrder"],
  });

  const mediaItems = useMemo(() => {
    if (!mediaQuery.data) return [];
    if ("pages" in mediaQuery.data && Array.isArray(mediaQuery.data.pages)) {
      return mediaQuery.data.pages.flatMap((page) => page.items ?? []);
    }
    if ("items" in mediaQuery.data && Array.isArray(mediaQuery.data.items)) {
      return mediaQuery.data.items;
    }
    return [];
  }, [mediaQuery.data]);

  const totalItems = useMemo(() => {
    if (!mediaQuery.data) return 0;
    if ("pages" in mediaQuery.data && Array.isArray(mediaQuery.data.pages)) {
      return mediaQuery.data.pages[0]?.total ?? mediaItems.length;
    }
    if ("items" in mediaQuery.data && Array.isArray(mediaQuery.data.items)) {
      return (mediaQuery.data as { total?: number }).total ?? mediaItems.length;
    }
    return mediaItems.length;
  }, [mediaQuery.data, mediaItems.length]);

  const hasNextPage = mediaQuery.hasNextPage;
  const isFetchingNextPage = mediaQuery.isFetchingNextPage;
  const fetchNextPage = mediaQuery.fetchNextPage;

  const loadMoreRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!hasNextPage || isFetchingNextPage) return;

    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0]?.isIntersecting) {
          fetchNextPage();
        }
      },
      { threshold: 0.1 },
    );

    const currentRef = loadMoreRef.current;
    if (currentRef) {
      observer.observe(currentRef);
    }

    return () => {
      if (currentRef) {
        observer.unobserve(currentRef);
      }
    };
  }, [hasNextPage, isFetchingNextPage, fetchNextPage]);

  const title = type ? (typeTitles[type] ?? "Media") : "Media";

  return {
    title,
    type,
    mediaItems,
    totalItems,
    hasNextPage,
    isFetchingNextPage,
    fetchNextPage,
    loadMoreRef,
    isLoading: mediaQuery.isLoading,
  };
}
