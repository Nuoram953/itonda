import { useMemo, useRef, useEffect } from "react";
import { Link, useSearch } from "@tanstack/react-router";
import { useInfiniteMedia, useMedia } from "../api/get-media";
import { useLibrary } from "../hooks/useLibrary";
import { Card } from "./card";
import { Workspace } from "@/components/workspace/Workspace";
import { Search } from "./action/search";
import { Refresh } from "./action/refresh";
import type { components } from "@/api/generated.d";

const typeTitles: Record<string, string> = {
  game: "Games",
  movie: "Movies",
  tv_show: "TV Series",
};

export const MediaGrid = () => {
  const searchParams = useSearch({ strict: false });
  const type =
    typeof searchParams?.type === "string" ? searchParams.type : undefined;

  const { search, filters, sort } = useLibrary();

  const infiniteQuery = useInfiniteMedia({
    type: (type ?? filters.type) as
      | components["schemas"]["MediaType"]
      | undefined,
    search: search.trim() ? search : undefined,
    status: filters.status as components["schemas"]["MediaStatus"] | undefined,
    storefront: filters.storefront,
    sort_by: sort.field as components["schemas"]["MediaSortField"],
    sort_order: sort.direction as components["schemas"]["SortOrder"],
  });

  const fallbackQuery = useMedia({ type });

  const mediaQuery = infiniteQuery.data ? infiniteQuery : fallbackQuery;

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

  const hasNextPage = infiniteQuery.hasNextPage;
  const isFetchingNextPage = infiniteQuery.isFetchingNextPage;
  const fetchNextPage = infiniteQuery.fetchNextPage;

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

  return (
    <Workspace>
      <Workspace.Header title={title} subtitle={`${totalItems} items`}>
        <Workspace.Actions>
          <Search />
          <Refresh />
        </Workspace.Actions>
      </Workspace.Header>

      <Workspace.Content>
        <ul className="grid grid-cols-[repeat(auto-fill,15rem)] gap-6 justify-center">
          {mediaItems.map((item) => (
            <Link
              key={item.id}
              to="/media/$mediaId"
              params={{ mediaId: item.id }}
            >
              <li>
                <Card media={item} />
              </li>
            </Link>
          ))}
        </ul>

        {hasNextPage && (
          <div ref={loadMoreRef} className="flex justify-center py-6">
            <button
              onClick={() => fetchNextPage()}
              disabled={isFetchingNextPage}
              className="rounded border border-border-strong bg-surface px-4 py-2 text-sm font-medium hover:bg-surface-hover disabled:opacity-50"
            >
              {isFetchingNextPage ? "Loading more..." : "Load more"}
            </button>
          </div>
        )}
      </Workspace.Content>
    </Workspace>
  );
};
