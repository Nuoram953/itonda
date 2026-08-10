import { Workspace } from "@/components/workspace/Workspace";
import { useParams } from "@tanstack/react-router";
import { Launch } from "./components/action/Launch";
import { useMediaById } from "./api/get-media-id";
import { Status } from "./components/action/status";

export const MediaDetails = () => {
  const { mediaId } = useParams({
    from: "/media/$mediaId",
  });

  const mediaQuery = useMediaById({ mediaId });

  if (mediaQuery.isPending) {
    return <div>Loading...</div>;
  }

  const data = mediaQuery.data;

  if (!data) {
    return;
  }

  return (
    <Workspace>
      <Workspace.Header title={data.title} showBackBtn>
        <Workspace.Actions>
          <Launch profiles={data.launches} />
          <Status mediaId={mediaId} currentStatus={data.status} />
        </Workspace.Actions>
      </Workspace.Header>

      <Workspace.Content>
        <div />
      </Workspace.Content>
    </Workspace>
  );
};
