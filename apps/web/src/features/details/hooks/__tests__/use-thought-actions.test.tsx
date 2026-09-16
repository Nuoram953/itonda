import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { useThoughtActions } from "../use-thought-actions";
import { useCreateThought } from "../../api/post-thought";
import { useUpdateThought } from "../../api/put-thought";
import { useDeleteThought } from "../../api/delete-thought";

vi.mock("../../api/post-thought", () => ({
  useCreateThought: vi.fn(),
}));

vi.mock("../../api/put-thought", () => ({
  useUpdateThought: vi.fn(),
}));

vi.mock("../../api/delete-thought", () => ({
  useDeleteThought: vi.fn(),
}));

describe("useThoughtActions Hook", () => {
  const mockMutateCreate = vi.fn();
  const mockMutateUpdate = vi.fn();
  const mockMutateDelete = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();

    vi.mocked(useCreateThought).mockReturnValue({
      mutateAsync: mockMutateCreate,
      isPending: false,
    } as unknown as ReturnType<typeof useCreateThought>);

    vi.mocked(useUpdateThought).mockReturnValue({
      mutateAsync: mockMutateUpdate,
      isPending: false,
    } as unknown as ReturnType<typeof useUpdateThought>);

    vi.mocked(useDeleteThought).mockReturnValue({
      mutateAsync: mockMutateDelete,
      isPending: false,
    } as unknown as ReturnType<typeof useDeleteThought>);
  });

  it("calls createThought mutation when createThought is invoked", async () => {
    mockMutateCreate.mockResolvedValueOnce({});

    const { result } = renderHook(() =>
      useThoughtActions({ mediaId: "media-1" }),
    );

    const thoughtData = {
      title: "Epic Boss",
      content: "Challenging phase 2",
      category: "gameplay",
      playtime_minutes: 60,
    };

    await act(async () => {
      await result.current.createThought(thoughtData);
    });

    expect(mockMutateCreate).toHaveBeenCalledWith(thoughtData);
  });

  it("calls updateThought mutation when updateThought is invoked", async () => {
    mockMutateUpdate.mockResolvedValueOnce({});

    const { result } = renderHook(() =>
      useThoughtActions({ mediaId: "media-1" }),
    );

    const updateData = {
      title: "Updated Title",
      content: "Updated Content",
      category: "story",
    };

    await act(async () => {
      await result.current.updateThought("thought-1", updateData);
    });

    expect(mockMutateUpdate).toHaveBeenCalledWith({
      thoughtId: "thought-1",
      payload: updateData,
    });
  });

  it("calls deleteThought mutation when deleteThought is invoked", async () => {
    mockMutateDelete.mockResolvedValueOnce(undefined);

    const { result } = renderHook(() =>
      useThoughtActions({ mediaId: "media-1" }),
    );

    await act(async () => {
      await result.current.deleteThought("thought-1");
    });

    expect(mockMutateDelete).toHaveBeenCalledWith("thought-1");
  });
});
