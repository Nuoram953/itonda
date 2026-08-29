import type { ReactNode } from "react";
import {
  Swords,
  Compass,
  MessageSquare,
  Castle,
  Shield,
  Hammer,
  Eye,
  Users,
  Award,
  Puzzle,
  Gamepad2,
  Crosshair,
  Sparkles,
} from "lucide-react";
import type { components } from "@/api/generated.d";
import { getAssetUrl } from "../../utils/asset-url";
import { getScreenshotAssets } from "../../utils/media-assets";

type HowDoesItPlaySectionProps = {
  media: components["schemas"]["Media"];
};

const ICON_MAP: Record<string, ReactNode> = {
  combat: <Swords className="w-4 h-4" />,
  swords: <Swords className="w-4 h-4" />,
  target: <Crosshair className="w-4 h-4" />,
  explore: <Compass className="w-4 h-4" />,
  compass: <Compass className="w-4 h-4" />,
  choices: <MessageSquare className="w-4 h-4" />,
  story: <MessageSquare className="w-4 h-4" />,
  survival: <Shield className="w-4 h-4" />,
  shield: <Shield className="w-4 h-4" />,
  life: <Castle className="w-4 h-4" />,
  crafting: <Hammer className="w-4 h-4" />,
  stealth: <Eye className="w-4 h-4" />,
  coop: <Users className="w-4 h-4" />,
  multiplayer: <Users className="w-4 h-4" />,
  progression: <Award className="w-4 h-4" />,
  puzzle: <Puzzle className="w-4 h-4" />,
  default: <Gamepad2 className="w-4 h-4" />,
};

function getPillarIcon(iconKey?: string | null): ReactNode {
  if (!iconKey) {
    return <Sparkles className="w-4 h-4" />;
  }
  return ICON_MAP[iconKey.toLowerCase()] ?? <Sparkles className="w-4 h-4" />;
}

export function HowDoesItPlaySection({ media }: HowDoesItPlaySectionProps) {
  const screenshots = getScreenshotAssets(media.assets);
  const rawPillars = media.details?.pillars;

  if (!rawPillars || rawPillars.length === 0) {
    return null;
  }

  const pillars = rawPillars.map((pillar, index) => {
    const imageUrl = pillar.asset_id
      ? getAssetUrl(pillar.asset_id)
      : screenshots[index]
        ? getAssetUrl(screenshots[index].id)
        : null;

    return {
      id: pillar.id || `pillar-${index}`,
      icon: getPillarIcon(pillar.icon),
      title: pillar.title,
      description: pillar.description,
      image: imageUrl,
    };
  });

  return (
    <section className="p-6 md:p-7 space-y-3">
      <h3 className="text-xl font-bold tracking-widest text-accent-gold uppercase">
        HOW DOES IT PLAY?
      </h3>

      <div className="flex flex-1 gap-4">
        {pillars.map((pillar) => (
          <div
            key={pillar.id}
            className="group flex-1 flex flex-col rounded-xl overflow-hidden bg-surface-card/60 border border-white/5 hover:border-accent-gold/30 transition-all duration-300 hover:-translate-y-0.5"
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

              <div className="absolute bottom-3 left-4 w-8 h-8 rounded-full bg-slate-950/80 backdrop-blur-xs border border-accent-gold flex items-center justify-center text-accent-gold shadow-md group-hover:bg-accent-gold group-hover:text-black transition-all duration-200">
                {pillar.icon}
              </div>
            </div>

            <div className="p-4 pt-3.5 space-y-1 flex-1 flex flex-col">
              <h4 className="text-sm font-bold text-white/90 group-hover:text-accent-gold transition-colors">
                {pillar.title}
              </h4>
              <p className="text-xs text-text-muted leading-snug font-normal">
                {pillar.description}
              </p>
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}
