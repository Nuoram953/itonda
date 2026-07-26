import { Workspace } from "@/components/workspace/Workspace";
import { RiRefreshLine } from "react-icons/ri";
import { useRefreshMedia } from "../../api/post-media-refresh";

export const Refresh = () => {
  const refreshMediaMutation = useRefreshMedia({});

  return (
    <Workspace.Action
      icon={RiRefreshLine}
      onClick={() => refreshMediaMutation.mutate({})}
      disabled={refreshMediaMutation.isPending}
    >
      Refresh
    </Workspace.Action>
  );
};
