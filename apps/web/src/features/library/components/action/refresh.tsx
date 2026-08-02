import { Workspace } from "@/components/workspace/Workspace";
import { useRefreshMedia } from "../../api/post-media-refresh";
import { RefreshCcw } from "lucide-react";

export const Refresh = () => {
  const refreshMediaMutation = useRefreshMedia({});

  return (
    <Workspace.Action
      icon={RefreshCcw}
      onClick={() => refreshMediaMutation.mutate({})}
      disabled={refreshMediaMutation.isPending}
    >
      Refresh
    </Workspace.Action>
  );
};
