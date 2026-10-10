import type { components } from "@/api/generated.d";
import { cn } from "@/lib/utils";

type GenresTagsCardProps = {
  media: components["schemas"]["Media"];
};

type ChipGroupProps = {
  label: string;
  items: string[];
  emphasized?: boolean;
};

function ChipGroup({ label, items, emphasized }: ChipGroupProps) {
  return (
    <div className="space-y-2">
      <h4 className="text-xs font-semibold uppercase tracking-wider text-text-muted">
        {label}
      </h4>
      <ul className="flex flex-wrap gap-1.5">
        {items.map((item) => (
          <li
            key={item}
            className={cn(
              "px-2.5 py-1 rounded-md text-xs font-medium border",
              emphasized
                ? "bg-surface-raised border-white/15 text-foreground"
                : "bg-surface-hover border-white/10 text-text-muted",
            )}
          >
            {item}
          </li>
        ))}
      </ul>
    </div>
  );
}

function hasGenresOrTags(media: GenresTagsCardProps["media"]) {
  return (
    media.genres.length > 0 ||
    media.tags.length > 0 ||
    (media.details?.developers && media.details.developers.length > 0) ||
    (media.details?.publishers && media.details.publishers.length > 0)
  );
}

export function GenresTagsCard({ media }: GenresTagsCardProps) {
  if (!hasGenresOrTags(media)) return null;

  const developers = media.details?.developers ?? [];
  const publishers = media.details?.publishers ?? [];

  return (
    <div className="rounded-2xl bg-surface/70 border border-white/10 p-6 space-y-5 h-fit">
      <h3 className="text-lg font-bold text-foreground tracking-tight">
        Genres &amp; Tags
      </h3>
      {media.genres.length > 0 && (
        <ChipGroup label="Genres" items={media.genres} emphasized />
      )}
      {media.tags.length > 0 && <ChipGroup label="Tags" items={media.tags} />}
      {developers.length > 0 && (
        <ChipGroup label="Developers" items={developers} />
      )}
      {publishers.length > 0 && (
        <ChipGroup label="Publishers" items={publishers} />
      )}
    </div>
  );
}
