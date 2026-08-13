import { ArrowRight, ChevronRight } from "lucide-react";
import type { components } from "@/api/generated.d";
import type { TabId } from "../navigation/DetailsTabs";
import { getAssetUrl } from "../../utils/asset-url";
import {
  getScreenshotAssets,
  getTrailerAssets,
} from "../../utils/media-assets";

type SeeItInActionSectionProps = {
  media: components["schemas"]["Media"];
  onNavigateTab: (tab: TabId) => void;
};

export function SeeItInActionSection({
  media,
  onNavigateTab,
}: SeeItInActionSectionProps) {
  const screenshots = getScreenshotAssets(media.assets);
  const trailers = getTrailerAssets(media.assets);

  //TODO: This should come from the api
  const videoCards = [
    { title: "Gameplay Walkthrough", duration: "1:54" },
    { title: "Combat & Mechanics", duration: "1:36" },
    { title: "Story Cinematic", duration: "2:08" },
    { title: "World Exploration", duration: "1:47" },
    { title: "Siege of Talmberg", duration: "2:22" },
  ];

  return (
    <section className="rounded-2xl bg-surface-card/60 border border-white/5 p-6 md:p-7 space-y-3">
      <div className="flex items-center justify-between">
        <h3 className="text-xl font-bold tracking-widest text-accent-gold uppercase">
          SEE IT IN ACTION
        </h3>

        <button
          type="button"
          onClick={() => onNavigateTab("gallery")}
          className="group inline-flex items-center gap-1.5 text-xs font-medium text-text-muted hover:text-accent-gold transition-colors cursor-pointer"
        >
          <span>View all videos</span>
          <ArrowRight className="w-3.5 h-3.5 transition-transform group-hover:translate-x-1" />
        </button>
      </div>

      <div className="relative group/reel">
        <div className="grid grid-cols-1 sm:grid-cols-3 lg:grid-cols-5 gap-3.5">
          {videoCards.map((card, idx) => {
            const asset = screenshots[idx % screenshots.length] || trailers[0];
            const imgUrl = asset ? getAssetUrl(asset.id) : null;

            return (
              <div
                key={card.title}
                onClick={() => {}}
                className="group/card cursor-pointer space-y-1.5"
              >
                <div className="relative aspect-video w-full rounded-lg overflow-hidden bg-slate-900 border border-white/10 group-hover/card:border-accent-gold/30 transition-all duration-300 group-hover/card:-translate-y-0.5 shadow-md">
                  {imgUrl ? (
                    <img
                      src={imgUrl}
                      alt={card.title}
                      className="w-full h-full object-cover transition-transform duration-500 group-hover/card:scale-105"
                    />
                  ) : (
                    <div className="w-full h-full bg-slate-900" />
                  )}

                  <div className="absolute inset-0 bg-slate-950/20 group-hover/card:bg-transparent transition-colors" />

                  <span className="absolute bottom-1.5 right-1.5 px-1.5 py-0.5 rounded text-xs font-bold bg-slate-950/80 text-white/80 border border-white/10">
                    {card.duration}
                  </span>
                </div>

                <h4 className="text-xs font-medium text-text-muted group-hover/card:text-white/90 transition-colors line-clamp-1">
                  {card.title}
                </h4>
              </div>
            );
          })}
        </div>

        <div className="absolute -right-3 top-1/2 -translate-y-1/2 w-7 h-7 rounded-full bg-slate-900/90 border border-white/20 flex items-center justify-center text-text-muted hover:text-white shadow-xl opacity-0 group-hover/reel:opacity-100 transition-opacity cursor-pointer">
          <ChevronRight className="w-4 h-4" />
        </div>
      </div>
    </section>
  );
}
