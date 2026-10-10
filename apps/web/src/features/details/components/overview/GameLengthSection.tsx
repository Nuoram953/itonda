import { Timer, Compass, Trophy, Layers } from "lucide-react";
import type { components } from "@/api/generated.d";
import { cn } from "@/lib/utils";
import { formatPlaytime } from "@/utils/datetime";
import {
  formatHltbHours,
  getMediaHltbData,
} from "../../utils/how-long-to-beat";
import type { HowLongToBeatData } from "../../types/how-long-to-beat";

type GameLengthSectionProps = {
  media: components["schemas"]["Media"];
  customHltbData?: HowLongToBeatData | null;
};

type MetricCard = {
  id: string;
  label: string;
  hours: number | null;
  description: string;
  icon: typeof Timer;
  accent: {
    badge: string;
    text: string;
    border: string;
  };
};

export function GameLengthSection({
  media,
  customHltbData,
}: GameLengthSectionProps) {
  const hltb =
    customHltbData !== undefined
      ? customHltbData
        ? getMediaHltbData({
            ...media,
            ...({ how_long_to_beat: customHltbData } as unknown as object),
          })
        : null
      : getMediaHltbData(media);

  if (!hltb) return null;

  const metrics: MetricCard[] = [
    {
      id: "main_story",
      label: "Main Story",
      hours: hltb.mainStory,
      description: "Core objectives & story",
      icon: Timer,
      accent: {
        badge: "bg-sky-500/10 text-sky-400 border-sky-500/20",
        text: "text-sky-400",
        border: "hover:border-sky-500/30",
      },
    },
    {
      id: "main_extra",
      label: "Main + Extra",
      hours: hltb.mainExtra,
      description: "Story plus side quests",
      icon: Compass,
      accent: {
        badge: "bg-emerald-500/10 text-emerald-400 border-emerald-500/20",
        text: "text-emerald-400",
        border: "hover:border-emerald-500/30",
      },
    },
    {
      id: "completionist",
      label: "Completionist",
      hours: hltb.completionist,
      description: "100% completion & all extras",
      icon: Trophy,
      accent: {
        badge: "bg-amber-500/10 text-amber-400 border-amber-500/20",
        text: "text-amber-400",
        border: "hover:border-amber-500/30",
      },
    },
    {
      id: "all_styles",
      label: "All Styles",
      hours: hltb.allStyles,
      description: "Average of all playstyles",
      icon: Layers,
      accent: {
        badge: "bg-purple-500/10 text-purple-400 border-purple-500/20",
        text: "text-purple-400",
        border: "hover:border-purple-500/30",
      },
    },
  ];

  const playtimeMinutes = media.details?.playtime_minutes ?? 0;
  const mainStoryHours = hltb.mainStory ?? 0;
  const currentHours = playtimeMinutes / 60;
  const progressPercent =
    mainStoryHours > 0
      ? Math.min(100, Math.round((currentHours / mainStoryHours) * 100))
      : 0;

  return (
    <section data-testid="game-length-section">
      <div className="border-b border-white/10 pb-4">
        <h2 className="text-2xl font-extrabold text-foreground tracking-tight">
          Game Length
        </h2>
        <p className="text-sm text-text-muted mt-1">
          Average completion estimates from the community.
        </p>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        {metrics.map((metric) => {
          const Icon = metric.icon;
          return (
            <div
              key={metric.id}
              className={cn(
                "group relative rounded-xl bg-background/50 p-4 sm:p-5 flex flex-col justify-between transition-all duration-200",
                metric.accent.border,
              )}
            >
              <div className="flex items-center gap-2 mb-3">
                <div
                  className={cn(
                    "p-1.5 rounded-lg border shrink-0",
                    metric.accent.badge,
                  )}
                >
                  <Icon className="w-3.5 h-3.5" />
                </div>
                <span className="text-xs font-bold uppercase tracking-wider text-text-muted">
                  {metric.label}
                </span>
              </div>

              <div className="space-y-1">
                <div className="text-2xl sm:text-3xl font-extrabold text-foreground tracking-tight">
                  {formatHltbHours(metric.hours)}
                </div>
                <p className="text-xs text-text-muted leading-relaxed">
                  {metric.description}
                </p>
              </div>
            </div>
          );
        })}
      </div>
    </section>
  );
}
