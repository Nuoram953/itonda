import { useState } from "react";
import { Cpu } from "lucide-react";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
} from "@/components/ui/sheet";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";
import { formatArgsString, parseArgsString } from "@/utils/launch-args";
import {
  useCreateEmulator,
  useUpdateEmulator,
  type Emulator,
} from "../../api/emulators";

type EmulatorDrawerProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  emulator: Emulator | null;
};

export function EmulatorDrawer({
  open,
  onOpenChange,
  emulator,
}: EmulatorDrawerProps) {
  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent
        side="right"
        className="w-full data-[side=right]:sm:max-w-xl bg-surface border-l border-white/10 p-0 flex flex-col justify-between overflow-hidden shadow-2xl"
      >
        {open && (
          <EmulatorDrawerForm
            key={emulator?.id ?? "new"}
            emulator={emulator}
            onClose={() => onOpenChange(false)}
          />
        )}
      </SheetContent>
    </Sheet>
  );
}

function EmulatorDrawerForm({
  emulator,
  onClose,
}: {
  emulator: Emulator | null;
  onClose: () => void;
}) {
  const createMutation = useCreateEmulator();
  const updateMutation = useUpdateEmulator();

  const [name, setName] = useState(emulator?.name ?? "");
  const [program, setProgram] = useState(emulator?.program ?? "");
  const [argsText, setArgsText] = useState(
    emulator ? formatArgsString(emulator.arguments) : "",
  );
  const [workingDirectory, setWorkingDirectory] = useState(
    emulator?.working_directory ?? "",
  );
  const [error, setError] = useState<string | null>(null);

  const isSubmitting = createMutation.isPending || updateMutation.isPending;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);

    const trimmedName = name.trim();
    const trimmedProgram = program.trim();
    if (!trimmedName) {
      setError("Emulator name is required.");
      return;
    }
    if (!trimmedProgram) {
      setError("Executable path or command is required.");
      return;
    }

    const parsedArgs = parseArgsString(argsText);
    const trimmedWorkingDir = workingDirectory.trim() || null;

    try {
      if (emulator) {
        await updateMutation.mutateAsync({
          id: emulator.id,
          data: {
            name: trimmedName,
            program: trimmedProgram,
            arguments: parsedArgs,
            working_directory: trimmedWorkingDir,
          },
        });
      } else {
        await createMutation.mutateAsync({
          name: trimmedName,
          program: trimmedProgram,
          arguments: parsedArgs,
          working_directory: trimmedWorkingDir,
        });
      }
      onClose();
    } catch (err) {
      setError(
        err instanceof Error ? err.message : "Failed to save emulator.",
      );
    }
  };

  return (
    <>
      <SheetHeader className="p-6 border-b border-white/10 bg-surface-raised/40">
        <div className="flex items-center gap-3.5">
          <div className="flex items-center justify-center size-12 rounded-2xl bg-cyan-500/10 border border-cyan-500/20 text-cyan-400 shadow-inner">
            <Cpu className="w-6 h-6" />
          </div>
          <div>
            <SheetTitle className="text-lg font-bold text-foreground">
              {emulator ? `Edit ${emulator.name}` : "Add Emulator"}
            </SheetTitle>
            <SheetDescription className="text-xs text-text-muted mt-0.5">
              Configure an emulator executable and default launch arguments.
            </SheetDescription>
          </div>
        </div>
      </SheetHeader>

      <form
        id="emulator-form"
        onSubmit={handleSubmit}
        className="flex-1 overflow-y-auto p-6 space-y-5"
      >
        {error && (
          <div className="p-3 rounded-lg bg-destructive/15 border border-destructive/30 text-destructive text-xs font-medium">
            {error}
          </div>
        )}

        <div className="space-y-2">
          <Label htmlFor="emulator-name">Name</Label>
          <Input
            id="emulator-name"
            placeholder="e.g. Dolphin, PCSX2, RetroArch"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>

        <div className="space-y-2">
          <Label htmlFor="emulator-program">Executable / Program</Label>
          <Input
            id="emulator-program"
            placeholder="e.g. /usr/bin/dolphin-emu or dolphin-emu"
            value={program}
            onChange={(e) => setProgram(e.target.value)}
          />
          <p className="text-xs text-text-muted">
            Full path or command available on the target agent machine.
          </p>
        </div>

        <div className="space-y-2">
          <Label htmlFor="emulator-args">Default Arguments</Label>
          <Input
            id="emulator-args"
            placeholder="e.g. -b -e {game}"
            value={argsText}
            onChange={(e) => setArgsText(e.target.value)}
          />
          <p className="text-xs text-text-muted">
            Use <code className="text-foreground">{"{game}"}</code> or{" "}
            <code className="text-foreground">{"{rom}"}</code> where the game
            location should be inserted. If omitted, the game location is
            appended at the end.
          </p>
        </div>

        <div className="space-y-2">
          <Label htmlFor="emulator-workdir">
            Default Working Directory (Optional)
          </Label>
          <Input
            id="emulator-workdir"
            placeholder="e.g. /home/user/emulators/dolphin"
            value={workingDirectory}
            onChange={(e) => setWorkingDirectory(e.target.value)}
          />
        </div>
      </form>

      <SheetFooter className="p-6 border-t border-white/10 bg-surface-raised/20 flex-row justify-end gap-2">
        <Button
          type="button"
          variant="ghost"
          onClick={onClose}
          disabled={isSubmitting}
        >
          Cancel
        </Button>
        <Button type="submit" form="emulator-form" disabled={isSubmitting}>
          {isSubmitting
            ? "Saving..."
            : emulator
              ? "Save Changes"
              : "Create Emulator"}
        </Button>
      </SheetFooter>
    </>
  );
}
