import type { components } from "@/api/generated.d";
import { Workspace } from "@/components/workspace/Workspace";
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
      <Workspace.Action
        disabled={!profiles.length}
        icon={Play}
        onClick={handleClick}
      >
        Play
      </Workspace.Action>

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
