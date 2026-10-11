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
import { LoaderCircle, Play } from "lucide-react";
import { useMediaLauncher } from "@/hooks/use-media-launcher";
import { cn } from "@/lib/utils";

type LaunchProps = {
  profiles: components["schemas"]["Media"]["launches"];
  mediaId?: string;
};

export const Launch = ({ profiles = [], mediaId }: LaunchProps) => {
  const {
    open,
    setOpen,
    isLoading,
    isPlaying: isCurrentMediaPlaying,
    formattedElapsed,
    canLaunch,
    disabledReason,
    isProfileAvailable,
    launch,
    handleLaunchClick,
  } = useMediaLauncher({ profiles, mediaId });

  return (
    <>
      <button
        type="button"
        disabled={!canLaunch || isLoading}
        onClick={handleLaunchClick}
        title={disabledReason}
        aria-label={isCurrentMediaPlaying ? "Now Playing" : "Play"}
        className={cn(
          "inline-flex items-center gap-2 px-5 py-2 rounded-xl font-extrabold text-xs sm:text-sm shadow-lg transition-all active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer",
          isCurrentMediaPlaying
            ? "bg-emerald-500 hover:bg-emerald-400 text-black shadow-emerald-500/20"
            : "bg-accent-gold hover:bg-accent-gold-hover text-black shadow-accent-gold/20",
        )}
      >
        {isCurrentMediaPlaying ? (
          <span>Playing ({formattedElapsed})</span>
        ) : (
          <>
            {isLoading ? (
              <LoaderCircle className="w-3.5 h-3.5 ml-0.5 animate-spin" />
            ) : (
              <Play className="w-3.5 h-3.5 fill-current ml-0.5" />
            )}
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
            {profiles.map((profile) => {
              const available = isProfileAvailable(profile);
              return (
                <Button
                  key={profile.id}
                  variant="outline"
                  disabled={isLoading || !available}
                  onClick={() => launch(profile.id)}
                  title={!available ? "Agent not connected" : undefined}
                >
                  {profile.name}
                </Button>
              );
            })}
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
