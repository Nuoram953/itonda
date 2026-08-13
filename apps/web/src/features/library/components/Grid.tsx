import { Link } from "@tanstack/react-router";
import { useMediaGrid } from "../hooks/useMediaGrid";
import { Card } from "./card";
import { Workspace } from "@/components/workspace/Workspace";
import { Filters } from "./action/Filters";
import { Sort } from "./action/sort";
import { Refresh } from "./action/refresh";
import { Layers } from "lucide-react";

import { EmptyState } from "@/components/feedback/EmptyState";

export const MediaGrid = () => {
  const {
    title,
    mediaItems,
    totalItems,
    hasNextPage,
    isFetchingNextPage,
    fetchNextPage,
    loadMoreRef,
  } = useMediaGrid();

  return (
    <Workspace>
      <Workspace.Header title={title} subtitle={`${totalItems} items`}>
        <Workspace.Actions>
          <Filters />
          <Sort />
          <Refresh />
        </Workspace.Actions>
      </Workspace.Header>

      <Workspace.Content className="p-6">
        {mediaItems.length === 0 ? (
          <EmptyState
            icon={<Layers className="w-8 h-8 text-primary/70" />}
            title="No media found"
            message="No items match your active filters or search criteria. Try clearing search or refreshing the library."
          />
        ) : (
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
        )}

        {hasNextPage && (
          <div ref={loadMoreRef} className="flex justify-center py-8">
            <button
              onClick={() => fetchNextPage()}
              disabled={isFetchingNextPage}
              className="px-5 py-2.5 rounded-xl border border-white/10 bg-surface/80 text-foreground text-xs font-semibold hover:bg-surface-hover hover:border-white/20 disabled:opacity-50 transition-all duration-200 shadow-md cursor-pointer"
            >
              {isFetchingNextPage ? "Loading more..." : "Load More Items"}
            </button>
          </div>
        )}
      </Workspace.Content>
    </Workspace>
  );
};
