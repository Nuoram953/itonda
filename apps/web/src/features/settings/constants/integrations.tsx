import type { ReactNode } from "react";
import { Gamepad2, Image, Film, Database } from "lucide-react";
import type { CombinedConfig, PatchConfigPayload } from "../types/settings";
import { SteamDrawer } from "../components/drawers/SteamDrawer";
import { SteamGridDbDrawer } from "../components/drawers/SteamGridDbDrawer";
import { TmdbDrawer } from "../components/drawers/TmdbDrawer";
import { IgdbDrawer } from "../components/drawers/IgdbDrawer";

export type DrawerId = "steam" | "steam_grid_db" | "tmdb" | "igdb";

export type DrawerComponent = React.ComponentType<{
  open: boolean;
  onOpenChange: (open: boolean) => void;
}>;

export type IntegrationItem = {
  id: DrawerId;
  title: string;
  mediaTypes: Array<"game" | "movie" | "tv_show">;
  description: string;
  icon: ReactNode;
  iconBgClass: string;
  drawer: DrawerComponent;
  isEnabled: (config: CombinedConfig) => boolean;
  getIssue: (config: CombinedConfig) => string | undefined;
  getTogglePayload: (enabled: boolean) => PatchConfigPayload;
};

export type SettingsSectionItem = {
  id: string;
  title: string;
  description: string;
  integrations: IntegrationItem[];
};

const DRAWER_ALIASES: Record<string, DrawerId> = {
  steam: "steam",
  steamgriddb: "steam_grid_db",
  steam_grid_db: "steam_grid_db",
  tmdb: "tmdb",
  the_movie_database: "tmdb",
  igdb: "igdb",
};

export function getInitialDrawer(): DrawerId | null {
  const drawer = new URLSearchParams(window.location.search).get("drawer");
  return drawer ? (DRAWER_ALIASES[drawer] ?? null) : null;
}

export const SETTINGS_SECTIONS: SettingsSectionItem[] = [
  {
    id: "storefronts",
    title: "Storefronts",
    description:
      "Connect gaming platforms and digital storefronts to sync libraries, playtimes, and achievements.",
    integrations: [
      {
        id: "steam",
        title: "Steam",
        mediaTypes: ["game"],
        description:
          "Automatically import owned games, playtime, achievements, and assets from your Steam account.",
        icon: <Gamepad2 className="w-5 h-5" />,
        iconBgClass: "bg-primary/10 text-primary border-primary/20",
        drawer: SteamDrawer,
        isEnabled: (c) => c.settings?.metadata?.steam?.enabled ?? true,
        getIssue: (c) => {
          const steamAccount =
            c.secrets?.storefronts?.steam?.account_name ||
            (c.secrets?.storefronts?.steam?.steam_id
              ? `Steam ID: ${c.secrets?.storefronts?.steam?.steam_id}`
              : null);
          return !steamAccount
            ? "Account not linked — sign in to sync library"
            : undefined;
        },
        getTogglePayload: (enabled) => ({
          settings: {
            metadata: {
              steam: { enabled },
            },
          },
        }),
      },
    ],
  },
  {
    id: "assets",
    title: "Assets",
    description:
      "Configure artwork, posters, banners, and media providers for your library.",
    integrations: [
      {
        id: "steam_grid_db",
        title: "SteamGridDB",
        mediaTypes: ["game"],
        description:
          "Download high-resolution vertical posters, hero banners, and artwork for your games.",
        icon: <Image className="w-5 h-5" />,
        iconBgClass: "bg-amber-500/10 text-amber-500 border-amber-500/20",
        drawer: SteamGridDbDrawer,
        isEnabled: (c) =>
          c.settings?.assets?.steam_grid_db?.enabled ?? true,
        getIssue: (c) =>
          !c.secrets?.asset_store?.steam_grid_db?.api_key?.trim()
            ? "API key required — configure key to fetch assets"
            : undefined,
        getTogglePayload: (enabled) => ({
          settings: {
            assets: {
              steam_grid_db: { enabled },
            },
          },
        }),
      },
      {
        id: "tmdb",
        title: "The Movie Database",
        mediaTypes: ["movie", "tv_show"],
        description:
          "Download high-resolution posters, backdrops, and artwork for movies and TV shows.",
        icon: <Film className="w-5 h-5" />,
        iconBgClass: "bg-sky-500/10 text-sky-500 border-sky-500/20",
        drawer: TmdbDrawer,
        isEnabled: (c) => c.settings?.assets?.tmdb?.enabled ?? true,
        getIssue: (c) =>
          !c.secrets?.asset_store?.tmdb?.api_key?.trim()
            ? "API key required — configure key to fetch assets"
            : undefined,
        getTogglePayload: (enabled) => ({
          settings: {
            assets: {
              tmdb: { enabled },
            },
          },
        }),
      },
    ],
  },
  {
    id: "metadata",
    title: "Metadata",
    description:
      "Configure metadata providers to fetch summaries, release dates, and developer details.",
    integrations: [
      {
        id: "igdb",
        title: "IGDB",
        mediaTypes: ["game"],
        description:
          "Retrieve game summaries, developer details, genres, and release information from The Internet Game Database.",
        icon: <Database className="w-5 h-5" />,
        iconBgClass: "bg-purple-500/10 text-purple-400 border-purple-500/20",
        drawer: IgdbDrawer,
        isEnabled: (c) => c.settings?.metadata?.igdb?.enabled ?? true,
        getIssue: (c) => {
          const hasClientId =
            !!c.secrets?.metadata_store?.igdb?.client_id?.trim();
          const hasClientSecret =
            !!c.secrets?.metadata_store?.igdb?.client_secret?.trim();
          return !hasClientId || !hasClientSecret
            ? "Credentials required — configure Twitch Client ID and Secret to fetch metadata"
            : undefined;
        },
        getTogglePayload: (enabled) => ({
          settings: {
            metadata: {
              igdb: { enabled },
            },
          },
        }),
      },
    ],
  },
];
