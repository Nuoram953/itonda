import type { components } from "@/api/generated.d";

type CardProps = {
  media: components["schemas"]["Media"];
};

export function Card({ media }: CardProps) {
  return (
    <article className="group w-56 overflow-hidden rounded-xl bg-surface shadow-sm transition hover:-translate-y-1 hover:shadow-lg">
      <div className="aspect-3/4 overflow-hidden">
        {/* <img */}
        {/*   src={cover} */}
        {/*   alt={name} */}
        {/*   className="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105" */}
        {/* /> */}
      </div>

      <div className="bg-surface-raised p-3">
        <h3 className="truncate font-medium">{media.title}</h3>
      </div>
    </article>
  );
}
