import { useState } from "react";
import { Loader2 } from "lucide-react";
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

  return (
    <article className="group w-60 overflow-hidden rounded-xl bg-surface shadow-sm transition hover:-translate-y-1 hover:shadow-lg">
      <div className="relative aspect-3/4 overflow-hidden bg-surface-raised">
        {poster && (
          <>
            {loading && (
              <div className="absolute inset-0 flex items-center justify-center">
                <Loader2 className="h-6 w-6 animate-spin text-text-muted" />
              </div>
            )}

            <img
              key={poster.id}
              src={`http://localhost:3005/api/v1/assets/${poster.id}`}
              alt={media.title}
              className={`
                h-full w-full object-cover
                transition-all duration-300
                group-hover:scale-105
                ${loading ? "opacity-0" : "opacity-100"}
              `}
              onLoad={() => setLoading(false)}
              onError={() => setLoading(false)}
            />
          </>
        )}
      </div>

      <div className="relative bg-surface-raised p-3">
        <div
          className={`absolute inset-x-0 top-0 h-1 bg-linear-to-r ${getStatusAccent(media.status)}`}
        />

        <h3 className="truncate font-medium">{media.title}</h3>
      </div>
    </article>
  );
}
