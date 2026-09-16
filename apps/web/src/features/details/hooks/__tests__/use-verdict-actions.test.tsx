import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { useVerdictActions } from "../use-verdict-actions";
import { useUpsertReview } from "../../api/put-review";
import { useDeleteReview } from "../../api/delete-review";

vi.mock("../../api/put-review", () => ({
  useUpsertReview: vi.fn(),
}));

vi.mock("../../api/delete-review", () => ({
  useDeleteReview: vi.fn(),
}));

describe("useVerdictActions Hook", () => {
  const mockMutateUpsert = vi.fn();
  const mockMutateDelete = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();

    vi.mocked(useUpsertReview).mockReturnValue({
      mutateAsync: mockMutateUpsert,
      isPending: false,
    } as unknown as ReturnType<typeof useUpsertReview>);

    vi.mocked(useDeleteReview).mockReturnValue({
      mutateAsync: mockMutateDelete,
      isPending: false,
    } as unknown as ReturnType<typeof useDeleteReview>);
  });

  it("calls upsertReview mutation when saveVerdict is invoked", async () => {
    mockMutateUpsert.mockResolvedValueOnce({
      media_id: "media-1",
      verdict: "masterpiece",
      summary: "Great game",
    });

    const { result } = renderHook(() =>
      useVerdictActions({ mediaId: "media-1" }),
    );

    await act(async () => {
      await result.current.saveVerdict({
        verdict: "masterpiece",
        summary: "Great game",
      });
    });

    expect(mockMutateUpsert).toHaveBeenCalledWith({
      verdict: "masterpiece",
      summary: "Great game",
    });
  });

  it("calls deleteReview mutation when deleteVerdict is invoked", async () => {
    mockMutateDelete.mockResolvedValueOnce(undefined);

    const { result } = renderHook(() =>
      useVerdictActions({ mediaId: "media-1" }),
    );

    await act(async () => {
      await result.current.deleteVerdict();
    });

    expect(mockMutateDelete).toHaveBeenCalledTimes(1);
  });
});
