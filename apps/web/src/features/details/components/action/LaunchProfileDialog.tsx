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

export type LaunchProfile = components["schemas"]["Launch"];

type LaunchProfileDialogProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  title?: string;
  profiles: LaunchProfile[];
  isProfileAvailable: (profile: LaunchProfile) => boolean;
  onSelectProfile: (profileId: string) => void;
  isLoading?: boolean;
};

export function LaunchProfileDialog({
  open,
  onOpenChange,
  title,
  profiles,
  isProfileAvailable,
  onSelectProfile,
  isLoading = false,
}: LaunchProfileDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Select launch profile</DialogTitle>
          <DialogDescription>
            {title
              ? `Choose how you want to launch ${title}.`
              : "Choose how you want to launch this game."}
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
                onClick={() => onSelectProfile(profile.id)}
                title={!available ? "Agent not connected" : undefined}
              >
                {profile.name}
              </Button>
            );
          })}
        </div>

        <DialogFooter>
          <Button variant="ghost" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
