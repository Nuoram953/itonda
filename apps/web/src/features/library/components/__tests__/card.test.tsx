import { describe, it, expect } from "vitest";
import { fireEvent } from "@testing-library/react";
import { render, screen } from "@/test/test-utils";
import { Card } from "../card";
import type { components } from "@/api/generated.d";

const mockMedia: components["schemas"]["Media"] = {
  id: "media-1",
  title: "Elden Ring",
  media_type: "game",
  status: "in_progress",
  assets: [],
  launches: [],
};

describe("Card", () => {
  it("renders the card title", () => {
    render(<Card media={mockMedia} />);

    expect(screen.getByText("Elden Ring")).toBeDefined();
  });

  it("renders the title inside a heading", () => {
    render(<Card media={{ ...mockMedia, title: "Cyberpunk 2077" }} />);

    const title = screen.getByRole("heading", {
      name: "Cyberpunk 2077",
    });

    expect(title).toBeDefined();
  });

  it("renders a card article container", () => {
    render(<Card media={mockMedia} />);

    expect(screen.getByRole("article")).toBeDefined();
  });

  it("does not render an image when no poster asset is present", () => {
    render(
      <Card
        media={{
          ...mockMedia,
          assets: [{ id: "asset-1", asset_type: "backdrop" }],
        }}
      />,
    );

    expect(screen.queryByRole("img")).toBeNull();
  });

  it("renders poster image with correct attributes when poster asset exists", () => {
    render(
      <Card
        media={{
          ...mockMedia,
          assets: [{ id: "poster-123", asset_type: "poster" }],
        }}
      />,
    );

    const img = screen.getByRole("img", { name: "Elden Ring" });
    expect(img).toBeDefined();
    expect(img.getAttribute("src")).toBe(
      "http://localhost:3005/api/v1/assets/poster-123",
    );
  });

  it("updates opacity from 0 to 100 after image loads", () => {
    render(
      <Card
        media={{
          ...mockMedia,
          assets: [{ id: "poster-123", asset_type: "poster" }],
        }}
      />,
    );

    const img = screen.getByRole("img");
    expect(img.className).toContain("opacity-0");

    fireEvent.load(img);

    expect(img.className).toContain("opacity-100");
  });

  it("updates opacity from 0 to 100 after image error occurs", () => {
    render(
      <Card
        media={{
          ...mockMedia,
          assets: [{ id: "poster-123", asset_type: "poster" }],
        }}
      />,
    );

    const img = screen.getByRole("img");
    expect(img.className).toContain("opacity-0");

    fireEvent.error(img);

    expect(img.className).toContain("opacity-100");
  });

  describe("status accent colors", () => {
    const statuses: Array<{
      status: components["schemas"]["Media"]["status"];
      expectedClass: string;
    }> = [
      {
        status: "not_started",
        expectedClass: "from-slate-600 via-slate-500 to-slate-400",
      },
      {
        status: "in_progress",
        expectedClass: "from-blue-700 via-blue-600 to-blue-500",
      },
      {
        status: "completed",
        expectedClass: "from-green-700 via-green-600 to-green-500",
      },
      {
        status: "paused",
        expectedClass: "from-amber-700 via-amber-600 to-amber-500",
      },
      {
        status: "abandoned",
        expectedClass: "from-red-700 via-red-600 to-red-500",
      },
    ];

    statuses.forEach(({ status, expectedClass }) => {
      it(`applies correct status accent for status "${status}"`, () => {
        const { container } = render(
          <Card media={{ ...mockMedia, status }} />,
        );

        const accentBar = container.querySelector(".bg-linear-to-r");
        expect(accentBar?.className).toContain(expectedClass);
      });
    });
  });
});

