import { useState } from "react";
import { Pencil, Plus, Star, Trash2 } from "lucide-react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { ConfirmDialog } from "@/components/feedback/ConfirmDialog";
import { useEmulators } from "@/features/settings/api/emulators";
import {
  useDeleteLaunchProfile,
  useUpdateLaunchProfile,
  type Launch,
} from "../../api/launches";
import { LaunchProfileFormDialog } from "./LaunchProfileFormDialog";

type ManageLaunchProfilesDialogProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  mediaId: string;
  profiles: Launch[];
};

export function ManageLaunchProfilesDialog({
  open,
  onOpenChange,
  mediaId,
  profiles,
}: ManageLaunchProfilesDialogProps) {
  const { data: emulatorsData } = useEmulators();
  const updateMutation = useUpdateLaunchProfile();
  const deleteMutation = useDeleteLaunchProfile();

  const [formOpen, setFormOpen] = useState(false);
  const [editingProfile, setEditingProfile] = useState<Launch | null>(null);
  const [deletingProfile, setDeletingProfile] = useState<Launch | null>(null);

  const emulatorsById = new Map(
    (emulatorsData?.items ?? []).map((e) => [e.id, e]),
  );

  const handleAdd = () => {
    setEditingProfile(null);
    setFormOpen(true);
  };

  const handleEdit = (profile: Launch) => {
    setEditingProfile(profile);
    setFormOpen(true);
  };

  const handleSetDefault = (profile: Launch) => {
    updateMutation.mutate({
      mediaId,
      launchId: profile.id,
      data: { is_default: true },
    });
  };

  const handleConfirmDelete = async () => {
    if (!deletingProfile) return;
    await deleteMutation.mutateAsync({
      mediaId,
      launchId: deletingProfile.id,
    });
    setDeletingProfile(null);
  };

  return (
    <>
      <Dialog open={open} onOpenChange={onOpenChange}>
        <DialogContent className="sm:max-w-lg">
          <DialogHeader>
            <DialogTitle>Launch Configurations</DialogTitle>
            <DialogDescription>
              Add emulator or custom launch profiles for this game and override
              settings per profile.
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-2.5 max-h-80 overflow-y-auto py-2">
            {profiles.length === 0 && (
              <div className="p-6 text-center text-sm text-text-muted rounded-xl border border-dashed border-border">
                No launch profiles configured yet.
              </div>
            )}

            {profiles.map((profile) => {
              const emulator = profile.emulator_id
                ? emulatorsById.get(profile.emulator_id)
                : null;

              return (
                <div
                  key={profile.id}
                  className="p-3 rounded-xl border border-border/60 bg-surface-raised/40 flex items-center justify-between gap-3"
                >
                  <div className="min-w-0 space-y-0.5">
                    <div className="flex items-center gap-2">
                      <span className="font-semibold text-sm text-foreground truncate">
                        {profile.name}
                      </span>
                      {profile.is_default && (
                        <Badge variant="secondary" className="text-[10px]">
                          Default
                        </Badge>
                      )}
                      {emulator && (
                        <Badge variant="outline" className="text-[10px]">
                          {emulator.name}
                        </Badge>
                      )}
                    </div>
                    {profile.game_location && (
                      <div className="text-xs text-text-muted font-mono truncate">
                        {profile.game_location}
                      </div>
                    )}
                  </div>

                  <div className="flex items-center gap-1 shrink-0">
                    {!profile.is_default && (
                      <Button
                        type="button"
                        variant="ghost"
                        size="sm"
                        title="Set as default"
                        aria-label={`Set ${profile.name} as default`}
                        onClick={() => handleSetDefault(profile)}
                        disabled={updateMutation.isPending}
                      >
                        <Star className="w-4 h-4" />
                      </Button>
                    )}
                    <Button
                      type="button"
                      variant="ghost"
                      size="sm"
                      aria-label={`Edit ${profile.name}`}
                      onClick={() => handleEdit(profile)}
                    >
                      <Pencil className="w-4 h-4" />
                    </Button>
                    <Button
                      type="button"
                      variant="ghost"
                      size="sm"
                      aria-label={`Delete ${profile.name}`}
                      onClick={() => setDeletingProfile(profile)}
                      className="text-destructive hover:text-destructive"
                    >
                      <Trash2 className="w-4 h-4" />
                    </Button>
                  </div>
                </div>
              );
            })}
          </div>

          <DialogFooter className="flex-row justify-between sm:justify-between gap-2">
            <Button type="button" onClick={handleAdd} className="gap-1.5">
              <Plus className="w-4 h-4" />
              Add Launch Profile
            </Button>
            <Button
              type="button"
              variant="ghost"
              onClick={() => onOpenChange(false)}
            >
              Done
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      <LaunchProfileFormDialog
        open={formOpen}
        onOpenChange={setFormOpen}
        mediaId={mediaId}
        profile={editingProfile}
      />

      <ConfirmDialog
        open={Boolean(deletingProfile)}
        onOpenChange={(nextOpen) => {
          if (!nextOpen) setDeletingProfile(null);
        }}
        title="Delete Launch Profile"
        description={`Are you sure you want to delete "${deletingProfile?.name}"?`}
        confirmLabel="Delete"
        variant="destructive"
        isLoading={deleteMutation.isPending}
        onConfirm={handleConfirmDelete}
      />
    </>
  );
}
