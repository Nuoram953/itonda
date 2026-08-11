import { Workspace } from "@/components/workspace/Workspace";
import { useParams } from "@tanstack/react-router";
import { Launch } from "./components/action/Launch";
import { useMediaById } from "./api/get-media-id";
import { Status } from "./components/action/status";
import { useState } from "react";

export const MediaDetails = () => {
  const [loading, setLoading] = useState(true);
  const { mediaId } = useParams({
    from: "/media/$mediaId",
  });

  const mediaQuery = useMediaById({ mediaId });

  if (mediaQuery.isPending) {
    return <div>Loading...</div>;
  }

  const media = mediaQuery.data;

  if (!media) {
    return;
  }

  const banner = media.assets.find((asset) => asset.asset_type === "banner");

  return (
    <Workspace>
      <Workspace.Header title={media.title} showBackBtn>
        <Workspace.Actions>
          <Launch profiles={media.launches} />
          <Status mediaId={mediaId} currentStatus={media.status} />
        </Workspace.Actions>
      </Workspace.Header>

      <Workspace.Content className="p-0">
        {banner && (
          <div className="relative w-full h-80 shrink-0 overflow-hidden bg-surface">
            {/* temporary  */}
            <img
              key={banner.id}
              src={`http://localhost:3005/api/v1/assets/${banner.id}`}
              alt={media.title}
              className={`
                h-full w-full object-cover block transition-opacity duration-200
                ${loading ? "opacity-0" : "opacity-100"}
              `}
              onLoad={() => setLoading(false)}
              onError={() => setLoading(false)}
            />
          </div>
        )}
        <div className="p-6" />
      </Workspace.Content>
    </Workspace>
  );
};
