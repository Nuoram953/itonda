import type { components } from "@/api/generated.d";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Play } from "lucide-react";
import { useState } from "react";
import { useLaunchMedia } from "../../api/post-media-launch";
import { useActiveMedia } from "@/hooks/use-active-media";
import { cn } from "@/lib/utils";

type LaunchProps = {
  profiles: components["schemas"]["Media"]["launches"];
  mediaId?: string;
};

export const Launch = ({ profiles = [], mediaId }: LaunchProps) => {
  const [open, setOpen] = useState(false);
  const launchMediaMutation = useLaunchMedia({});
  const { session, isPlaying, formattedElapsed } = useActiveMedia();

  const isCurrentMediaPlaying = Boolean(
    isPlaying && mediaId && session?.mediaId === mediaId,
  );

  function launch(id: string) {
    launchMediaMutation.mutate(id, {
      onSuccess: () => setOpen(false),
    });
  }

  function handleClick() {
    if (profiles.length === 0) {
      return;
    }

    if (profiles.length === 1) {
      launch(profiles[0].id);
      return;
    }

    setOpen(true);
  }

  return (
    <>
      <button
        type="button"
        disabled={!profiles.length}
        onClick={handleClick}
        aria-label={isCurrentMediaPlaying ? "Now Playing" : "Play"}
        className={cn(
          "inline-flex items-center gap-2 px-5 py-2 rounded-xl font-extrabold text-xs sm:text-sm shadow-lg transition-all active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer",
          isCurrentMediaPlaying
            ? "bg-emerald-500 hover:bg-emerald-400 text-black shadow-emerald-500/20"
            : "bg-accent-gold hover:bg-accent-gold-hover text-black shadow-accent-gold/20",
        )}
      >
        {isCurrentMediaPlaying ? (
          <>
            <span className="relative flex h-2 w-2">
              <span className="relative inline-flex h-2 w-2 rounded-full bg-black" />
            </span>
            <span>Playing ({formattedElapsed})</span>
          </>
        ) : (
          <>
            <Play className="w-3.5 h-3.5 fill-current ml-0.5" />
            <span>Launch</span>
          </>
        )}
      </button>

      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Select launch profile</DialogTitle>
            <DialogDescription>
              Choose how you want to launch this game.
            </DialogDescription>
          </DialogHeader>

          <div className="flex flex-col gap-2">
            {profiles.map((profile) => (
              <Button
                key={profile.id}
                variant="outline"
                onClick={() => launch(profile.id)}
              >
                {profile.name}
              </Button>
            ))}
          </div>

          <DialogFooter>
            <Button variant="ghost" onClick={() => setOpen(false)}>
              Cancel
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
};
