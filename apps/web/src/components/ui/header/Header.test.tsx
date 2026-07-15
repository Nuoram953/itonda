import { describe, it, expect } from "vitest";
import { screen, render } from "@/test/test-utils";
import { Header } from "./Header";

describe("Header", () => {
  it("renders name", () => {
    render(<Header />);

    expect(screen.findByText("Itonda")).toBeDefined();
  });
});
