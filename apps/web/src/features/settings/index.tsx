import { useState } from "react";
import { Workspace } from "@/components/workspace/Workspace";
import { LoadingState } from "@/components/feedback/LoadingState";
import { SettingsSection } from "./components/sections/SettingsSection";
import { IntegrationRow } from "./components/rows/IntegrationRow";
import { useConfig } from "./api/get-config";
import { usePatchConfig } from "./api/patch-config";
import { useSettingsUrlFeedback } from "./hooks/use-settings-url-feedback";
import {
  SETTINGS_SECTIONS,
  getInitialDrawer,
  type DrawerId,
} from "./constants/integrations";

export const Settings = () => {
  useSettingsUrlFeedback();
  const [activeDrawer, setActiveDrawer] = useState<DrawerId | null>(getInitialDrawer);

  const { data: config, isPending } = useConfig();
  const patchMutation = usePatchConfig();

  if (isPending || !config) {
    return <LoadingState message="Loading settings..." />;
  }

  return (
    <Workspace>
      <Workspace.Header title="Settings" showBackBtn />

      <Workspace.Content className="p-6 max-w-5xl mx-auto w-full space-y-10 pb-16 animate-in fade-in duration-300">
        {SETTINGS_SECTIONS.map((section) => (
          <SettingsSection
            key={section.id}
            title={section.title}
            description={section.description}
          >
            {section.integrations.map((integration) => {
              const enabled = integration.isEnabled(config);
              const issueText = enabled
                ? integration.getIssue(config)
                : undefined;

              return (
                <IntegrationRow
                  key={integration.id}
                  title={integration.title}
                  mediaTypes={integration.mediaTypes}
                  description={integration.description}
                  icon={integration.icon}
                  iconBgClass={integration.iconBgClass}
                  enabled={enabled}
                  issueText={issueText}
                  onToggleEnabled={(checked) => {
                    patchMutation.mutate(integration.getTogglePayload(checked));
                  }}
                  onOpenSheet={() => setActiveDrawer(integration.id)}
                />
              );
            })}
          </SettingsSection>
        ))}

        {SETTINGS_SECTIONS.flatMap((s) => s.integrations).map(
          ({ id, drawer: Drawer }) => (
            <Drawer
              key={id}
              open={activeDrawer === id}
              onOpenChange={(open) => setActiveDrawer(open ? id : null)}
            />
          ),
        )}
      </Workspace.Content>
    </Workspace>
  );
};
