import { useQuery, useMutation, useQueryClient, queryOptions } from "@tanstack/react-query";
import { api } from "@/lib/api-client";
import { type QueryConfig, type MutationConfig } from "@/lib/react-query";

export type StorefrontAuthStatus = {
  storefront: "Steam";
  connected: boolean;
  steam_id: string | null;
  account_name: string | null;
  avatar_url: string | null;
};

export type AuthActionResponse = {
  success: boolean;
  message: string;
};

export type SteamCallbackPayload = {
  params: Array<[string, string]>;
};

export const getSteamAuthStatus = (): Promise<StorefrontAuthStatus> => {
  return api.get(`/auth/steam/status`);
};

export const verifySteamCallback = (
  payload: SteamCallbackPayload
): Promise<StorefrontAuthStatus> => {
  return api.post(`/auth/steam/callback`, payload);
};

export const getSteamAuthStatusQueryOptions = () => {
  return queryOptions({
    queryKey: ["auth", "steam", "status"],
    queryFn: () => getSteamAuthStatus(),
  });
};

export const useSteamAuthStatus = (
  { queryConfig }: { queryConfig?: QueryConfig<typeof getSteamAuthStatusQueryOptions> } = {}
) => {
  return useQuery({
    ...getSteamAuthStatusQueryOptions(),
    ...queryConfig,
  });
};

export const disconnectSteam = (): Promise<AuthActionResponse> => {
  return api.post(`/auth/steam/disconnect`);
};

export const useDisconnectSteam = (
  { mutationConfig }: { mutationConfig?: MutationConfig<typeof disconnectSteam> } = {}
) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: disconnectSteam,
    onSuccess: (...args) => {
      queryClient.invalidateQueries({ queryKey: ["auth", "steam", "status"] });
      queryClient.invalidateQueries({ queryKey: ["config"] });
      mutationConfig?.onSuccess?.(...args);
    },
    ...mutationConfig,
  });
};
