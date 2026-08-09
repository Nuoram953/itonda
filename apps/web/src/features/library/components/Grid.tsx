import { useMemo } from "react";
import { Link, useSearch } from "@tanstack/react-router";
import { useMedia } from "../api/get-media";
import { useLibrary } from "../hooks/useLibrary";
import { Card } from "./card";
import { Workspace } from "@/components/workspace/Workspace";
import { Search } from "./action/search";
import { Refresh } from "./action/refresh";

const typeTitles: Record<string, string> = {
  game: "Games",
  movie: "Movies",
  tv_show: "TV Series",
};

export const MediaGrid = () => {
  const searchParams = useSearch({ strict: false });
  const type = typeof searchParams?.type === "string" ? searchParams.type : undefined;

  const { search, filters, sort, applyFilters } = useLibrary();
  const mediaQuery = useMedia({ type });

  const media = mediaQuery.data?.items;

  const filteredMedia = useMemo(
    () => applyFilters(media ?? [], search, filters, sort),
    [media, search, filters, sort, applyFilters],
  );

  const title = type ? typeTitles[type] ?? "Media" : "Media";

  return (
    <Workspace>
      <Workspace.Header
        title={title}
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
