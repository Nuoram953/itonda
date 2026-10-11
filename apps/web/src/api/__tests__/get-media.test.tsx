import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import type { ReactNode } from "react";
import { api } from "@/lib/api-client";
import {
  getMedia,
  getMediaQueryOptions,
  getInfiniteMediaQueryOptions,
  useMedia,
  useInfiniteMedia,
} from "../get-media";
import { createMediaResponse } from "@/test/test-utils";

vi.mock("@/lib/api-client", () => ({
  api: {
    get: vi.fn(),
  },
}));

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: { retry: false },
    },
  });
  return function Wrapper({ children }: { children: ReactNode }) {
    return (
      <QueryClientProvider client={queryClient}>
        {children}
      </QueryClientProvider>
    );
  };
}

describe("get-media API module", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe("getMedia", () => {
    it("calls api.get with /media and provided params", async () => {
      const mockResponse = createMediaResponse({ items: [] });
      vi.mocked(api.get).mockResolvedValueOnce(mockResponse);

      const params = { search: "Elden", limit: 8, sort_by: "last_played_at" as const };
      const result = await getMedia(params);

      expect(api.get).toHaveBeenCalledWith("/media", { params });
      expect(result).toEqual(mockResponse);
    });
  });

  describe("getMediaQueryOptions", () => {
    it("creates query options with correct queryKey and queryFn", async () => {
      const params = { search: "Cyberpunk" };
      const options = getMediaQueryOptions(params);

      expect(options.queryKey).toEqual(["media", params]);
      expect(typeof options.queryFn).toBe("function");
    });
  });

  describe("getInfiniteMediaQueryOptions", () => {
    it("creates infinite query options with pageParam handler", async () => {
      const mockResponse = createMediaResponse({ items: [], page: 1, has_next: true });
      vi.mocked(api.get).mockResolvedValueOnce(mockResponse);

      const options = getInfiniteMediaQueryOptions({ search: "Halo" });
      expect(options.queryKey).toEqual(["media", "infinite", { search: "Halo" }]);
      expect(options.initialPageParam).toBe(1);

      const next = options.getNextPageParam(mockResponse, [mockResponse], 1, [1]);
      expect(next).toBe(2);
    });
  });

  describe("useMedia hook", () => {
    it("resolves nested params object and queryConfig properly", async () => {
      const mockResponse = createMediaResponse({ items: [] });
      vi.mocked(api.get).mockResolvedValueOnce(mockResponse);

      const { result } = renderHook(
        () =>
          useMedia({
            params: { search: "Elden", limit: 8 },
            queryConfig: { enabled: true },
          }),
        { wrapper: createWrapper() },
      );

      await waitFor(() => {
        expect(result.current.isSuccess).toBe(true);
      });

      expect(api.get).toHaveBeenCalledWith("/media", {
        params: { search: "Elden", limit: 8 },
      });
    });

    it("resolves direct parameter properties without nested params object", async () => {
      const mockResponse = createMediaResponse({ items: [] });
      vi.mocked(api.get).mockResolvedValueOnce(mockResponse);

      const { result } = renderHook(
        () =>
          useMedia({
            search: "Direct",
            type: "game",
          }),
        { wrapper: createWrapper() },
      );

      await waitFor(() => {
        expect(result.current.isSuccess).toBe(true);
      });

      expect(api.get).toHaveBeenCalledWith("/media", {
        params: { search: "Direct", type: "game" },
      });
    });

    it("respects enabled: false in queryConfig", async () => {
      const { result } = renderHook(
        () =>
          useMedia({
            params: { search: "Disabled" },
            queryConfig: { enabled: false },
          }),
        { wrapper: createWrapper() },
      );

      expect(result.current.isFetching).toBe(false);
      expect(api.get).not.toHaveBeenCalled();
    });
  });

  describe("useInfiniteMedia hook", () => {
    it("fetches first page through infinite query", async () => {
      const mockResponse = createMediaResponse({ items: [], page: 1, has_next: false });
      vi.mocked(api.get).mockResolvedValueOnce(mockResponse);

      const { result } = renderHook(
        () => useInfiniteMedia({ type: "game" }),
        { wrapper: createWrapper() },
      );

      await waitFor(() => {
        expect(result.current.isSuccess).toBe(true);
      });

      expect(api.get).toHaveBeenCalledWith("/media", {
        params: { type: "game", page: 1 },
      });
    });
  });
});
