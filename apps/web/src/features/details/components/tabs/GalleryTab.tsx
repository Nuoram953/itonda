import { Film, Image as ImageIcon, Play } from "lucide-react";
import type { components } from "@/api/generated.d";
import { getAssetUrl } from "../../utils/asset-url";
import {
  getScreenshotAssets,
  getTrailerAssets,
} from "../../utils/media-assets";

type GalleryTabProps = {
  media: components["schemas"]["Media"];
};

export function GalleryTab({ media }: GalleryTabProps) {
  const trailers = getTrailerAssets(media.assets);
  const screenshots = getScreenshotAssets(media.assets);

  return (
    <div className="space-y-8 animate-in fade-in duration-300 pb-12">
      <div className="space-y-1">
        <h2 className="text-2xl font-extrabold text-foreground tracking-tight">
          Gallery & Videos
        </h2>
        <p className="text-sm text-text-muted">
          Browse media, trailers, and captured screenshots for {media.title}.
        </p>
      </div>

      {/* Videos Section */}
      <div className="space-y-4">
        <div className="flex items-center gap-2 text-xs font-bold text-primary uppercase tracking-wider">
          <Film className="w-4 h-4" />
          <span>Trailers & Clips</span>
        </div>

        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          {trailers.length > 0 ? (
            trailers.map((trailer, idx) => (
              <div
                key={trailer.id}
                className="group relative aspect-video rounded-2xl overflow-hidden bg-slate-900 border border-white/10 hover:border-primary/40 cursor-pointer shadow-lg transition-all hover:-translate-y-1"
              >
                <video
                  src={getAssetUrl(trailer.id)}
                  className="w-full h-full object-cover"
                />
                <div className="absolute inset-0 bg-slate-950/40 group-hover:bg-slate-950/20 transition-colors" />
                <div className="absolute inset-0 flex items-center justify-center">
                  <div className="w-12 h-12 rounded-full bg-primary text-primary-foreground flex items-center justify-center shadow-xl group-hover:scale-110 transition-transform">
                    <Play className="w-5 h-5 fill-current ml-0.5" />
                  </div>
                </div>
                <div className="absolute bottom-3 left-3 text-xs font-bold text-foreground">
                  Trailer #{idx + 1}
                </div>
              </div>
            ))
          ) : (
            <div className="col-span-full p-8 rounded-2xl bg-surface/50 border border-white/10 text-center text-text-muted text-sm">
              No standalone video trailers uploaded for this media yet.
            </div>
          )}
        </div>
      </div>

      {/* Screenshots Section */}
      <div className="space-y-4 pt-4">
        <div className="flex items-center gap-2 text-xs font-bold text-primary uppercase tracking-wider">
          <ImageIcon className="w-4 h-4" />
          <span>Screenshots</span>
        </div>

        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
          {screenshots.map((shot, idx) => (
            <div
              key={shot.id}
              className="group relative aspect-video rounded-2xl overflow-hidden bg-slate-900 border border-white/10 hover:border-primary/40 cursor-pointer shadow-md transition-all hover:-translate-y-1"
            >
              <img
                src={getAssetUrl(shot.id)}
                alt={`Screenshot ${idx + 1}`}
                className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
              />
              <div className="absolute inset-0 bg-slate-950/20 group-hover:bg-transparent transition-colors" />
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
