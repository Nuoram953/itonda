import { useState, useRef, useEffect } from "react";
import { useParams } from "@tanstack/react-router";
import { Bookmark, MoreVertical, RefreshCw } from "lucide-react";
import { cn } from "@/lib/utils";
import { Workspace } from "@/components/workspace/Workspace";
import { LoadingState } from "@/components/feedback/LoadingState";
import { ErrorState } from "@/components/feedback/ErrorState";
import { useMediaById } from "./api/get-media-id";
import { useSyncMedia } from "./api/post-media-sync";
import { Launch } from "./components/action/Launch";
import { Status } from "./components/action/status";
import { HeroHeader } from "./components/hero/HeroHeader";
import { DetailsTabs, type TabId } from "./components/navigation/DetailsTabs";
import { OverviewTab } from "./components/overview/OverviewTab";
import { ReviewsTab } from "./components/tabs/ReviewsTab";
import { GalleryTab } from "./components/tabs/GalleryTab";
import { DetailsInfoTab } from "./components/tabs/DetailsInfoTab";
import { FloatingOstPlayer } from "./components/audio/FloatingOstPlayer";
import { getHeroAudioAsset } from "./utils/media-assets";
import { getAssetUrl } from "./utils/asset-url";
import { useActiveMedia } from "@/hooks/use-active-media";
import { useAppFocus } from "@/hooks/use-app-focus";

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
  const syncMutation = useSyncMedia();
  const isAppFocused = useAppFocus();
  const { isPlaying: isGamePlaying } = useActiveMedia();

  const [activeTab, setActiveTab] = useState<TabId>("overview");
  const [userWantsPlayback, setUserWantsPlayback] = useState(true);
  const [isAudioPlaying, setIsAudioPlaying] = useState(false);
  const [volume, setVolume] = useState(0.3);
  const audioRef = useRef<HTMLAudioElement | null>(null);

  const media = mediaQuery.data;
  const audioAsset = getHeroAudioAsset(media?.assets);

  // The music should stop when playing a game or the application is not focused
  const shouldPlay = Boolean(
    audioAsset && userWantsPlayback && !isGamePlaying && isAppFocused,
  );

  useEffect(() => {
    const audio = audioRef.current;
    if (!audio || !audioAsset) return;

    if (shouldPlay) {
      const playPromise = audio.play();
      if (playPromise !== undefined) {
        playPromise
          .then(() => {
            setIsAudioPlaying(true);
          })
          .catch(() => {
            setIsAudioPlaying(false);
          });
      }
    } else {
      audio.pause();
      setIsAudioPlaying(false);
    }
  }, [shouldPlay, audioAsset?.id]);

  useEffect(() => {
    if (audioRef.current) {
      audioRef.current.volume = volume;
    }
  }, [volume]);

  const handleVolumeChange = (newVolume: number) => {
    setVolume(newVolume);
    if (audioRef.current) {
      audioRef.current.volume = newVolume;
    }
  };

  const toggleAudio = () => {
    if (isAudioPlaying) {
      setUserWantsPlayback(false);
    } else {
      setUserWantsPlayback(true);
    }
  };

  if (mediaQuery.isPending) {
    return <LoadingState message="Loading media details..." />;
  }

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
          <Launch profiles={media.launches} mediaId={mediaId} />
          <Status mediaId={mediaId} currentStatus={media.status} />
          <HeaderActionButton
            icon={
              <RefreshCw
                className={cn(
                  "w-4 h-4",
                  syncMutation.isPending && "animate-spin",
                )}
              />
            }
            title="Sync Media"
            onClick={() => syncMutation.mutate(mediaId)}
          />
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

        <div className="flex-1 max-w-7xl w-full mx-auto px-6 pt-6 sm:pt-8 pb-16">
          {activeTab === "overview" && (
            <OverviewTab media={media} onNavigateTab={setActiveTab} />
          )}
          {activeTab === "reviews" && <ReviewsTab media={media} />}
          {activeTab === "gallery" && <GalleryTab media={media} />}
          {activeTab === "details" && <DetailsInfoTab media={media} />}
        </div>

        {audioAsset && (
          <>
            <audio
              ref={(node) => {
                audioRef.current = node;
                if (node) {
                  node.volume = volume;
                }
              }}
              src={getAssetUrl(audioAsset.id)}
              preload="auto"
              loop
              onPlay={() => setIsAudioPlaying(true)}
              onPause={() => setIsAudioPlaying(false)}
              onEnded={() => setIsAudioPlaying(false)}
            />
            <FloatingOstPlayer
              title={media.title}
              trackName={audioAsset.name}
              isPlaying={isAudioPlaying}
              onToggle={toggleAudio}
              volume={volume}
              onVolumeChange={handleVolumeChange}
            />
          </>
        )}
      </Workspace.Content>
    </Workspace>
  );
};
