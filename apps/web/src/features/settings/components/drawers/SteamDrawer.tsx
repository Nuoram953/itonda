import { useEffect } from "react";
import { Gamepad2, ExternalLink } from "lucide-react";
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

type SteamDrawerProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
};

export function SteamDrawer({ open, onOpenChange }: SteamDrawerProps) {
  const { data: config } = useConfig();
  const patchMutation = usePatchConfig();

  const steamSettings = config?.settings?.metadata?.steam;
  const steamSecrets = config?.secrets?.storefronts?.steam;

  const form = useForm({
    defaultValues: {
      enabled: steamSettings?.enabled ?? true,
      apiKey: steamSecrets?.api_key ?? "",
      steamId: steamSecrets?.steam_id ?? "",
      fetchPlaytime: steamSettings?.fetch_playtime ?? true,
      fetchAchievements: steamSettings?.fetch_achievements ?? true,
    },
    onSubmit: async ({ value }) => {
      await patchMutation.mutateAsync({
        settings: {
          metadata: {
            steam: {
              enabled: value.enabled,
              fetch_playtime: value.fetchPlaytime,
              fetch_achievements: value.fetchAchievements,
            },
          },
        },
        secrets: {
          storefronts: {
            steam: {
              api_key: value.apiKey,
              steam_id: value.steamId.trim(),
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
        enabled: config.settings?.metadata?.steam?.enabled ?? true,
        apiKey: config.secrets?.storefronts?.steam?.api_key ?? "",
        steamId: config.secrets?.storefronts?.steam?.steam_id ?? "",
        fetchPlaytime: config.settings?.metadata?.steam?.fetch_playtime ?? true,
        fetchAchievements:
          config.settings?.metadata?.steam?.fetch_achievements ?? true,
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
            <div className="flex items-center justify-center size-12 rounded-2xl bg-primary/10 border border-primary/20 text-primary shadow-inner">
              <Gamepad2 className="w-6 h-6" />
            </div>
            <div>
              <SheetTitle className="text-lg font-bold text-foreground">
                Steam Integration
              </SheetTitle>
              <SheetDescription className="text-xs text-text-muted mt-0.5">
                Configure your Steam Web API credentials and synchronization
                preferences.
              </SheetDescription>
            </div>
          </div>
        </SheetHeader>

        <form.Subscribe
          selector={(state) => ({ enabled: state.values.enabled })}
          children={({ enabled }) => (
            <div className="flex-1 overflow-y-auto p-6 space-y-5">
              <form.Field
                name="enabled"
                children={(field) => (
                  <SettingRow
                    label="Enable Steam Integration"
                    description="Allow Itonda to communicate with Steam and index owned games."
                  >
                    <Switch
                      checked={field.state.value}
                      onCheckedChange={(checked) => {
                        field.handleChange(checked);
                        triggerSave(true);
                      }}
                      aria-label="Toggle Steam Enabled"
                    />
                  </SettingRow>
                )}
              />

              <Separator className="bg-white/5" />

              <div className="space-y-3 pt-2">
                <h4 className="text-xs font-semibold uppercase tracking-wider text-text-muted/70">
                  Auth
                </h4>

                <form.Field
                  name="apiKey"
                  children={(field) => (
                    <SettingRow
                      label="Steam Web API Key"
                      description="Required to query storefront details, owned games, and player achievements."
                      layout="vertical"
                      htmlFor="drawer-steam-api-key"
                    >
                      <SecretInput
                        id="drawer-steam-api-key"
                        value={field.state.value}
                        onChange={(apiKey) => {
                          field.handleChange(apiKey);
                          triggerSave(false);
                        }}
                        placeholder="e.g. 4B8A9C123..."
                        portalUrl="https://steamcommunity.com/dev/apikey"
                        portalLabel="Get Steam API key"
                        disabled={!enabled}
                      />
                    </SettingRow>
                  )}
                />

                <form.Field
                  name="steamId"
                  validators={{
                    onChange: ({ value }) => {
                      if (value && !/^\d+$/.test(value.trim())) {
                        return "Steam ID must contain numbers only";
                      }
                      return undefined;
                    },
                  }}
                  children={(field) => (
                    <SettingRow
                      label="Steam ID (64-bit)"
                      description="Your 17-digit Steam community ID (e.g. 76561198000000000)."
                      layout="vertical"
                      htmlFor="drawer-steam-id"
                    >
                      <div className="space-y-1.5 w-full">
                        <Input
                          id="drawer-steam-id"
                          value={field.state.value}
                          onChange={(e) => {
                            field.handleChange(e.target.value);
                            triggerSave(false);
                          }}
                          placeholder="76561198..."
                          disabled={!enabled}
                          className="font-mono text-xs bg-surface/80 border-white/10 text-foreground focus-visible:border-primary/50"
                        />
                        {field.state.meta.errors?.length ? (
                          <p className="text-[11px] text-destructive">
                            {field.state.meta.errors.join(", ")}
                          </p>
                        ) : null}
                        <div className="flex items-center justify-between gap-2 text-xs">
                          <a
                            href="https://steamid.io/"
                            target="_blank"
                            rel="noopener noreferrer"
                            className="inline-flex items-center gap-1 text-primary hover:text-primary-hover hover:underline transition-colors cursor-pointer"
                          >
                            <span>Find your SteamID64</span>
                            <ExternalLink className="w-3 h-3" />
                          </a>
                        </div>
                      </div>
                    </SettingRow>
                  )}
                />
              </div>

              <Separator className="bg-white/5" />

              <div className="space-y-3 pt-2">
                <h4 className="text-xs font-semibold uppercase tracking-wider text-text-muted/70">
                  Sync Preferences
                </h4>

                <form.Field
                  name="fetchPlaytime"
                  children={(field) => (
                    <SettingRow
                      label="Sync Playtime & Last Played"
                      description="Update played duration and recent session timestamps."
                    >
                      <Switch
                        checked={field.state.value}
                        disabled={!enabled}
                        onCheckedChange={(checked) => {
                          field.handleChange(checked);
                          triggerSave(true);
                        }}
                        aria-label="Toggle Sync Playtime"
                      />
                    </SettingRow>
                  )}
                />

                <form.Field
                  name="fetchAchievements"
                  children={(field) => (
                    <SettingRow
                      label="Fetch Achievements"
                      description="Query unlocked achievements count for each game in library."
                    >
                      <Switch
                        checked={field.state.value}
                        disabled={!enabled}
                        onCheckedChange={(checked) => {
                          field.handleChange(checked);
                          triggerSave(true);
                        }}
                        aria-label="Toggle Fetch Achievements"
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


