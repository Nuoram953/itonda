import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@/test/test-utils";
import { FloatingOstPlayer } from "../FloatingOstPlayer";

describe("FloatingOstPlayer Component", () => {
  it("renders the track title and trackName when provided", () => {
    render(
      <FloatingOstPlayer
        title="Elden Ring"
        trackName="The Final Battle"
        isPlaying={true}
        onToggle={vi.fn()}
      />
    );

    expect(screen.getByText("Elden Ring OST")).toBeDefined();
    expect(screen.getByText("The Final Battle")).toBeDefined();
  });

  it("renders default fallback when trackName is not provided", () => {
    render(
      <FloatingOstPlayer
        title="Elden Ring"
        isPlaying={true}
        onToggle={vi.fn()}
      />
    );

    expect(screen.getByText("Elden Ring OST")).toBeDefined();
    expect(screen.getByText("Original Soundtrack")).toBeDefined();
  });

  it("calls onToggle when the play/pause button is clicked", () => {
    const handleToggle = vi.fn();
    render(
      <FloatingOstPlayer
        title="Elden Ring"
        isPlaying={false}
        onToggle={handleToggle}
      />
    );

    const playBtn = screen.getByTitle("Play OST");
    fireEvent.click(playBtn);
    expect(handleToggle).toHaveBeenCalledTimes(1);
  });

  it("renders a volume slider initialized with the provided volume", () => {
    render(
      <FloatingOstPlayer
        title="Elden Ring"
        isPlaying={true}
        onToggle={vi.fn()}
        volume={0.75}
      />
    );

    const slider = screen.getByRole("slider", { name: "Volume" }) as HTMLInputElement;
    expect(slider).toBeDefined();
    expect(Number(slider.value)).toBe(0.75);
    expect(slider.title).toBe("Volume: 75%");
  });

  it("calls onVolumeChange when the slider value changes", () => {
    const handleVolumeChange = vi.fn();
    render(
      <FloatingOstPlayer
        title="Elden Ring"
        isPlaying={true}
        onToggle={vi.fn()}
        volume={0.75}
        onVolumeChange={handleVolumeChange}
      />
    );

    const slider = screen.getByRole("slider", { name: "Volume" });
    fireEvent.change(slider, { target: { value: "0.4" } });

    expect(handleVolumeChange).toHaveBeenCalledWith(0.4);
  });

  it("mutes and restores volume when clicking the mute toggle button", () => {
    const handleVolumeChange = vi.fn();
    const { rerender } = render(
      <FloatingOstPlayer
        title="Elden Ring"
        isPlaying={true}
        onToggle={vi.fn()}
        volume={0.75}
        onVolumeChange={handleVolumeChange}
      />
    );

    // Initial state: volume > 0, button is "Mute"
    const muteBtn = screen.getByRole("button", { name: "Mute" });
    fireEvent.click(muteBtn);

    expect(handleVolumeChange).toHaveBeenCalledWith(0);

    // Rerender with volume = 0
    rerender(
      <FloatingOstPlayer
        title="Elden Ring"
        isPlaying={true}
        onToggle={vi.fn()}
        volume={0}
        onVolumeChange={handleVolumeChange}
      />
    );

    // Now button should be "Unmute"
    const unmuteBtn = screen.getByRole("button", { name: "Unmute" });
    fireEvent.click(unmuteBtn);

    expect(handleVolumeChange).toHaveBeenCalledWith(0.75);
  });
});
