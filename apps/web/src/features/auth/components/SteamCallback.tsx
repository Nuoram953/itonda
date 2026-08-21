import { useEffect, useState } from "react";
import { LoadingState } from "@/components/feedback/LoadingState";
import { verifySteamCallback } from "@/features/settings/api/auth";

export function SteamCallback() {
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const searchParams = new URLSearchParams(window.location.search);
    const params: Array<[string, string]> = Array.from(searchParams.entries());

    if (params.length === 0) {
      setError("No authentication parameters found");
      return;
    }

    verifySteamCallback({ params })
      .then((res) => {
        if (window.opener) {
          window.opener.postMessage(
            {
              type: "STEAM_AUTH_SUCCESS",
              steamId: res.steam_id,
              accountName: res.account_name,
              avatarUrl: res.avatar_url,
            },
            "*"
          );
        }
        window.close();
      })
      .catch((err: unknown) => {
        const errorMsg =
          err instanceof Error
            ? err.message
            : "Failed to verify Steam authentication";

        if (window.opener) {
          window.opener.postMessage(
            { type: "STEAM_AUTH_ERROR", error: errorMsg },
            "*"
          );
        }
        setError(errorMsg);
        setTimeout(() => window.close(), 2000);
      });
  }, []);

  if (error) {
    return (
      <div className="flex h-screen items-center justify-center bg-surface p-6 text-destructive text-center">
        <p className="text-sm font-semibold">{error}</p>
      </div>
    );
  }

  return (
    <div className="flex h-screen items-center justify-center bg-surface">
      <LoadingState message="Verifying Steam authentication..." />
    </div>
  );
}
