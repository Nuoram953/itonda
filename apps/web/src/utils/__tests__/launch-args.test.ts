import { describe, expect, it } from "vitest";
import { formatArgsString, parseArgsString } from "../launch-args";

describe("launch-args", () => {
  it("splits space-separated arguments and preserves quoted strings", () => {
    expect(parseArgsString('-b -e "/roms/Super Mario.iso" --fullscreen')).toEqual([
      "-b",
      "-e",
      "/roms/Super Mario.iso",
      "--fullscreen",
    ]);
  });

  it("handles empty or whitespace-only strings", () => {
    expect(parseArgsString("   ")).toEqual([]);
  });

  it("formats arguments and quotes those with spaces", () => {
    expect(
      formatArgsString(["-b", "-e", "/roms/Super Mario.iso", "{game}"]),
    ).toBe('-b -e "/roms/Super Mario.iso" {game}');
  });
});
