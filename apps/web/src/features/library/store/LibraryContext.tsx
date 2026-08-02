/* eslint-disable react-refresh/only-export-components */
import {
  createContext,
  useContext,
  useMemo,
  useState,
  type ReactNode,
} from "react";

export type LibrarySortField = "title" | "last_played_at";

export type LibrarySortDirection = "asc" | "desc";

export type LibrarySort = {
  field: LibrarySortField;
  direction: LibrarySortDirection;
};

export type LibraryFilters = {
  type?: string;
  status?: string;
  storefront?: string;
};

type LibraryContextValue = {
  search: string;
  setSearch: (value: string) => void;

  filters: LibraryFilters;
  setFilters: (filters: LibraryFilters) => void;

  sort: LibrarySort;
  setSort: (sort: LibrarySort) => void;

  clearFilters: () => void;
};

const defaultSort: LibrarySort = {
  field: "last_played_at",
  direction: "desc",
};

const defaultFilters: LibraryFilters = {};

const LibraryContext = createContext<LibraryContextValue | undefined>(
  undefined,
);

export function LibraryProvider({ children }: { children: ReactNode }) {
  const [search, setSearch] = useState("");
  const [filters, setFilters] = useState<LibraryFilters>(defaultFilters);
  const [sort, setSort] = useState<LibrarySort>(defaultSort);

  const clearFilters = () => {
    setSearch("");
    setFilters(defaultFilters);
    setSort(defaultSort);
  };

  const value = useMemo(
    () => ({
      search,
      setSearch,

      filters,
      setFilters,

      sort,
      setSort,

      clearFilters,
    }),
    [search, filters, sort],
  );

  return (
    <LibraryContext.Provider value={value}>{children}</LibraryContext.Provider>
  );
}

export function useLibraryContext() {
  const context = useContext(LibraryContext);

  if (!context) {
    throw new Error("useLibraryContext must be used inside a LibraryProvider");
  }

  return context;
}
