import { useRef } from "react";
import {
  Play,
  Pause,
  SkipBack,
  SkipForward,
  Music,
  Volume2,
  Volume1,
  VolumeX,
} from "lucide-react";
import { cn } from "@/lib/utils";

type FloatingOstPlayerProps = {
  title: string;
  trackName?: string | null;
  isPlaying: boolean;
  onToggle: () => void;
  volume?: number;
  onVolumeChange?: (volume: number) => void;
  onClose?: () => void;
};

export function FloatingOstPlayer({
  title,
  trackName,
  isPlaying,
  onToggle,
  volume = 0.75,
  onVolumeChange,
}: FloatingOstPlayerProps) {
  const prevVolumeRef = useRef(volume > 0 ? volume : 0.75);

  const handleToggleMute = () => {
    if (!onVolumeChange) return;
    if (volume > 0) {
      prevVolumeRef.current = volume;
      onVolumeChange(0);
    } else {
      onVolumeChange(prevVolumeRef.current || 0.75);
    }
  };

  const handleSliderChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const val = parseFloat(e.target.value);
    if (val > 0) {
      prevVolumeRef.current = val;
    }
    onVolumeChange?.(val);
  };

  return (
    <div
      className={cn(
        "fixed bottom-6 right-6 z-50 flex items-center gap-3.5 px-4 py-3",
        "bg-surface/95 backdrop-blur-xl border border-white/15 rounded-2xl shadow-2xl ring-1 ring-black/40",
        "transition-all duration-300 animate-in fade-in slide-in-from-bottom-4 select-none"
      )}
    >
      {/* Album Artwork Square */}
      <div className="w-10 h-10 rounded-lg overflow-hidden bg-slate-900 border border-white/10 shrink-0 flex items-center justify-center relative">
        <div className="absolute inset-0 bg-linear-to-br from-amber-600/30 to-amber-950/80 flex items-center justify-center">
          <Music className="w-5 h-5 text-primary" />
        </div>
      </div>

      {/* Track Title & Subtitle */}
      <div className="flex flex-col max-w-40 sm:max-w-48">
        <span className="text-xs font-bold text-foreground truncate leading-tight">
          {title} OST
        </span>
        <span
          className="text-xs font-medium text-text-muted truncate mt-0.5"
          title={trackName || undefined}
        >
          {trackName || "Original Soundtrack"}
        </span>
      </div>

      {/* Animated Gold Soundwave Visualizer */}
      <div className="hidden sm:flex items-end gap-0.5 h-4 px-2">
        {[
          "h-2",
          "h-4",
          "h-3",
          "h-4",
          "h-2.5",
          "h-3.5",
          "h-2",
        ].map((heightClass, idx) => (
          <div
            key={idx}
            className={cn(
              "w-0.5 bg-primary rounded-full transition-all",
              isPlaying ? `${heightClass} animate-pulse` : "h-1 opacity-40"
            )}
          />
        ))}
      </div>

      {/* Player Action Buttons: Prev, Play/Pause, Next */}
      <div className="flex items-center gap-1.5 ml-1">
        <button
          type="button"
          className="p-1 text-text-muted hover:text-foreground transition-colors cursor-pointer"
          title="Previous Track"
        >
          <SkipBack className="w-3.5 h-3.5" />
        </button>

        <button
          type="button"
          onClick={onToggle}
          className="w-8 h-8 rounded-full bg-primary hover:bg-primary-hover text-primary-foreground flex items-center justify-center transition-transform active:scale-95 shadow-md shadow-primary/20 cursor-pointer"
          title={isPlaying ? "Pause OST" : "Play OST"}
        >
          {isPlaying ? (
            <Pause className="w-4 h-4 fill-current" />
          ) : (
            <Play className="w-4 h-4 fill-current ml-0.5" />
          )}
        </button>

        <button
          type="button"
          className="p-1 text-text-muted hover:text-foreground transition-colors cursor-pointer"
          title="Next Track"
        >
          <SkipForward className="w-3.5 h-3.5" />
        </button>
      </div>

      {/* Vertical Divider */}
      <div className="w-px h-5 bg-border/60" />

      {/* Volume Control Slider */}
      <div className="flex items-center gap-2">
        <button
          type="button"
          onClick={handleToggleMute}
          className="p-1 text-text-muted hover:text-foreground transition-colors cursor-pointer"
          title={volume === 0 ? "Unmute" : "Mute"}
          aria-label={volume === 0 ? "Unmute" : "Mute"}
        >
          {volume === 0 ? (
            <VolumeX className="w-4 h-4 text-text-muted" />
          ) : volume <= 0.5 ? (
            <Volume1 className="w-4 h-4 text-text-muted" />
          ) : (
            <Volume2 className="w-4 h-4 text-text-muted" />
          )}
        </button>

        <input
          type="range"
          min={0}
          max={1}
          step={0.01}
          value={volume}
          onChange={handleSliderChange}
          style={{
            background: `linear-gradient(to right, var(--primary) ${volume * 100}%, rgba(255, 255, 255, 0.2) ${volume * 100}%)`,
          }}
          className="w-16 sm:w-20 h-1.5 accent-primary cursor-pointer rounded-full appearance-none focus:outline-none focus-visible:ring-1 focus-visible:ring-primary"
          title={`Volume: ${Math.round(volume * 100)}%`}
          aria-label="Volume"
        />
      </div>
    </div>
  );
}
