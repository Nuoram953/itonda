import { useState } from "react";
import { useParams } from "@tanstack/react-router";
import { Bookmark, MoreVertical } from "lucide-react";
import { Workspace } from "@/components/workspace/Workspace";
import { LoadingState } from "@/components/feedback/LoadingState";
import { ErrorState } from "@/components/feedback/ErrorState";
import { useMediaById } from "./api/get-media-id";
import { Launch } from "./components/action/Launch";
import { Status } from "./components/action/status";
import { HeroHeader } from "./components/hero/HeroHeader";
import { DetailsTabs, type TabId } from "./components/navigation/DetailsTabs";
import { OverviewTab } from "./components/overview/OverviewTab";
import { GalleryTab } from "./components/tabs/GalleryTab";
import { DetailsInfoTab } from "./components/tabs/DetailsInfoTab";

type HeaderActionButtonProps = {
  icon: React.ReactNode;
  title: string;
  onClick?: () => void;
};

const HeaderActionButton = ({
  icon,
  title,
  onClick,
}: HeaderActionButtonProps) => (
  <button
    type="button"
    onClick={onClick}
    className="p-2.5 rounded-xl bg-surface/70 border border-white/10 text-white/80 hover:text-white hover:bg-white/10 transition-colors shadow-lg cursor-pointer"
    title={title}
  >
    {icon}
  </button>
);

export const MediaDetails = () => {
  const { mediaId } = useParams({
    from: "/media/$mediaId",
  });

  const mediaQuery = useMediaById({ mediaId });
  const [activeTab, setActiveTab] = useState<TabId>("overview");

  if (mediaQuery.isPending) {
    return <LoadingState message="Loading media details..." />;
  }

  const media = mediaQuery.data;

  if (!media) {
    return (
      <ErrorState
        title="Media not found"
        message="The requested item could not be loaded or may have been deleted."
      />
    );
  }

  return (
    <Workspace>
      <Workspace.Header
        title={
          <span className="text-sm font-medium text-white/90">
            Back to Library
          </span>
        }
        showBackBtn
        className="absolute top-0 inset-x-0 z-30 bg-transparent border-none text-foreground py-5 px-6 pointer-events-none *:pointer-events-auto"
      >
        <Workspace.Actions className="flex items-center gap-3">
          <Launch profiles={media.launches} />
          <Status mediaId={mediaId} currentStatus={media.status} />
          <HeaderActionButton
            icon={<Bookmark className="w-4 h-4" />}
            title="Bookmark"
          />
          <HeaderActionButton
            icon={<MoreVertical className="w-4 h-4" />}
            title="More options"
          />
        </Workspace.Actions>
      </Workspace.Header>

      <Workspace.Content className="p-0 flex flex-col min-h-full bg-background relative">
        <HeroHeader media={media} />
        <DetailsTabs activeTab={activeTab} onChange={setActiveTab} />

        <div className="flex-1 max-w-7xl w-full mx-auto px-6">
          {activeTab === "overview" && (
            <OverviewTab media={media} onNavigateTab={setActiveTab} />
          )}
          {activeTab === "gallery" && <GalleryTab media={media} />}
          {activeTab === "details" && <DetailsInfoTab media={media} />}
        </div>
      </Workspace.Content>
    </Workspace>
  );
};
