import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@/test/test-utils";
import { SecretInput } from "../SecretInput";

describe("SecretInput", () => {
  it("renders with masked password input by default", () => {
    render(
      <SecretInput
        value="secret-api-key-123"
        onChange={vi.fn()}
        placeholder="Enter key..."
      />,
    );

    const input = screen.getByPlaceholderText("Enter key...") as HTMLInputElement;
    expect(input.getAttribute("type")).toBe("password");
    expect(input.value).toBe("secret-api-key-123");
  });

  it("toggles password visibility when clicking eye button", () => {
    render(
      <SecretInput
        value="secret-api-key-123"
        onChange={vi.fn()}
        placeholder="Enter key..."
      />,
    );

    const toggleBtn = screen.getByRole("button", { name: /show api key/i });
    fireEvent.click(toggleBtn);

    const input = screen.getByPlaceholderText("Enter key...") as HTMLInputElement;
    expect(input.getAttribute("type")).toBe("text");

    const hideBtn = screen.getByRole("button", { name: /hide api key/i });
    fireEvent.click(hideBtn);
    expect(input.getAttribute("type")).toBe("password");
  });

  it("copies value to clipboard on copy button click", async () => {
    const writeTextMock = vi.fn().mockResolvedValue(undefined);
    Object.assign(navigator, {
      clipboard: {
        writeText: writeTextMock,
      },
    });

    render(
      <SecretInput
        value="my-copied-key"
        onChange={vi.fn()}
        placeholder="Enter key..."
      />,
    );

    const copyBtn = screen.getByRole("button", { name: /copy to clipboard/i });
    fireEvent.click(copyBtn);

    expect(writeTextMock).toHaveBeenCalledWith("my-copied-key");
  });

  it("calls onTest when clicking test key button", () => {
    const onTestMock = vi.fn();
    render(
      <SecretInput
        value="test-key"
        onChange={vi.fn()}
        onTest={onTestMock}
      />,
    );

    const testBtn = screen.getByRole("button", { name: /test key/i });
    fireEvent.click(testBtn);
    expect(onTestMock).toHaveBeenCalled();
  });

  it("renders external portal link when portalUrl is provided", () => {
    render(
      <SecretInput
        value=""
        onChange={vi.fn()}
        portalUrl="https://steamcommunity.com/dev/apikey"
        portalLabel="Get Steam API key"
      />,
    );

    const link = screen.getByRole("link", { name: /get steam api key/i });
    expect(link.getAttribute("href")).toBe(
      "https://steamcommunity.com/dev/apikey",
    );
  });
});
