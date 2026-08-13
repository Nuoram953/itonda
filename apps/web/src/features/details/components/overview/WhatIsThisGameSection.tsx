import { Play, User, Globe, BookOpen, Shield, Castle } from "lucide-react";
import type { components } from "@/api/generated.d";
import { getAssetUrl } from "../../utils/asset-url";
import { getHeroTrailerAsset, findAssetByType } from "../../utils/media-assets";

type WhatIsThisGameSectionProps = {
  media: components["schemas"]["Media"];
};

export function WhatIsThisGameSection({ media }: WhatIsThisGameSectionProps) {
  const trailerAsset = getHeroTrailerAsset(media.assets);
  const backdropAsset = findAssetByType(media.assets, [
    "backdrop",
    "banner",
    "screenshot",
  ]);

  const visualUrl = trailerAsset
    ? getAssetUrl(trailerAsset.id)
    : backdropAsset
      ? getAssetUrl(backdropAsset.id)
      : null;

  const description =
    "Kingdom Come: Deliverance is an immersive open-world RPG set in medieval Bohemia. Experience a living world, realistic combat, and a gripping story where your choices matter.";

  const featurePills = [
    { label: "Singleplayer", icon: <User className="w-8 h-8" /> },
    { label: "Open World", icon: <Globe className="w-8 h-8" /> },
    { label: "Story Rich", icon: <BookOpen className="w-8 h-8" /> },
    { label: "Realistic", icon: <Shield className="w-8 h-8" /> },
    { label: "Medieval", icon: <Castle className="w-8 h-8" /> },
  ];

  return (
    <section className="rounded-2xl bg-surface-card/60 border border-white/5 p-6 md:p-7">
      <div className="grid grid-cols-1 lg:grid-cols-5 gap-7 items-stretch">
        <div className="lg:col-span-2 flex flex-col justify-between py-1">
          <div className="space-y-3">
            <h3 className="text-xl font-bold tracking-widest text-accent-gold uppercase">
              WHAT IS THIS GAME?
            </h3>
            <p className="text-base text-text-muted leading-relaxed font-normal">
              {description}
            </p>
          </div>

          <div className="grid grid-cols-5 gap-2 w-full pt-4 mt-auto">
            {featurePills.map((pill) => (
              <div
                key={pill.label}
                className="flex flex-col items-center justify-center gap-1.5 group cursor-pointer text-center"
              >
                <div className="w-10 h-10 flex items-center justify-center text-accent-gold group-hover:scale-105 group-hover:border-accent-gold/40 transition-all duration-200">
                  {pill.icon}
                </div>
                <span className="text-xs font-medium text-text-muted group-hover:text-white/90 transition-colors leading-tight">
                  {pill.label}
                </span>
              </div>
            ))}
          </div>
        </div>

        <div
          onClick={() => {}}
          className="lg:col-span-3 group relative aspect-video w-full rounded-xl overflow-hidden border border-white/10 bg-slate-900 cursor-pointer shadow-xl transition-all duration-300 hover:border-accent-gold/40"
        >
          {visualUrl ? (
            trailerAsset ? (
              <video
                src={visualUrl}
                muted
                loop
                autoPlay
                playsInline
                className="w-full h-full object-cover transition-transform duration-700 group-hover:scale-105"
              />
            ) : (
              <img
                src={visualUrl}
                alt={media.title}
                className="w-full h-full object-cover transition-transform duration-700 group-hover:scale-105"
              />
            )
          ) : (
            <div className="w-full h-full bg-slate-900" />
          )}

          <div className="absolute inset-0 bg-linear-to-t from-slate-950/80 via-slate-950/20 to-transparent" />

          <div className="absolute inset-0 flex items-center justify-center">
            <div className="w-12 h-12 rounded-full bg-slate-950/70 border border-white/20 backdrop-blur-md flex items-center justify-center text-white group-hover:scale-110 group-hover:bg-accent-gold group-hover:text-black transition-all duration-300 shadow-xl">
              <Play className="w-5 h-5 fill-current ml-0.5" />
            </div>
          </div>

          <div className="absolute bottom-3 left-4 text-xs font-medium text-white/90 drop-shadow-md">
            Gameplay Overview Trailer
          </div>

          <div className="absolute bottom-3 right-4 px-1.5 py-0.5 rounded text-xs font-bold bg-slate-950/80 text-white/80 border border-white/10">
            2:01
          </div>
        </div>
      </div>
    </section>
  );
}
