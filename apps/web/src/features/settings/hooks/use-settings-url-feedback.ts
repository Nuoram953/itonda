import { useEffect } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { useNotification } from "@/hooks/use-notification";

export function useSettingsUrlFeedback() {
  const { notify } = useNotification();
  const queryClient = useQueryClient();

  useEffect(() => {
    const searchParams = new URLSearchParams(window.location.search);
    const authStatus = searchParams.get("auth");
    const drawer = searchParams.get("drawer");
    const errorMessage = searchParams.get("error");

    if (authStatus === "success") {
      notify.success({
        title: "Steam Connected",
        description: "Your Steam account was linked successfully.",
      });
      queryClient.invalidateQueries({ queryKey: ["config"] });
      queryClient.invalidateQueries({ queryKey: ["auth", "steam", "status"] });
    } else if (authStatus === "error") {
      notify.error({
        title: "Steam Authentication Failed",
        description:
          errorMessage || "Could not complete Steam login. Please try again.",
      });
    }

    if (authStatus || drawer) {
      const cleanUrl = window.location.pathname;
      window.history.replaceState({}, document.title, cleanUrl);
    }
  }, [notify, queryClient]);
}
