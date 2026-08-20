import { describe, it, expect } from "vitest";
import { render, screen } from "@/test/test-utils";
import { SettingRow } from "../SettingRow";
import { Switch } from "@/components/ui/switch";

describe("SettingRow", () => {
  it("renders label and description properly", () => {
    render(
      <SettingRow
        label="Steam Integration"
        description="Connect your Steam account"
      >
        <Switch aria-label="Toggle Steam" />
      </SettingRow>,
    );

    expect(screen.getByText("Steam Integration")).toBeDefined();
    expect(
      screen.getByText("Connect your Steam account"),
    ).toBeDefined();
    expect(screen.getByRole("switch", { name: "Toggle Steam" })).toBeDefined();
  });

  it("renders vertical column layout when layout='vertical' is specified", () => {
    render(
      <SettingRow
        label="Steam API Key"
        description="Enter your 32-character key"
        layout="vertical"
        className="custom-setting-row"
      >
        <input data-testid="test-input" />
      </SettingRow>,
    );

    expect(screen.getByText("Steam API Key")).toBeDefined();
    expect(screen.getByText("Enter your 32-character key")).toBeDefined();
    expect(screen.getByTestId("test-input")).toBeDefined();
    const rowElement = screen.getByTestId("test-input").closest(".custom-setting-row");
    expect(rowElement?.className).toContain("flex-col");
    expect(rowElement?.className).not.toContain("sm:flex-row");
  });
});
