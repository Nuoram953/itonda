import { useState } from "react";
import { Film, Gamepad2, Loader2, Tv, Sparkles, Eye } from "lucide-react";
import type { components } from "@/api/generated.d";

type CardProps = {
  media: components["schemas"]["Media"];
};

export function Card({ media }: CardProps) {
  const poster = media.assets.find((asset) => asset.asset_type === "poster");
  const [loading, setLoading] = useState(true);

  function getStatusAccent(status: components["schemas"]["Media"]["status"]) {
    switch (status) {
      case "not_started":
        return "from-slate-600 via-slate-500 to-slate-400";
      case "in_progress":
        return "from-blue-700 via-blue-600 to-blue-500";
      case "completed":
        return "from-green-700 via-green-600 to-green-500";
      case "paused":
        return "from-amber-700 via-amber-600 to-amber-500";
      case "abandoned":
        return "from-red-700 via-red-600 to-red-500";
      default:
        return "from-slate-600 via-slate-500 to-slate-400";
    }
  }

  function getStatusLabel(status: components["schemas"]["Media"]["status"]) {
    switch (status) {
      case "not_started":
        return "Unplayed";
      case "in_progress":
        return "In Progress";
      case "completed":
        return "Completed";
      case "paused":
        return "Paused";
      case "abandoned":
        return "Dropped";
      default:
        return status;
    }
  }

  function renderMediaTypeIcon(type?: string) {
    switch (type) {
      case "game":
        return <Gamepad2 className="w-3 h-3 text-primary" />;
      case "movie":
        return <Film className="w-3 h-3 text-primary" />;
      case "tv_show":
        return <Tv className="w-3 h-3 text-primary" />;
      default:
        return <Sparkles className="w-3 h-3 text-primary" />;
    }
  }

  return (
    <article className="group relative flex flex-col w-60 overflow-hidden rounded-2xl bg-surface/80 border border-white/10 shadow-sm transition-all duration-300 hover:-translate-y-1.5 hover:border-primary/40 hover:shadow-2xl hover:shadow-primary/10 cursor-pointer">
      <div className="relative aspect-3/4 overflow-hidden bg-surface-raised/80">
        {poster ? (
          <>
            {loading && (
              <div className="absolute inset-0 flex items-center justify-center bg-surface/80 backdrop-blur-sm z-10">
                <Loader2 className="h-6 w-6 animate-spin text-primary" />
              </div>
            )}

            <img
              key={poster.id}
              src={`http://localhost:3005/api/v1/assets/${poster.id}`}
              alt={media.title}
              className={`
                h-full w-full object-cover
                transition-all duration-500 ease-out
                group-hover:scale-105
                ${loading ? "opacity-0" : "opacity-100"}
              `}
              onLoad={() => setLoading(false)}
              onError={() => setLoading(false)}
            />
          </>
        ) : (
          <div className="absolute inset-0 flex flex-col items-center justify-center p-4 text-center bg-gradient-to-br from-surface to-surface-raised">
            <div className="p-3 rounded-2xl bg-surface-hover/60 border border-white/10 text-text-muted mb-2">
              {renderMediaTypeIcon(media.media_type)}
            </div>
            <span className="text-xs font-medium text-text-muted/70 line-clamp-2">
              No Poster Available
            </span>
          </div>
        )}

        <div className="absolute inset-0 z-20 bg-linear-to-t from-background/90 via-background/40 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex items-center justify-center p-4">
          <div className="transform translate-y-2 group-hover:translate-y-0 transition-transform duration-300 flex items-center gap-2 px-3.5 py-2 rounded-xl bg-primary text-primary-foreground text-xs font-semibold shadow-lg shadow-primary/30">
            <Eye className="w-4 h-4" />
            <span>View Details</span>
          </div>
        </div>
      </div>

      <div className="relative bg-surface-raised/90 p-3.5 flex flex-col gap-1.5">
        <div
          className={`absolute inset-x-0 top-0 h-1 bg-linear-to-r ${getStatusAccent(media.status)}`}
        />

        <div className="flex items-center justify-between gap-2 pt-0.5">
          <h3 className="truncate font-semibold text-sm text-foreground group-hover:text-primary transition-colors">
            {media.title}
          </h3>
        </div>

        <div className="flex items-center justify-between text-xs text-text-muted">
          <span className="capitalize font-medium">
            {getStatusLabel(media.status)}
          </span>
        </div>
      </div>
    </article>
  );
}
