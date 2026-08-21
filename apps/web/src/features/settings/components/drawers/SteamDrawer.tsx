import { useEffect } from "react";
import {
  Gamepad2,
  ExternalLink,
  CheckCircle2,
  LogIn,
  Unlink,
  User,
} from "lucide-react";
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
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { SettingRow } from "../cards/SettingRow";
import { SecretInput } from "../forms/SecretInput";
import { useConfig } from "../../api/get-config";
import { usePatchConfig } from "../../api/patch-config";
import { useAutoSave } from "../../hooks/use-auto-save";
import { useDisconnectSteam, useSteamAuthStatus } from "../../api/auth";
import { useNotification } from "@/hooks/use-notification";
import { useQueryClient } from "@tanstack/react-query";

type SteamDrawerProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
};

export function SteamDrawer({ open, onOpenChange }: SteamDrawerProps) {
  const { notify } = useNotification();
  const queryClient = useQueryClient();
  const { data: config } = useConfig();
  const patchMutation = usePatchConfig();
  const disconnectMutation = useDisconnectSteam();
  const { data: authStatus } = useSteamAuthStatus();

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

  // OpenID Popup Message listener
  useEffect(() => {
    const handleAuthMessage = (event: MessageEvent) => {
      if (event.data?.type === "STEAM_AUTH_SUCCESS" && event.data?.steamId) {
        form.setFieldValue("steamId", event.data.steamId);
        triggerSave(true);
        queryClient.invalidateQueries({
          queryKey: ["auth", "steam", "status"],
        });
        queryClient.invalidateQueries({ queryKey: ["config"] });
        notify.success({
          title: "Steam Connected",
          description: event.data.accountName
            ? `Welcome, ${event.data.accountName}!`
            : "Successfully authenticated with Steam.",
        });
      } else if (event.data?.type === "STEAM_AUTH_ERROR") {
        notify.error({
          title: "Steam Authentication Failed",
          description:
            event.data.error || "Could not verify Steam credentials.",
        });
      }
    };

    window.addEventListener("message", handleAuthMessage);
    return () => window.removeEventListener("message", handleAuthMessage);
  }, [form, notify, queryClient, triggerSave]);

  const handleSteamOpenIdLogin = () => {
    const width = 800;
    const height = 600;
    const left = window.screenX + (window.outerWidth - width) / 2;
    const top = window.screenY + (window.outerHeight - height) / 2;

    const serverUrl =
      import.meta.env.VITE_SERVER_URL ||
      `${window.location.protocol}//${window.location.hostname}:3005`;

    window.open(
      `${serverUrl}/auth/steam/login`,
      "steam_openid_login",
      `width=${width},height=${height},left=${left},top=${top},resizable=yes,scrollbars=yes`,
    );
  };

  const handleDisconnect = async () => {
    try {
      await disconnectMutation.mutateAsync();
      form.setFieldValue("steamId", "");
      triggerSave(true);
      notify.info({
        title: "Steam Disconnected",
        description: "Your Steam account has been unlinked.",
      });
    } catch (e: unknown) {
      const msg =
        e instanceof Error ? e.message : "Could not unlink Steam account.";
      notify.error({
        title: "Disconnect Failed",
        description: msg,
      });
    }
  };

  const accountName = authStatus?.account_name ?? steamSecrets?.account_name;
  const avatarUrl = authStatus?.avatar_url ?? steamSecrets?.avatar_url;

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
                Configure your Steam OpenID authentication and synchronization
                preferences.
              </SheetDescription>
            </div>
          </div>
        </SheetHeader>

        <form.Subscribe
          selector={(state) => ({
            enabled: state.values.enabled,
            steamId: state.values.steamId,
          })}
          children={({ enabled, steamId }) => {
            const isConnected = !!steamId && steamId !== "0";

            return (
              <div className="flex-1 overflow-y-auto p-6 space-y-5">
                <SettingRow
                  label="Enable Steam Integration"
                  description="Allow Itonda to communicate with Steam and index owned games."
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
                        aria-label="Toggle Steam Enabled"
                      />
                    )}
                  />
                </SettingRow>

                <Separator className="bg-white/5" />

                {/* Steam OpenID Authentication & Profile Card */}
                <div className="rounded-xl border border-white/10 bg-surface-raised/40 p-4 space-y-3">
                  {isConnected ? (
                    <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                      <div className="flex items-center gap-3.5">
                        {avatarUrl ? (
                          <img
                            src={avatarUrl}
                            alt={accountName || "Steam Avatar"}
                            className="size-12 rounded-full border-2 border-primary/30 object-cover shadow-md"
                          />
                        ) : (
                          <div className="size-12 rounded-full bg-primary/10 border border-primary/20 flex items-center justify-center text-primary shadow-inner">
                            <User className="w-6 h-6" />
                          </div>
                        )}
                        <div className="space-y-0.5">
                          <div className="flex items-center gap-2">
                            <span className="text-sm font-bold text-foreground">
                              {accountName || "Steam User"}
                            </span>
                          </div>
                          <div className="flex items-center gap-2 text-xs text-text-muted">
                            <span className="font-mono text-[11px]">
                              {steamId}
                            </span>
                            <span>•</span>
                            <a
                              href={`https://steamcommunity.com/profiles/${steamId}`}
                              target="_blank"
                              rel="noopener noreferrer"
                              className="inline-flex items-center gap-0.5 text-primary hover:text-primary-hover hover:underline transition-colors cursor-pointer"
                            >
                              <span>Profile</span>
                              <ExternalLink className="w-2.5 h-2.5" />
                            </a>
                          </div>
                        </div>
                      </div>

                      <Button
                        type="button"
                        variant="outline"
                        size="sm"
                        onClick={handleDisconnect}
                        disabled={!enabled || disconnectMutation.isPending}
                        className="text-xs gap-1.5 border-destructive/30 text-destructive hover:bg-destructive/10 cursor-pointer self-start sm:self-auto"
                      >
                        <Unlink className="w-3.5 h-3.5" />
                        <span>Disconnect</span>
                      </Button>
                    </div>
                  ) : (
                    <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                      <div>
                        <div className="flex items-center gap-2">
                          <h4 className="text-sm font-semibold text-foreground">
                            Steam OpenID Authentication
                          </h4>
                        </div>
                        <p className="text-xs text-text-muted mt-0.5">
                          Authenticate securely via Steam to automatically
                          detect your SteamID and profile.
                        </p>
                      </div>

                      <Button
                        type="button"
                        variant="default"
                        size="sm"
                        onClick={handleSteamOpenIdLogin}
                        disabled={!enabled}
                        className="text-xs gap-1.5 shadow-md cursor-pointer bg-[#171a21] hover:bg-[#2a475e] text-white border border-white/10 whitespace-nowrap self-start sm:self-auto"
                      >
                        <LogIn className="w-3.5 h-3.5" />
                        <span>Sign in with Steam</span>
                      </Button>
                    </div>
                  )}
                </div>

                <div className="space-y-3 pt-2">
                  <h4 className="text-xs font-semibold uppercase tracking-wider text-text-muted/70">
                    Credentials & Advanced Config
                  </h4>

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
                        description="Your 17-digit Steam community ID (automatically filled via OpenID)."
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

                  <form.Field
                    name="apiKey"
                    children={(field) => (
                      <SettingRow
                        label="Steam Web API Key"
                        description="Optional key for querying private libraries and extended playtime/achievements data."
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
            );
          }}
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
