import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@/test/test-utils";
import { IntegrationCard } from "../IntegrationCard";
import { Gamepad2 } from "lucide-react";

describe("IntegrationCard", () => {
  it("renders card title, category, and description", () => {
    render(
      <IntegrationCard
        title="Steam Storefront"
        category="Storefront"
        description="Sync owned games and playtime from Steam"
        icon={<Gamepad2 />}
        metaText="142 Games Indexed"
        onManage={vi.fn()}
      />,
    );

    expect(screen.getByText("Steam Storefront")).toBeDefined();
    expect(screen.getByText("Storefront")).toBeDefined();
    expect(
      screen.getByText("Sync owned games and playtime from Steam"),
    ).toBeDefined();
    expect(screen.getByText("142 Games Indexed")).toBeDefined();
  });

  it("calls onManage when clicking manage button", () => {
    const onManageMock = vi.fn();
    render(
      <IntegrationCard
        title="Steam Storefront"
        category="Storefront"
        description="Sync owned games"
        icon={<Gamepad2 />}
        onManage={onManageMock}
      />,
    );

    const manageBtn = screen.getByRole("button", { name: /manage/i });
    fireEvent.click(manageBtn);
    expect(onManageMock).toHaveBeenCalled();
  });

  it("toggles enabled switch when onToggleEnabled is provided", () => {
    const onToggleMock = vi.fn();
    render(
      <IntegrationCard
        title="Steam Storefront"
        category="Storefront"
        description="Sync owned games"
        icon={<Gamepad2 />}
        enabled={true}
        onToggleEnabled={onToggleMock}
      />,
    );

    const toggle = screen.getByRole("switch", {
      name: /toggle steam storefront/i,
    });
    fireEvent.click(toggle);
    expect(onToggleMock).toHaveBeenCalledWith(false);
  });
});

