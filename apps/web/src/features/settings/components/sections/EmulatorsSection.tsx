import { useState } from "react";
import { Cpu, Pencil, Plus, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { ConfirmDialog } from "@/components/feedback/ConfirmDialog";
import { formatArgsString } from "@/utils/launch-args";
import {
  useDeleteEmulator,
  useEmulators,
  type Emulator,
} from "../../api/emulators";
import { SettingsSection } from "./SettingsSection";
import { EmulatorDrawer } from "../drawers/EmulatorDrawer";

export function EmulatorsSection() {
  const { data, isPending } = useEmulators();
  const deleteMutation = useDeleteEmulator();

  const [drawerOpen, setDrawerOpen] = useState(false);
  const [editingEmulator, setEditingEmulator] = useState<Emulator | null>(null);
  const [deletingEmulator, setDeletingEmulator] = useState<Emulator | null>(
    null,
  );

  const emulators = data?.items ?? [];

  const handleAdd = () => {
    setEditingEmulator(null);
    setDrawerOpen(true);
  };

  const handleEdit = (emulator: Emulator) => {
    setEditingEmulator(emulator);
    setDrawerOpen(true);
  };

  const handleConfirmDelete = async () => {
    if (!deletingEmulator) return;
    await deleteMutation.mutateAsync(deletingEmulator.id);
    setDeletingEmulator(null);
  };

  return (
    <>
      <SettingsSection
        title="Emulators"
        description="Configure emulators and their default executables and launch flags. You can attach them to games and override settings per game."
      >
        <div className="p-4 flex items-center justify-between border-b border-border/60 bg-surface-raised/20">
          <span className="text-xs font-medium text-text-muted">
            {isPending
              ? "Loading emulators..."
              : `${emulators.length} emulator${emulators.length === 1 ? "" : "s"} configured`}
          </span>
          <Button size="sm" onClick={handleAdd} className="gap-1.5">
            <Plus className="w-4 h-4" />
            Add Emulator
          </Button>
        </div>

        {!isPending && emulators.length === 0 && (
          <div className="p-8 text-center text-sm text-text-muted">
            No emulators configured yet. Click <strong>Add Emulator</strong> to
            get started.
          </div>
        )}

        {emulators.map((emu) => (
          <div
            key={emu.id}
            className="p-4 flex items-center justify-between gap-4 hover:bg-surface-hover/30 transition-colors"
          >
            <div className="flex items-start gap-3.5 min-w-0">
              <div className="flex items-center justify-center size-10 rounded-xl bg-cyan-500/10 border border-cyan-500/20 text-cyan-400 shrink-0">
                <Cpu className="w-5 h-5" />
              </div>
              <div className="min-w-0 space-y-0.5">
                <div className="font-semibold text-sm text-foreground truncate">
                  {emu.name}
                </div>
                <div className="text-xs text-text-muted font-mono truncate">
                  {emu.program}
                  {emu.arguments.length > 0 &&
                    ` ${formatArgsString(emu.arguments)}`}
                </div>
                {emu.working_directory && (
                  <div className="text-xs text-text-muted truncate">
                    Working dir:{" "}
                    <span className="font-mono">{emu.working_directory}</span>
                  </div>
                )}
              </div>
            </div>

            <div className="flex items-center gap-1.5 shrink-0">
              <Button
                type="button"
                variant="ghost"
                size="sm"
                aria-label={`Edit ${emu.name}`}
                onClick={() => handleEdit(emu)}
              >
                <Pencil className="w-4 h-4" />
              </Button>
              <Button
                type="button"
                variant="ghost"
                size="sm"
                aria-label={`Delete ${emu.name}`}
                onClick={() => setDeletingEmulator(emu)}
                className="text-destructive hover:text-destructive"
              >
                <Trash2 className="w-4 h-4" />
              </Button>
            </div>
          </div>
        ))}
      </SettingsSection>

      <EmulatorDrawer
        open={drawerOpen}
        onOpenChange={setDrawerOpen}
        emulator={editingEmulator}
      />

      <ConfirmDialog
        open={Boolean(deletingEmulator)}
        onOpenChange={(open) => {
          if (!open) setDeletingEmulator(null);
        }}
        title="Delete Emulator"
        description={`Are you sure you want to delete "${deletingEmulator?.name}"? Any game launch profiles using this emulator will also be removed.`}
        confirmLabel="Delete"
        variant="destructive"
        isLoading={deleteMutation.isPending}
        onConfirm={handleConfirmDelete}
      />
    </>
  );
}
