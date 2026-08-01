import { useMedia } from "./api/get-media";
import { Workspace } from "@/components/workspace/Workspace";
import { Card } from "./components/card";
import { Refresh } from "./components/action/refresh";
import { Link } from "@tanstack/react-router";

export const Libary = () => {
  const mediaQuery = useMedia({});

  if (mediaQuery.isLoading) {
    return <p>Loading...</p>;
  }

  const media = mediaQuery.data?.items ?? [];

  return (
    <Workspace>
      <Workspace.Header title="Media" subtitle={`${media.length} items`}>
        <Workspace.Actions>
          <Refresh />
        </Workspace.Actions>
      </Workspace.Header>

      <Workspace.Content>
        <ul className="grid grid-cols-[repeat(auto-fill,15rem)] gap-6 justify-center">
          {media.map((item) => (
            <Link
              key={item.id}
              to="/media/$mediaId"
              params={{ mediaId: item.id }}
            >
              <li>
                <Card media={item} />
              </li>
            </Link>
          ))}
        </ul>
      </Workspace.Content>
    </Workspace>
  );
};
