import type { components } from "@/api/generated.d";

export type CombinedConfig = components["schemas"]["CombinedConfig"];
export type PatchConfigPayload = Partial<
  components["schemas"]["PatchConfigPayload"]
>;

export type SettingsCategoryFilter =
  | "all"
  | "storefronts"
  | "assets"
  | "metadata"
  | "system"
  | "preferences";

export type ConnectionStatus =
  | "connected"
  | "disconnected"
  | "testing"
  | "error";
