import { useCallback, useEffect, useRef, useState } from "react";
import { ArrowRight, ChevronLeft, ChevronRight } from "lucide-react";
import type { components } from "@/api/generated.d";
import { getAssetUrl } from "../../utils/asset-url";
import { getAssetsByType } from "../../utils/media-assets";

const SCREENSHOTS_PER_PAGE = 8;

type ScreenshotCarouselProps = {
  media: components["schemas"]["Media"];
  onViewGallery: () => void;
};

const navButtonClass =
  "p-2 rounded-full bg-surface/70 border border-white/10 text-text-muted hover:text-foreground hover:bg-white/10 transition-colors cursor-pointer disabled:opacity-30 disabled:pointer-events-none";

export function ScreenshotCarousel({
  media,
  onViewGallery,
}: ScreenshotCarouselProps) {
  const screenshots = getAssetsByType(media.assets, "screenshot");
  const trackRef = useRef<HTMLDivElement>(null);
  const [canScrollPrev, setCanScrollPrev] = useState(false);
  const [canScrollNext, setCanScrollNext] = useState(false);

  const updateScrollState = useCallback(() => {
    const el = trackRef.current;
    if (!el) return;
    setCanScrollPrev(el.scrollLeft > 1);
    setCanScrollNext(el.scrollLeft + el.clientWidth < el.scrollWidth - 1);
  }, []);

  useEffect(() => {
    updateScrollState();
    window.addEventListener("resize", updateScrollState);
    return () => window.removeEventListener("resize", updateScrollState);
  }, [updateScrollState, screenshots.length]);

  const scrollByPage = (direction: 1 | -1) => {
    const el = trackRef.current;
    if (!el) return;
    el.scrollBy({ left: direction * el.clientWidth, behavior: "smooth" });
  };

  if (screenshots.length === 0) return null;

  const pages: (typeof screenshots)[] = [];
  for (let i = 0; i < screenshots.length; i += SCREENSHOTS_PER_PAGE) {
    pages.push(screenshots.slice(i, i + SCREENSHOTS_PER_PAGE));
  }

  return (
    <section className="space-y-4">
      <div className="flex items-center justify-between gap-4">
        <h3 className="text-xl font-bold text-foreground tracking-tight flex items-center gap-2.5">
          <span>Screenshots</span>
          <span className="text-xs font-semibold px-2.5 py-0.5 rounded-full bg-surface/80 border border-white/10 text-text-muted">
            {screenshots.length}
          </span>
        </h3>

        <div className="flex items-center gap-2">
          <button
            type="button"
            onClick={onViewGallery}
            className="group inline-flex items-center gap-1.5 text-sm font-medium text-text-muted hover:text-foreground transition-colors cursor-pointer mr-2"
          >
            <span>View all</span>
            <ArrowRight className="w-4 h-4 transition-transform group-hover:translate-x-1" />
          </button>
          <button
            type="button"
            aria-label="Previous screenshots"
            onClick={() => scrollByPage(-1)}
            disabled={!canScrollPrev}
            className={navButtonClass}
          >
            <ChevronLeft className="w-4 h-4" />
          </button>
          <button
            type="button"
            aria-label="Next screenshots"
            onClick={() => scrollByPage(1)}
            disabled={!canScrollNext}
            className={navButtonClass}
          >
            <ChevronRight className="w-4 h-4" />
          </button>
        </div>
      </div>

      <div
        ref={trackRef}
        onScroll={updateScrollState}
        data-testid="screenshot-track"
        className="flex overflow-x-auto snap-x snap-mandatory scroll-smooth pb-2 [scrollbar-width:none] [&::-webkit-scrollbar]:hidden"
      >
        {pages.map((page, pageIdx) => (
          <div
            key={page[0].id}
            data-testid="screenshot-page"
            className="w-full shrink-0 snap-start grid grid-cols-2 lg:grid-cols-4 gap-3 sm:gap-4"
          >
            {page.map((shot, idx) => {
              const globalIdx = pageIdx * SCREENSHOTS_PER_PAGE + idx;
              return (
                <div
                  key={shot.id}
                  className="aspect-video rounded-xl overflow-hidden bg-slate-900 border border-white/10"
                >
                  <img
                    src={getAssetUrl(shot.id)}
                    alt={`${media.title} screenshot ${globalIdx + 1}`}
                    loading="lazy"
                    className="w-full h-full object-cover"
                  />
                </div>
              );
            })}
          </div>
        ))}
      </div>
    </section>
  );
}
