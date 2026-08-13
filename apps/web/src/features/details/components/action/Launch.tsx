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

type LaunchProps = {
  profiles: components["schemas"]["Media"]["launches"];
};

export const Launch = ({ profiles = [] }: LaunchProps) => {
  const [open, setOpen] = useState(false);
  const launchMediaMutation = useLaunchMedia({});

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
        aria-label="Play"
        className="inline-flex items-center gap-2 px-5 py-2 rounded-xl bg-accent-gold hover:bg-accent-gold-hover text-black font-extrabold text-xs sm:text-sm shadow-lg shadow-accent-gold/20 transition-all active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer"
      >
        <Play className="w-3.5 h-3.5 fill-current ml-0.5" />
        <span>Launch</span>
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
