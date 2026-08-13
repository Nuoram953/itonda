import { Swords, Compass, MessageSquare, Castle } from "lucide-react";
import type { components } from "@/api/generated.d";
import { getAssetUrl } from "../../utils/asset-url";
import { getScreenshotAssets } from "../../utils/media-assets";

type HowDoesItPlaySectionProps = {
  media: components["schemas"]["Media"];
};

export function HowDoesItPlaySection({ media }: HowDoesItPlaySectionProps) {
  const screenshots = getScreenshotAssets(media.assets);

  // TODO: this should come from the api
  const pillars = [
    {
      id: "combat",
      icon: <Swords className="w-4 h-4" />,
      title: "Realistic Combat",
      description:
        "Master challenging swordplay, archery, and tactics in grounded medieval battles.",
      image: screenshots[0] ? getAssetUrl(screenshots[0].id) : null,
    },
    {
      id: "explore",
      icon: <Compass className="w-4 h-4" />,
      title: "Explore Bohemia",
      description:
        "Roam a vast, authentic world filled with towns, forests, countryside and secrets.",
      image: screenshots[1] ? getAssetUrl(screenshots[1].id) : null,
    },
    {
      id: "choices",
      icon: <MessageSquare className="w-4 h-4" />,
      title: "Choices & Consequences",
      description:
        "Dialogue, reputation and decisions shape how the world reacts to you.",
      image: screenshots[2] ? getAssetUrl(screenshots[2].id) : null,
    },
    {
      id: "life",
      icon: <Castle className="w-4 h-4" />,
      title: "Live a Medieval Life",
      description:
        "Craft, train, trade, read and survive like a true man of the 15th century.",
      image: screenshots[3] ? getAssetUrl(screenshots[3].id) : null,
    },
  ];

  return (
    <section className="rounded-2xl bg-surface-card/60 border border-white/5 p-6 md:p-7 space-y-3">
      <h3 className="text-xl font-bold tracking-widest text-accent-gold uppercase">
        HOW DOES IT PLAY?
      </h3>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        {pillars.map((pillar) => (
          <div
            key={pillar.id}
            className="group relative flex flex-col rounded-xl overflow-hidden bg-surface-card/60 border border-white/5 hover:border-accent-gold/30 transition-all duration-300 hover:-translate-y-0.5"
          >
            <div className="relative aspect-video w-full overflow-hidden bg-slate-900">
              {pillar.image ? (
                <img
                  src={pillar.image}
                  alt={pillar.title}
                  className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
                />
              ) : (
                <div className="w-full h-full bg-linear-to-br from-slate-900 to-slate-950" />
              )}
              <div className="absolute inset-0 bg-linear-to-t from-surface-card/90 via-transparent to-transparent" />
            </div>

            <div className="relative p-4 pt-1 space-y-1.5 flex-1 flex flex-col justify-between">
              <div className="-mt-7 mb-1.5 w-8 h-8 rounded-full border border-accent-gold flex items-center justify-center text-accent-gold shadow-md group-hover:bg-accent-gold group-hover:text-black transition-all duration-200">
                {pillar.icon}
              </div>

              <div className="space-y-1">
                <h4 className="text-sm font-bold text-white/90 group-hover:text-accent-gold transition-colors">
                  {pillar.title}
                </h4>
                <p className="text-xs text-text-muted leading-snug font-normal">
                  {pillar.description}
                </p>
              </div>
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}
