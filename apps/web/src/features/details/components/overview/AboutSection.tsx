import { useLayoutEffect, useRef, useState } from "react";
import { ChevronDown, ChevronUp } from "lucide-react";
import type { components } from "@/api/generated.d";
import { cn } from "@/lib/utils";
import { GenresTagsCard } from "./GenresTagsCard";

type AboutSectionProps = {
  media: components["schemas"]["Media"];
};

export function AboutSection({ media }: AboutSectionProps) {
  const summary = media.summary?.trim() || null;
  const rawDescription = media.description?.trim() || null;
  const description = rawDescription !== summary ? rawDescription : null;

  const descriptionRef = useRef<HTMLParagraphElement>(null);
  const [expanded, setExpanded] = useState(false);
  const [isClamped, setIsClamped] = useState(false);

  useLayoutEffect(() => {
    const el = descriptionRef.current;
    if (!el || expanded) return;
    setIsClamped(el.scrollHeight > el.clientHeight + 1);
  }, [description, expanded]);

  const showSideCard =
    media.genres.length > 0 ||
    media.tags.length > 0 ||
    (media.details?.developers && media.details.developers.length > 0) ||
    (media.details?.publishers && media.details.publishers.length > 0);

  return (
    <section className="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <div
        className={cn(
          "rounded-2xl bg-surface/70 border border-white/10 p-6 sm:p-8 space-y-4",
          showSideCard ? "lg:col-span-2" : "col-span-full",
        )}
      >
        <h3 className="text-xl font-bold text-foreground tracking-tight">
          About This Game
        </h3>

        {!summary && !description ? (
          <p className="text-sm text-text-muted">
            No description available for this game yet.
          </p>
        ) : (
          <div className="space-y-4">
            {summary && (
              <p className="text-base text-white/90 leading-relaxed whitespace-pre-line">
                {summary}
              </p>
            )}

            {description && (
              <div className="space-y-2">
                <p
                  ref={descriptionRef}
                  className={cn(
                    "text-sm text-text-muted leading-relaxed whitespace-pre-line",
                    !expanded && "line-clamp-6",
                  )}
                >
                  {description}
                </p>

                {(isClamped || expanded) && (
                  <button
                    type="button"
                    onClick={() => setExpanded((prev) => !prev)}
                    className="inline-flex items-center gap-1 text-sm font-medium text-primary hover:underline cursor-pointer"
                  >
                    {expanded ? (
                      <>
                        <span>Show less</span>
                        <ChevronUp className="w-4 h-4" />
                      </>
                    ) : (
                      <>
                        <span>Read more</span>
                        <ChevronDown className="w-4 h-4" />
                      </>
                    )}
                  </button>
                )}
              </div>
            )}
          </div>
        )}
      </div>

      <GenresTagsCard media={media} />
    </section>
  );
}
