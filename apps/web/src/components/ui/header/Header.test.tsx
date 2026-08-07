import { describe, it, expect, vi } from "vitest";
import { screen, render } from "@/test/test-utils";
import { Header } from "./Header";

vi.mock("@/hooks/use-websocket-status", () => ({
  useWebSocketStatus: () => "connected",
}));

vi.mock("@/api/get-agents", () => ({
  useAgents: () => ({ data: { agents: [] }, isLoading: false }),
}));

describe("Header", () => {
  it("renders name", () => {
    render(<Header />);

    expect(screen.findByText("Itonda")).toBeDefined();
  });
});
