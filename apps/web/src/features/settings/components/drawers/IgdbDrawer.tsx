import { useEffect } from "react";
import { Database, ExternalLink } from "lucide-react";
import { useForm } from "@tanstack/react-form";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
  SheetFooter,
} from "@/components/ui/sheet";
import { Switch } from "@/components/ui/switch";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { SettingRow } from "../cards/SettingRow";
import { SecretInput } from "../forms/SecretInput";
import { useConfig } from "../../api/get-config";
import { usePatchConfig } from "../../api/patch-config";
import { useAutoSave } from "../../hooks/use-auto-save";

type IgdbDrawerProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
};

export function IgdbDrawer({ open, onOpenChange }: IgdbDrawerProps) {
  const { data: config } = useConfig();
  const patchMutation = usePatchConfig();

  const igdbSettings = config?.settings?.metadata?.igdb;
  const igdbSecrets = config?.secrets?.metadata_store?.igdb;

  const form = useForm({
    defaultValues: {
      enabled: igdbSettings?.enabled ?? true,
      clientId: igdbSecrets?.client_id ?? "",
      clientSecret: igdbSecrets?.client_secret ?? "",
    },
    onSubmit: async ({ value }) => {
      await patchMutation.mutateAsync({
        settings: {
          metadata: {
            igdb: {
              enabled: value.enabled,
            },
          },
        },
        secrets: {
          metadata_store: {
            igdb: {
              client_id: value.clientId.trim(),
              client_secret: value.clientSecret.trim(),
            },
          },
        },
      });
    },
  });

  const { triggerSave } = useAutoSave(() => {
    form.handleSubmit();
  });

  useEffect(() => {
    if (config && !form.state.isDirty) {
      form.reset({
        enabled: config.settings?.metadata?.igdb?.enabled ?? true,
        clientId: config.secrets?.metadata_store?.igdb?.client_id ?? "",
        clientSecret: config.secrets?.metadata_store?.igdb?.client_secret ?? "",
      });
    }
  }, [config, form]);

  return (
    <Sheet
      open={open}
      onOpenChange={(nextOpen) => {
        if (!nextOpen) {
          triggerSave(true);
        }
        onOpenChange(nextOpen);
      }}
    >
      <SheetContent
        side="right"
        className="w-full data-[side=right]:sm:max-w-2xl data-[side=right]:lg:max-w-3xl bg-surface border-l border-white/10 p-0 flex flex-col justify-between overflow-hidden shadow-2xl"
      >
        <SheetHeader className="p-6 border-b border-white/10 bg-surface-raised/40">
          <div className="flex items-center gap-3.5">
            <div className="flex items-center justify-center size-12 rounded-2xl bg-purple-500/10 border border-purple-500/20 text-purple-400 shadow-inner">
              <Database className="w-6 h-6" />
            </div>
            <div>
              <SheetTitle className="text-lg font-bold text-foreground">
                Internet Game Database (IGDB) Integration
              </SheetTitle>
              <SheetDescription className="text-xs text-text-muted mt-0.5">
                Configure your Twitch Developer credentials to retrieve game
                summaries, release dates, genres, and developer details.
              </SheetDescription>
            </div>
          </div>
        </SheetHeader>

        <form.Subscribe
          selector={(state) => ({
            enabled: state.values.enabled,
          })}
          children={({ enabled }) => (
            <div className="flex-1 overflow-y-auto p-6 space-y-5">
              <SettingRow
                label="Enable IGDB Integration"
                description="Allow Itonda to retrieve game metadata, developers, genres, and release information from IGDB."
              >
                <form.Field
                  name="enabled"
                  children={(field) => (
                    <Switch
                      checked={field.state.value}
                      onCheckedChange={(checked) => {
                        field.handleChange(checked);
                        triggerSave(true);
                      }}
                      aria-label="Toggle IGDB Enabled"
                    />
                  )}
                />
              </SettingRow>

              <Separator className="bg-white/5" />

              <div className="space-y-4 pt-2">
                <h4 className="text-xs font-semibold uppercase tracking-wider text-text-muted/70">
                  Authentication & Credentials
                </h4>

                <form.Field
                  name="clientId"
                  children={(field) => (
                    <SettingRow
                      label="Twitch Client ID"
                      description="Client ID generated from your Twitch developer application."
                      layout="vertical"
                      htmlFor="drawer-igdb-client-id"
                    >
                      <div className="space-y-1.5 w-full">
                        <Input
                          id="drawer-igdb-client-id"
                          value={field.state.value}
                          onChange={(e) => {
                            field.handleChange(e.target.value);
                            triggerSave(false);
                          }}
                          placeholder="e.g. gp762nuuoqcoxypju8c569th9wz7q5"
                          disabled={!enabled}
                          className="font-mono text-xs bg-surface/80 border-white/10 text-foreground focus-visible:border-primary/50"
                        />
                        <div className="flex items-center justify-between gap-2 text-xs">
                          <a
                            href="https://dev.twitch.tv/console/apps"
                            target="_blank"
                            rel="noopener noreferrer"
                            className="inline-flex items-center gap-1 text-primary hover:text-primary-hover hover:underline transition-colors cursor-pointer"
                          >
                            <span>Twitch Developer Console</span>
                            <ExternalLink className="w-3 h-3" />
                          </a>
                        </div>
                      </div>
                    </SettingRow>
                  )}
                />

                <form.Field
                  name="clientSecret"
                  children={(field) => (
                    <SettingRow
                      label="Twitch Client Secret"
                      description="OAuth client secret generated from your Twitch developer application."
                      layout="vertical"
                      htmlFor="drawer-igdb-client-secret"
                    >
                      <SecretInput
                        id="drawer-igdb-client-secret"
                        value={field.state.value}
                        onChange={(secret) => {
                          field.handleChange(secret);
                          triggerSave(false);
                        }}
                        placeholder="e.g. 2b93k78a..."
                        portalUrl="https://dev.twitch.tv/console/apps"
                        portalLabel="Manage Twitch Applications"
                        disabled={!enabled}
                      />
                    </SettingRow>
                  )}
                />
              </div>
            </div>
          )}
        />

        <SheetFooter className="p-4 border-t border-white/10 bg-surface-raised/40 flex items-center justify-end gap-3">
          <Button
            type="button"
            variant="default"
            size="sm"
            onClick={() => {
              triggerSave(true);
              onOpenChange(false);
            }}
            className="text-xs px-4 cursor-pointer"
          >
            Done
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  );
}
