import type { components } from "@/api/generated.d";

type CardProps = {
  media: components["schemas"]["Media"];
};

export function Card({ media }: CardProps) {
  const poster = media.assets.find((asset) => asset.asset_type === "poster");

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
      <div className="aspect-3/4 overflow-hidden">
        {poster && (
          <img
            src={`http://localhost:3005/api/v1/assets/${media.assets[0].id}`}
            className="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
          />
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
