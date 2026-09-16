import { LayoutDashboard, Image, Info, MessageSquareQuote } from "lucide-react";
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
    { id: "reviews", label: "Reviews & Notes", icon: MessageSquareQuote },
    {
      id: "gallery",
      label: "Gallery & Clips",
      icon: Image,
    },
    { id: "details", label: "Details", icon: Info },
  ];

  return (
    <nav
      aria-label="Media navigation"
      className="w-full border-b border-white/10 bg-background/80 backdrop-blur-md sticky top-0 z-20"
    >
      <div className="max-w-7xl mx-auto px-6">
        <Tabs
          value={activeTab}
          onValueChange={(value) => onChange(value as TabId)}
          className="w-full"
        >
          <TabsList
            variant="line"
            className="h-13 sm:h-14 w-full text-accent-gold justify-start gap-1 sm:gap-2 bg-transparent p-0 no-scrollbar overflow-x-auto border-none"
          >
            {tabs.map((tab) => {
              const Icon = tab.icon;
              const isActive = activeTab === tab.id;

              return (
                <TabsTrigger
                  key={tab.id}
                  value={tab.id}
                  className={cn(
                    "group relative inline-flex flex-none items-center gap-2 px-3.5 sm:px-5 py-3 text-xs sm:text-sm font-semibold transition-all duration-200 cursor-pointer select-none outline-none rounded-xl",
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
                    <div className="absolute inset-x-2 -bottom-[1px] h-0.5 bg-accent-gold rounded-full shadow-[0_0_8px_rgba(212,163,89,0.5)] animate-in fade-in" />
                  )}
                </TabsTrigger>
              );
            })}
          </TabsList>
        </Tabs>
      </div>
    </nav>
  );
}
