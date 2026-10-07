import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@/test/test-utils";
import { IntegrationRow } from "../IntegrationRow";
import { Gamepad2 } from "lucide-react";

describe("IntegrationRow", () => {
  it("renders row title, media type badges, and description", () => {
    render(
      <IntegrationRow
        title="Steam Storefront"
        mediaTypes={["game"]}
        description="Sync owned games and playtime from Steam"
        icon={<Gamepad2 />}
        onOpenSheet={vi.fn()}
      />,
    );

    expect(screen.getByText("Steam Storefront")).toBeDefined();
    expect(screen.getByText("Game")).toBeDefined();
    expect(
      screen.getByText("Sync owned games and playtime from Steam"),
    ).toBeDefined();
  });

  it("renders issue message when issueText is provided and hides when omitted", () => {
    const { rerender } = render(
      <IntegrationRow
        title="Steam Storefront"
        description="Sync owned games"
        icon={<Gamepad2 />}
        issueText="Account not linked — sign in to sync library"
      />,
    );

    expect(
      screen.getByText("Account not linked — sign in to sync library"),
    ).toBeDefined();

    rerender(
      <IntegrationRow
        title="Steam Storefront"
        description="Sync owned games"
        icon={<Gamepad2 />}
      />,
    );

    expect(
      screen.queryByText("Account not linked — sign in to sync library"),
    ).toBeNull();
  });

  it("calls onOpenSheet when clicking configure icon button", () => {
    const onOpenSheetMock = vi.fn();
    render(
      <IntegrationRow
        title="Steam Storefront"
        mediaTypes={["game"]}
        description="Sync owned games"
        icon={<Gamepad2 />}
        onOpenSheet={onOpenSheetMock}
      />,
    );

    const configureBtn = screen.getByRole("button", {
      name: "Configure Steam Storefront",
    });
    fireEvent.click(configureBtn);
    expect(onOpenSheetMock).toHaveBeenCalledTimes(1);
  });

  it("toggles enabled switch when onToggleEnabled is provided", () => {
    const onToggleMock = vi.fn();
    render(
      <IntegrationRow
        title="Steam Storefront"
        mediaTypes={["game"]}
        description="Sync owned games"
        icon={<Gamepad2 />}
        enabled={true}
        onToggleEnabled={onToggleMock}
      />,
    );

    const toggle = screen.getByRole("switch", {
      name: "Toggle Steam Storefront",
    });
    fireEvent.click(toggle);
    expect(onToggleMock).toHaveBeenCalledWith(false);
  });
});
