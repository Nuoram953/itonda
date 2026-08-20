import { useState } from "react";
import { Gamepad2 } from "lucide-react";
import { Workspace } from "@/components/workspace/Workspace";
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { LoadingState } from "@/components/feedback/LoadingState";
import { IntegrationCard } from "./components/cards/IntegrationCard";
import { SteamDrawer } from "./components/drawers/SteamDrawer";
import type { SettingsCategoryFilter } from "./types/settings";
import { useConfig } from "./api/get-config";
import { usePatchConfig } from "./api/patch-config";

const SETTINGS_CATEGORIES: Array<{
  id: SettingsCategoryFilter;
  label: string;
}> = [
  { id: "storefronts", label: "Storefronts" },
  { id: "assets", label: "Assets" },
  { id: "metadata", label: "Metadata" },
  { id: "preferences", label: "Preferences" },
];

export const Settings = () => {
  const [filter, setFilter] = useState<SettingsCategoryFilter>("storefronts");
  const [steamDrawerOpen, setSteamDrawerOpen] = useState(false);

  const { data: config, isPending } = useConfig();
  const patchMutation = usePatchConfig();

  if (isPending || !config) {
    return <LoadingState message="Loading settings..." />;
  }

  const showStorefronts = filter === "all" || filter === "storefronts";
  const steamEnabled = config.settings?.metadata?.steam?.enabled ?? true;

  return (
    <Workspace>
      <Workspace.Header title="Settings & Integrations" showBackBtn />

      <Workspace.Content className="p-6 max-w-7xl mx-auto w-full space-y-6">
        <Tabs
          value={filter}
          onValueChange={(val) => setFilter(val as SettingsCategoryFilter)}
          className="w-full space-y-6"
        >
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-white/5 pb-4">
            <TabsList className="bg-surface/60 border border-white/5 p-1 rounded-xl">
              {SETTINGS_CATEGORIES.map((cat) => (
                <TabsTrigger
                  key={cat.id}
                  value={cat.id}
                  className="rounded-lg px-3 py-1.5 text-xs font-medium cursor-pointer"
                >
                  {cat.label}
                </TabsTrigger>
              ))}
            </TabsList>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
            {showStorefronts && (
              <IntegrationCard
                title="Steam"
                category="Storefront"
                description="Automatically import owned games, playtime, achievements, and assets from your Steam account."
                icon={<Gamepad2 className="w-6 h-6" />}
                iconBgClass="bg-primary/10 text-primary border-primary/20"
                enabled={steamEnabled}
                onToggleEnabled={(enabled) => {
                  patchMutation.mutate({
                    settings: {
                      metadata: {
                        steam: { enabled },
                      },
                    },
                  });
                }}
                onManage={() => setSteamDrawerOpen(true)}
              />
            )}
          </div>

          <SteamDrawer
            open={steamDrawerOpen}
            onOpenChange={setSteamDrawerOpen}
          />
        </Tabs>
      </Workspace.Content>
    </Workspace>
  );
};
