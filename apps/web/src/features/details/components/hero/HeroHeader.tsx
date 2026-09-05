import { useState, useRef } from "react";
import { Play, Pause, Volume2, VolumeX } from "lucide-react";
import type { components } from "@/api/generated.d";
import { getAssetUrl } from "../../utils/asset-url";
import {
  getHeroBannerAsset,
  getHeroPosterAsset,
  getHeroTrailerAsset,
} from "../../utils/media-assets";
import { formatLastPlayedDate, formatPlaytime } from "@/utils/datetime";
import { cn } from "@/lib/utils";

type HeroHeaderProps = {
  media: components["schemas"]["Media"];
};

//TODO: Should come from the api
const DEFAULT_TAGS = [
  "Souls-like",
  "Dark Fantasy",
  "Singleplayer",
  "Masterpiece",
];

export function HeroHeader({ media }: HeroHeaderProps) {
  const trailerAsset = getHeroTrailerAsset(media.assets);
  const bannerAsset = getHeroBannerAsset(media.assets);
  const posterAsset = getHeroPosterAsset(media.assets);

  const videoRef = useRef<HTMLVideoElement>(null);
  const [isVideoPlaying, setIsVideoPlaying] = useState(true);
  const [isMuted, setIsMuted] = useState(true);
  const [videoError, setVideoError] = useState(false);
  const [imageLoaded, setImageLoaded] = useState(false);

  const togglePlayVideo = () => {
    if (!videoRef.current) return;
    if (isVideoPlaying) {
      videoRef.current.pause();
      setIsVideoPlaying(false);
    } else {
      videoRef.current
        .play()
        .then(() => setIsVideoPlaying(true))
        .catch(() => setIsVideoPlaying(false));
    }
  };

  const toggleMuteVideo = () => {
    if (!videoRef.current) return;
    const nextMuted = !isMuted;
    videoRef.current.muted = nextMuted;
    setIsMuted(nextMuted);
  };

  const showVideo = Boolean(trailerAsset && !videoError);
  const playtimeHours = formatPlaytime(media.details?.playtime_minutes, {
    mode: "compact",
  });
  const lastPlayedDate = formatLastPlayedDate(media.details?.last_played_at);

  return (
    <div className="relative w-full min-h-120 md:min-h-136 overflow-hidden bg-background select-none flex flex-col justify-end">
      <div className="absolute inset-0 w-full h-full">
        {showVideo && trailerAsset ? (
          <video
            ref={videoRef}
            src={getAssetUrl(trailerAsset.id)}
            autoPlay
            muted={isMuted}
            loop
            playsInline
            onError={() => setVideoError(true)}
            onPlay={() => setIsVideoPlaying(true)}
            onPause={() => setIsVideoPlaying(false)}
            className="w-full h-full object-cover transition-opacity duration-700"
          />
        ) : null}

        {bannerAsset && (!showVideo || !isVideoPlaying) && (
          <img
            src={getAssetUrl(bannerAsset.id)}
            alt={media.title}
            onLoad={() => setImageLoaded(true)}
            className={cn(
              "absolute inset-0 w-full h-full object-cover transition-opacity duration-700",
              imageLoaded ? "opacity-100" : "opacity-0",
            )}
          />
        )}

        {!bannerAsset && !showVideo && (
          <div className="w-full h-full bg-linear-to-br from-slate-900 via-surface to-slate-950" />
        )}
      </div>

      <div className="absolute inset-x-0 top-0 h-20 bg-linear-to-b from-black/50 to-transparent pointer-events-none" />
      <div className="absolute inset-x-0 bottom-0 h-48 bg-linear-to-t from-background via-background/60 to-transparent pointer-events-none" />
      <div className="absolute inset-0 bg-linear-to-r from-background/80 via-background/40 to-transparent pointer-events-none md:w-3/4" />
      <div className="relative z-10 w-full max-w-7xl mx-auto px-6 flex flex-col justify-end pt-20 pb-8">
        <div className="flex flex-col md:flex-row md:items-start gap-6">
          <div className="shrink-0 w-36 h-48 md:w-44 md:h-66 rounded-xl overflow-hidden shadow-2xl border border-white/15 bg-surface transition-transform duration-300 hover:scale-105">
            {posterAsset ? (
              <img
                src={getAssetUrl(posterAsset.id)}
                alt={media.title}
                className="w-full h-full object-cover"
              />
            ) : (
              <div className="w-full h-full bg-linear-to-br from-slate-800 to-slate-950 flex flex-col items-center justify-center p-4 text-center">
                <span className="text-xs font-bold text-text-muted">
                  {media.title}
                </span>
              </div>
            )}
          </div>

          <div className="flex-1 flex flex-col justify-between md:min-h-66">
            <div className="space-y-2.5">
              <h1 className="text-4xl font-extrabold tracking-tight text-foreground drop-shadow-md leading-tight">
                {media.title}
              </h1>

              <div className="flex flex-wrap items-center gap-2 text-xs sm:text-sm font-medium text-text-muted">
                <span>
                  Action RPG · Open World · Medieval · FromSoftware · 2022
                </span>
              </div>

              <div className="flex flex-wrap items-end justify-between gap-4 pt-1">
                {media.media_type == "game" && (
                  <div className="flex flex-wrap items-center gap-6 text-xs">
                    <div>
                      <span className="block text-xs font-semibold uppercase tracking-wider text-text-muted">
                        Playtime
                      </span>
                      <span className="text-md font-bold text-foreground">
                        {playtimeHours}
                      </span>
                    </div>

                    <div className="w-px h-6 bg-border/60" />

                    <div>
                      <span className="block text-xs font-semibold uppercase tracking-wider text-text-muted">
                        Last Played
                      </span>
                      <span className="text-md font-semibold text-foreground">
                        {lastPlayedDate}
                      </span>
                    </div>

                    <div className="w-px h-6 bg-border/60" />
                  </div>
                )}
              </div>
            </div>
          </div>

          {showVideo && (
            <div className="flex items-center gap-2 self-start bg-surface/70 backdrop-blur px-3 py-1.5 rounded-full border border-white/10 shadow-lg">
              <button
                type="button"
                onClick={togglePlayVideo}
                className="p-1.5 rounded-full hover:bg-white/10 text-foreground transition-colors"
                title={isVideoPlaying ? "Pause Trailer" : "Play Trailer"}
              >
                {isVideoPlaying ? (
                  <Pause className="w-4 h-4" />
                ) : (
                  <Play className="w-4 h-4" />
                )}
              </button>

              <div className="w-px h-4 bg-border" />

              <button
                type="button"
                onClick={toggleMuteVideo}
                className="p-1.5 rounded-full hover:bg-white/10 text-foreground transition-colors"
                title={isMuted ? "Unmute Audio" : "Mute Audio"}
              >
                {isMuted ? (
                  <VolumeX className="w-4 h-4 text-text-muted" />
                ) : (
                  <Volume2 className="w-4 h-4 text-primary" />
                )}
              </button>

              <span className="text-xs font-medium text-text-muted uppercase px-1">
                Trailer
              </span>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
