import { Workspace } from "@/components/workspace/Workspace";
import { useParams } from "@tanstack/react-router";

export const MediaDetails = () => {
  const { mediaId } = useParams({
    from: "/media/$mediaId",
  });
  return (
    <Workspace>
      <Workspace.Header title={mediaId}></Workspace.Header>

      <Workspace.Content>
        <div />
      </Workspace.Content>
    </Workspace>
  );
};
