import { useMemo } from "react";
import { useMedia } from "../api/get-media";
import { useLibrary } from "../hooks/useLibrary";
import { Link } from "@tanstack/react-router";
import { Card } from "./card";
import { Workspace } from "@/components/workspace/Workspace";
import { Search } from "./action/search";
import { Refresh } from "./action/refresh";

export const MediaGrid = () => {
  const { search, filters, sort, applyFilters } = useLibrary();
  const mediaQuery = useMedia({});

  const media = mediaQuery.data?.items;

  const filteredMedia = useMemo(
    () => applyFilters(media ?? [], search, filters, sort),
    [media, search, filters, sort, applyFilters],
  );

  return (
    <Workspace>
      <Workspace.Header
        title="Media"
        subtitle={`${filteredMedia.length} items`}
      >
        <Workspace.Actions>
          <Search />
          <Refresh />
        </Workspace.Actions>
      </Workspace.Header>

      <Workspace.Content>
        <ul className="grid grid-cols-[repeat(auto-fill,15rem)] gap-6 justify-center">
          {filteredMedia.map((item) => (
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
