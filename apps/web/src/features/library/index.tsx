import { useMedia } from "./api/get-media";
import { Workspace } from "@/components/workspace/Workspace";
import { Card } from "./components/card";

export const Libary = () => {
  const mediaQuery = useMedia({});

  if (mediaQuery.isLoading) {
    return <p>Loading...</p>;
  }

  const media = mediaQuery.data?.items ?? [];

  return (
    <Workspace>
      <Workspace.Header title="Media" subtitle={`${media.length} items`} />

      <Workspace.Content>
        <ul className="grid grid-cols-[repeat(auto-fill,15rem)] gap-6 justify-center">
          {media.map((item) => (
            <li key={item.id}>
              <Card name={item.title} />
            </li>
          ))}
        </ul>
      </Workspace.Content>
    </Workspace>
  );
};
