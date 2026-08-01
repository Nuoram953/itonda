import type { components } from "@/api/generated.d";

type CardProps = {
  media: components["schemas"]["Media"];
};

export function Card({ media }: CardProps) {
  const poster = media.assets.find((asset) => asset.asset_type === "poster");

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
        <div className="absolute inset-x-0 top-0 h-1 bg-linear-to-r from-green-700 via-success to-green-500" />
        <h3 className="truncate font-medium">{media.title}</h3>
      </div>
    </article>
  );
}
