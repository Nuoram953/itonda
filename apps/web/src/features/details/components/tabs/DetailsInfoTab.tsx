import { Info, Monitor } from "lucide-react";
import type { components } from "@/api/generated.d";

type DetailsInfoTabProps = {
  media: components["schemas"]["Media"];
};

export function DetailsInfoTab({ media }: DetailsInfoTabProps) {
  return (
    <div className="space-y-6 animate-in fade-in duration-300 pb-12">
      <div className="border-b border-white/10 pb-4">
        <h2 className="text-2xl font-extrabold text-foreground tracking-tight">
          Game Details & Specifications
        </h2>
        <p className="text-sm text-text-muted">
          Metadata, developer details, and technical specifications for{" "}
          {media.title}.
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="p-6 rounded-2xl bg-surface/70 border border-white/10 space-y-4">
          <h3 className="text-sm font-bold uppercase tracking-wider text-primary flex items-center gap-2">
            <Info className="w-4 h-4" />
            <span>Metadata Summary</span>
          </h3>

          <div className="space-y-3 text-sm">
            <div className="flex justify-between py-2 border-b border-white/5">
              <span className="text-text-muted">Title</span>
              <span className="font-semibold text-foreground">
                {media.title}
              </span>
            </div>
            <div className="flex justify-between py-2 border-b border-white/5">
              <span className="text-text-muted">Media Type</span>
              <span className="font-semibold text-foreground capitalize">
                {media.media_type}
              </span>
            </div>
            <div className="flex justify-between py-2 border-b border-white/5">
              <span className="text-text-muted">Developer</span>
              <span className="font-semibold text-foreground">
                Warhorse Studios
              </span>
            </div>
            <div className="flex justify-between py-2 border-b border-white/5">
              <span className="text-text-muted">Release Year</span>
              <span className="font-semibold text-foreground">2022</span>
            </div>
            <div className="flex justify-between py-2">
              <span className="text-text-muted">Platform Support</span>
              <span className="font-semibold text-foreground">
                PC (Windows / Linux)
              </span>
            </div>
          </div>
        </div>

        <div className="p-6 rounded-2xl bg-surface/70 border border-white/10 space-y-4">
          <h3 className="text-sm font-bold uppercase tracking-wider text-primary flex items-center gap-2">
            <Monitor className="w-4 h-4" />
            <span>Configured Launch Profiles ({media.launches.length})</span>
          </h3>

          <div className="space-y-2">
            {media.launches.length > 0 ? (
              media.launches.map((launch) => (
                <div
                  key={launch.id}
                  className="p-3 rounded-xl bg-surface-raised border border-white/5 flex items-center justify-between text-xs"
                >
                  <span className="font-bold text-foreground">
                    {launch.name}
                  </span>
                  <span className="text-text-muted">ID: {launch.id}</span>
                </div>
              ))
            ) : (
              <p className="text-xs text-text-muted">
                No launch profiles configured yet.
              </p>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
