import { Play, Pause, SkipBack, SkipForward, Music } from "lucide-react";
import { cn } from "@/lib/utils";

type FloatingOstPlayerProps = {
  title: string;
  isPlaying: boolean;
  onToggle: () => void;
  onClose?: () => void;
};

export function FloatingOstPlayer({
  title,
  isPlaying,
  onToggle,
}: FloatingOstPlayerProps) {
  return (
    <div
      className={cn(
        "fixed bottom-6 right-6 z-50 flex items-center gap-4 px-4 py-3",
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
        <span className="text-xs font-medium text-text-muted truncate mt-0.5">
          A World Unforgiving
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
    </div>
  );
}
