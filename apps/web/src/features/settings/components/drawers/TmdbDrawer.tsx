import { useEffect } from "react";
import { Film } from "lucide-react";
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
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { SettingRow } from "../cards/SettingRow";
import { SecretInput } from "../forms/SecretInput";
import { useConfig } from "../../api/get-config";
import { usePatchConfig } from "../../api/patch-config";
import { useAutoSave } from "../../hooks/use-auto-save";

type TmdbDrawerProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
};

export function TmdbDrawer({ open, onOpenChange }: TmdbDrawerProps) {
  const { data: config } = useConfig();
  const patchMutation = usePatchConfig();

  const tmdbSettings = config?.settings?.assets?.tmdb;
  const tmdbSecrets = config?.secrets?.asset_store?.tmdb;

  const form = useForm({
    defaultValues: {
      enabled: tmdbSettings?.enabled ?? true,
      apiKey: tmdbSecrets?.api_key ?? "",
    },
    onSubmit: async ({ value }) => {
      await patchMutation.mutateAsync({
        settings: {
          assets: {
            tmdb: {
              enabled: value.enabled,
            },
          },
        },
        secrets: {
          asset_store: {
            tmdb: {
              api_key: value.apiKey.trim(),
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
        enabled: config.settings?.assets?.tmdb?.enabled ?? true,
        apiKey: config.secrets?.asset_store?.tmdb?.api_key ?? "",
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
            <div className="flex items-center justify-center size-12 rounded-2xl bg-sky-500/10 border border-sky-500/20 text-sky-500 shadow-inner">
              <Film className="w-6 h-6" />
            </div>
            <div>
              <SheetTitle className="text-lg font-bold text-foreground">
                The Movie Database (TMDB) Integration
              </SheetTitle>
              <SheetDescription className="text-xs text-text-muted mt-0.5">
                Configure your TMDB API key to retrieve movie and TV show
                artwork, posters, and metadata.
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
                label="Enable TMDB Integration"
                description="Allow Itonda to retrieve posters, backdrops, and artwork from The Movie Database."
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
                      aria-label="Toggle TMDB Enabled"
                    />
                  )}
                />
              </SettingRow>

              <Separator className="bg-white/5" />

              <div className="space-y-3 pt-2">
                <h4 className="text-xs font-semibold uppercase tracking-wider text-text-muted/70">
                  Authentication & Credentials
                </h4>

                <form.Field
                  name="apiKey"
                  children={(field) => (
                    <SettingRow
                      label="TMDB API Key (v3 auth)"
                      description="Personal API key used to authenticate requests to The Movie Database."
                      layout="vertical"
                      htmlFor="drawer-tmdb-api-key"
                    >
                      <SecretInput
                        id="drawer-tmdb-api-key"
                        value={field.state.value}
                        onChange={(apiKey) => {
                          field.handleChange(apiKey);
                          triggerSave(false);
                        }}
                        placeholder="e.g. 1a2b3c4d5e..."
                        portalUrl="https://www.themoviedb.org/settings/api"
                        portalLabel="Get TMDB API key"
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
