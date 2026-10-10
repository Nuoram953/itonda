import { Timer, Compass, Trophy, Layers } from "lucide-react";
// import type { components } from "@/api/generated.d";
import { cn } from "@/lib/utils";
import { formatPlaytime } from "@/utils/datetime";

// type GameLengthSectionProps = {
//   media: components["schemas"]["Media"];
// };

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

export function GameLengthSection() {
  const metrics: MetricCard[] = [
    {
      id: "main_story",
      label: "Main Story",
      hours: 25,
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
      hours: 33,
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
      hours: 65,
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
      hours: 99,
      description: "Average of all playstyles",
      icon: Layers,
      accent: {
        badge: "bg-purple-500/10 text-purple-400 border-purple-500/20",
        text: "text-purple-400",
        border: "hover:border-purple-500/30",
      },
    },
  ];

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
                  {/* temporary. Need to be replaced by real values */}
                  {formatPlaytime((metric.hours ?? 20) * 60)}
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
