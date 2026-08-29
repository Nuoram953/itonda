import { describe, it, expect, vi, beforeEach } from "vitest";
import { screen, render, fireEvent, createAgent } from "@/test/test-utils";
import { ServerAgentStatus } from "./ServerAgentStatus";
import { useWebSocketStatus } from "@/hooks/use-websocket-status";
import { useAgents } from "@/api/get-agents";

vi.mock("@/hooks/use-websocket-status", () => ({
  useWebSocketStatus: vi.fn(),
}));

vi.mock("@/api/get-agents", () => ({
  useAgents: vi.fn(),
  getAgentsQueryOptions: () => ({ queryKey: ["agents"] }),
}));

describe("ServerAgentStatus", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders trigger button with server offline state (red)", () => {
    vi.mocked(useWebSocketStatus).mockReturnValue("disconnected");
    vi.mocked(useAgents).mockReturnValue({
      data: { agents: [] },
      isLoading: false,
    } as unknown as ReturnType<typeof useAgents>);

    render(<ServerAgentStatus />);

    const button = screen.getByRole("button", {
      name: /Server and Agent Status/i,
    });
    expect(button).toBeDefined();
  });

  it("renders trigger button with server online but no agents state (yellow)", () => {
    vi.mocked(useWebSocketStatus).mockReturnValue("connected");
    vi.mocked(useAgents).mockReturnValue({
      data: { agents: [] },
      isLoading: false,
    } as unknown as ReturnType<typeof useAgents>);

    render(<ServerAgentStatus />);

    const button = screen.getByRole("button", {
      name: /Server and Agent Status/i,
    });
    expect(button).toBeDefined();
  });

  it("renders trigger button with server online and agents connected state (green)", () => {
    vi.mocked(useWebSocketStatus).mockReturnValue("connected");
    vi.mocked(useAgents).mockReturnValue({
      data: {
        agents: [
          createAgent({
            id: "agent-1",
            name: "Desktop-Agent",
            hostname: "desktop-pc",
            platform: "linux",
            is_connected: true,
          }),
        ],
      },
      isLoading: false,
    } as ReturnType<typeof useAgents>);

    render(<ServerAgentStatus />);

    const button = screen.getByRole("button", {
      name: /Server and Agent Status/i,
    });
    expect(button).toBeDefined();
  });

  it("opens popover content on click and displays agent details", () => {
    vi.mocked(useWebSocketStatus).mockReturnValue("connected");
    vi.mocked(useAgents).mockReturnValue({
      data: {
        agents: [
          createAgent({
            id: "agent-12345678",
            name: "Primary Agent Node",
            hostname: "agent-host-1",
            platform: "linux",
            is_connected: true,
          }),
        ],
      },
      isLoading: false,
    } as ReturnType<typeof useAgents>);

    render(<ServerAgentStatus />);

    const button = screen.getByRole("button", {
      name: /Server and Agent Status/i,
    });
    fireEvent.click(button);

    expect(screen.getByText("itonda-server")).toBeDefined();
    expect(screen.getByText("Primary Agent Node")).toBeDefined();
    expect(screen.getByText("agent-host-1")).toBeDefined();
    expect(screen.getByText("linux")).toBeDefined();
  });

  it("displays empty state message when no agents are connected", () => {
    vi.mocked(useWebSocketStatus).mockReturnValue("connected");
    vi.mocked(useAgents).mockReturnValue({
      data: { agents: [] },
      isLoading: false,
    } as unknown as ReturnType<typeof useAgents>);

    render(<ServerAgentStatus />);

    const button = screen.getByRole("button", {
      name: /Server and Agent Status/i,
    });
    fireEvent.click(button);

    expect(screen.getByText("No Agents Paired")).toBeDefined();
  });
});

