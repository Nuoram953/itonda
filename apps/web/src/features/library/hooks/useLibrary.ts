import { useMemo } from "react";

import {
  useLibraryContext,
  type LibraryFilters,
  type LibrarySort,
} from "../store/LibraryContext";

import type { components } from "@/api/generated.d";

type Media = components["schemas"]["Media"];

export function useLibrary() {
  const library = useLibraryContext();

  const applyFilters = (
    media: Media[],
    search: string,
    filters: LibraryFilters,
    sort: LibrarySort,
  ) => {
    let result = [...media];

    if (search.trim()) {
      const query = search.toLowerCase();

      result = result.filter((item) =>
        item.title.toLowerCase().includes(query),
      );
    }

    if (filters.type) {
      result = result.filter((item) => item.media_type === filters.type);
    }

    if (filters.status) {
      result = result.filter((item) => item.status === filters.status);
    }

    result.sort((a, b) => {
      switch (sort.field) {
        case "title":
          return a.title.localeCompare(b.title);

        case "last_played_at":
          return (
            (a.details?.last_played_at ?? Number.MAX_SAFE_INTEGER) -
            (b.details?.last_played_at ?? Number.MAX_SAFE_INTEGER)
          );

        default:
          return 0;
      }
    });

    if (sort.direction === "desc") {
      result.reverse();
    }

    return result;
  };

  return useMemo(
    () => ({
      ...library,
      applyFilters,
    }),
    [library],
  );
}
