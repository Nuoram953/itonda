import { describe, it, expect } from "vitest";
import { render, screen } from "@/test/test-utils";
import { Card } from "../card";

describe("Card", () => {
  it("renders the card title", () => {
    render(
      <Card
        media={{
          assets: [],
          id: "",
          media_type: "",
          title: "Elden Ring",
        }}
      />,
    );

    expect(screen.getByText("Elden Ring")).toBeDefined();
  });

  it("renders the title inside a heading", () => {
    render(
      <Card
        media={{
          assets: [],
          id: "",
          media_type: "",
          title: "Cyberpunk 2077",
        }}
      />,
    );

    const title = screen.getByRole("heading", {
      name: "Cyberpunk 2077",
    });

    expect(title).toBeDefined();
  });

  it("renders a card article container", () => {
    render(
      <Card
        media={{
          assets: [],
          id: "",
          media_type: "",
          title: "Cyberpunk 2077",
        }}
      />,
    );

    expect(screen.getByRole("article")).toBeDefined();
  });
});
