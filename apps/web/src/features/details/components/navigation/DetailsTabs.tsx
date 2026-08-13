import { LayoutDashboard, Image, Info } from "lucide-react";
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { cn } from "@/lib/utils";

export type TabId =
  | "overview"
  | "achievements"
  | "reviews"
  | "gallery"
  | "details";

type DetailsTabsProps = {
  activeTab: TabId;
  onChange: (tab: TabId) => void;
};

export function DetailsTabs({ activeTab, onChange }: DetailsTabsProps) {
  const tabs: Array<{
    id: TabId;
    label: string;
    icon: typeof LayoutDashboard;
    badge?: number;
  }> = [
    { id: "overview", label: "Overview", icon: LayoutDashboard },
    {
      id: "gallery",
      label: "Gallery & Clips",
      icon: Image,
    },
    { id: "details", label: "Details", icon: Info },
  ];

  return (
    <div className="w-full max-w-7xl mx-auto px-6 py-4">
      <div className="rounded-2xl bg-surface-card/60 border border-white/5 p-2 px-3 shadow-lg">
        <Tabs
          value={activeTab}
          onValueChange={(value) => onChange(value as TabId)}
          className="w-full "
        >
          <TabsList
            variant="line"
            className="h-12 w-full text-accent-gold justify-start gap-2 bg-transparent p-0 no-scrollbar overflow-x-auto border-none"
          >
            {tabs.map((tab) => {
              const Icon = tab.icon;
              const isActive = activeTab === tab.id;

              return (
                <TabsTrigger
                  key={tab.id}
                  value={tab.id}
                  className={cn(
                    "group relative inline-flex items-center gap-2.5 px-5 py-2.5 text-xs sm:text-sm font-semibold transition-all duration-200 cursor-pointer select-none outline-none rounded-xl",
                    isActive
                      ? "text-accent-gold font-bold bg-transparent"
                      : "text-text-muted hover:text-white/90 hover:bg-white/5",
                  )}
                >
                  <Icon
                    className={cn(
                      "w-4 h-4 shrink-0 transition-transform duration-200 group-hover:scale-105",
                      isActive ? "text-accent-gold" : "text-text-muted",
                    )}
                  />
                  <span>{tab.label}</span>

                  {tab.badge != null && tab.badge > 0 && (
                    <span
                      className={cn(
                        "ml-1 px-2 py-0.5 text-xs rounded-full font-bold transition-colors duration-200",
                        isActive
                          ? "bg-accent-gold/20 text-accent-gold"
                          : "bg-surface-hover text-text-muted",
                      )}
                    >
                      {tab.badge}
                    </span>
                  )}

                  {isActive && (
                    <div className="absolute inset-x-3 bottom-0.5 h-0.5 bg-accent-gold rounded-full shadow-accent-gold/50 animate-in fade-in" />
                  )}
                </TabsTrigger>
              );
            })}
          </TabsList>
        </Tabs>
      </div>
    </div>
  );
}
