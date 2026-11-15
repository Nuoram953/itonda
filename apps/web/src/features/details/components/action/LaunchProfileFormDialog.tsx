import { useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { useAgents } from "@/api/get-agents";
import { useEmulators } from "@/features/settings/api/emulators";
import { formatArgsString, parseArgsString } from "@/utils/launch-args";
import {
  useCreateLaunchProfile,
  useUpdateLaunchProfile,
  type Launch,
} from "../../api/launches";

type LaunchProfileFormDialogProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  mediaId: string;
  profile: Launch | null;
};

const SELECT_CLASS =
  "flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm text-foreground shadow-xs transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring";

export function LaunchProfileFormDialog({
  open,
  onOpenChange,
  mediaId,
  profile,
}: LaunchProfileFormDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-lg max-h-[90vh] overflow-y-auto">
        {open && (
          <LaunchProfileFormBody
            key={profile?.id ?? "new"}
            mediaId={mediaId}
            profile={profile}
            onClose={() => onOpenChange(false)}
          />
        )}
      </DialogContent>
    </Dialog>
  );
}

function LaunchProfileFormBody({
  mediaId,
  profile,
  onClose,
}: {
  mediaId: string;
  profile: Launch | null;
  onClose: () => void;
}) {
  const { data: emulatorsData } = useEmulators();
  const { data: agentsData } = useAgents();
  const createMutation = useCreateLaunchProfile();
  const updateMutation = useUpdateLaunchProfile();

  const emulators = emulatorsData?.items ?? [];
  const agents = agentsData?.agents ?? [];

  const [emulatorId, setEmulatorId] = useState<string>(
    profile?.emulator_id ?? emulators[0]?.id ?? "",
  );
  const [name, setName] = useState(
    profile?.name ?? emulators[0]?.name ?? "",
  );
  const [gameLocation, setGameLocation] = useState(
    profile?.game_location ?? "",
  );
  const [program, setProgram] = useState(profile?.program ?? "");
  const [argsText, setArgsText] = useState(
    profile ? formatArgsString(profile.arguments ?? []) : "",
  );
  const [argumentMode, setArgumentMode] = useState<"append" | "replace">(
    profile?.argument_mode === "replace" ? "replace" : "append",
  );
  const [workingDirectory, setWorkingDirectory] = useState(
    profile?.working_directory ?? "",
  );
  const [agentId, setAgentId] = useState<string>(profile?.agent_id ?? "");
  const [isDefault, setIsDefault] = useState(profile?.is_default ?? false);
  const [error, setError] = useState<string | null>(null);

  const selectedEmulator = emulators.find((e) => e.id === emulatorId) ?? null;
  const isEmulatorMode = Boolean(emulatorId);

  const handleEmulatorChange = (nextEmulatorId: string) => {
    const prevEmu = emulators.find((e) => e.id === emulatorId);
    const nextEmu = emulators.find((e) => e.id === nextEmulatorId);
    setEmulatorId(nextEmulatorId);
    if (!profile && (!name || name === prevEmu?.name)) {
      setName(nextEmu?.name ?? "");
    }
  };

  const isSubmitting = createMutation.isPending || updateMutation.isPending;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);

    const trimmedName = name.trim();
    const trimmedGameLocation = gameLocation.trim();
    const trimmedProgram = program.trim();

    if (!trimmedName) {
      setError("Profile name is required.");
      return;
    }
    if (isEmulatorMode && !trimmedGameLocation) {
      setError("Game location (ROM/ISO path) is required for emulator launches.");
      return;
    }
    if (!isEmulatorMode && !trimmedProgram) {
      setError("Executable / program path is required for custom launches.");
      return;
    }

    const parsedArgs = parseArgsString(argsText);
    const trimmedWorkDir = workingDirectory.trim() || null;
    const selectedAgent = agentId.trim() || null;

    try {
      if (profile) {
        await updateMutation.mutateAsync({
          mediaId,
          launchId: profile.id,
          data: {
            name: trimmedName,
            launch_type: isEmulatorMode ? "emulator" : "custom",
            emulator_id: isEmulatorMode ? emulatorId : null,
            game_location: trimmedGameLocation || null,
            program: trimmedProgram,
            arguments: parsedArgs,
            argument_mode: argumentMode,
            working_directory: trimmedWorkDir,
            agent_id: selectedAgent,
            is_default: isDefault,
          },
        });
      } else {
        await createMutation.mutateAsync({
          mediaId,
          data: {
            name: trimmedName,
            launch_type: isEmulatorMode ? "emulator" : "custom",
            emulator_id: isEmulatorMode ? emulatorId : null,
            game_location: trimmedGameLocation || null,
            program: trimmedProgram,
            arguments: parsedArgs,
            argument_mode: argumentMode,
            working_directory: trimmedWorkDir,
            agent_id: selectedAgent,
            is_default: isDefault,
            enabled: true,
          },
        });
      }
      onClose();
    } catch (err) {
      setError(
        err instanceof Error ? err.message : "Failed to save launch profile.",
      );
    }
  };

  return (
    <>
      <DialogHeader>
        <DialogTitle>
          {profile ? `Edit "${profile.name}"` : "Add Launch Configuration"}
        </DialogTitle>
        <DialogDescription>
          Configure an emulator or custom executable for this game and override
          settings as needed.
        </DialogDescription>
      </DialogHeader>

      <form
        id="launch-profile-form"
        onSubmit={handleSubmit}
        className="space-y-4 py-2"
      >
        {error && (
          <div className="p-3 rounded-lg bg-destructive/15 border border-destructive/30 text-destructive text-xs font-medium">
            {error}
          </div>
        )}

        <div className="space-y-1.5">
          <Label htmlFor="lp-emulator">Emulator</Label>
          <select
            id="lp-emulator"
            className={SELECT_CLASS}
            value={emulatorId}
            onChange={(e) => handleEmulatorChange(e.target.value)}
          >
            <option value="">Custom Executable (No Emulator)</option>
            {emulators.map((emu) => (
              <option key={emu.id} value={emu.id}>
                {emu.name} ({emu.program})
              </option>
            ))}
          </select>
        </div>

        <div className="space-y-1.5">
          <Label htmlFor="lp-name">Profile Name</Label>
          <Input
            id="lp-name"
            placeholder="e.g. Dolphin, Dolphin (4K), Direct Launch"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>

        <div className="space-y-1.5">
          <Label htmlFor="lp-game-location">
            Game Location {isEmulatorMode ? "(Required)" : "(Optional)"}
          </Label>
          <Input
            id="lp-game-location"
            placeholder="e.g. /roms/gamecube/Super Mario Sunshine.iso"
            value={gameLocation}
            onChange={(e) => setGameLocation(e.target.value)}
          />
        </div>

        <div className="rounded-xl border border-border/60 bg-surface-raised/30 p-4 space-y-3">
          <div className="text-xs font-semibold uppercase tracking-wider text-text-muted">
            {isEmulatorMode ? "Emulator Overrides" : "Execution Settings"}
          </div>

          <div className="space-y-1.5">
            <Label htmlFor="lp-arg-mode">Arguments Mode</Label>
            <select
              id="lp-arg-mode"
              className={SELECT_CLASS}
              value={argumentMode}
              onChange={(e) =>
                setArgumentMode(e.target.value as "append" | "replace")
              }
              disabled={!isEmulatorMode}
            >
              <option value="append">
                Append to emulator default arguments
              </option>
              <option value="replace">
                Replace emulator default arguments
              </option>
            </select>
          </div>

          <div className="space-y-1.5">
            <Label htmlFor="lp-args">
              {isEmulatorMode ? "Extra / Override Arguments" : "Arguments"}
            </Label>
            <Input
              id="lp-args"
              placeholder={
                selectedEmulator && selectedEmulator.arguments.length > 0
                  ? `Emulator defaults: ${formatArgsString(selectedEmulator.arguments)}`
                  : "e.g. --fullscreen"
              }
              value={argsText}
              onChange={(e) => setArgsText(e.target.value)}
            />
          </div>

          <div className="space-y-1.5">
            <Label htmlFor="lp-program">
              {isEmulatorMode
                ? "Executable Override (Optional)"
                : "Executable / Program"}
            </Label>
            <Input
              id="lp-program"
              placeholder={
                selectedEmulator
                  ? `Default: ${selectedEmulator.program}`
                  : "e.g. /path/to/game.exe"
              }
              value={program}
              onChange={(e) => setProgram(e.target.value)}
            />
          </div>

          <div className="space-y-1.5">
            <Label htmlFor="lp-workdir">
              Working Directory Override (Optional)
            </Label>
            <Input
              id="lp-workdir"
              placeholder={
                selectedEmulator?.working_directory
                  ? `Default: ${selectedEmulator.working_directory}`
                  : "e.g. /path/to/working/dir"
              }
              value={workingDirectory}
              onChange={(e) => setWorkingDirectory(e.target.value)}
            />
          </div>

          <div className="space-y-1.5">
            <Label htmlFor="lp-agent">Target Agent Override (Optional)</Label>
            <select
              id="lp-agent"
              className={SELECT_CLASS}
              value={agentId}
              onChange={(e) => setAgentId(e.target.value)}
            >
              <option value="">Any available agent</option>
              {agents.map((agent) => (
                <option key={agent.id} value={agent.id}>
                  {agent.name} ({agent.is_connected ? "Online" : "Offline"})
                </option>
              ))}
            </select>
          </div>
        </div>

        <div className="flex items-center justify-between pt-1">
          <div>
            <Label htmlFor="lp-default" className="cursor-pointer">
              Set as Default Launch Profile
            </Label>
            <p className="text-xs text-text-muted">
              Launch immediately with this profile when clicking Play.
            </p>
          </div>
          <Switch
            id="lp-default"
            checked={isDefault}
            onCheckedChange={setIsDefault}
          />
        </div>
      </form>

      <DialogFooter>
        <Button
          type="button"
          variant="ghost"
          onClick={onClose}
          disabled={isSubmitting}
        >
          Cancel
        </Button>
        <Button
          type="submit"
          form="launch-profile-form"
          disabled={isSubmitting}
        >
          {isSubmitting
            ? "Saving..."
            : profile
              ? "Save Changes"
              : "Add Profile"}
        </Button>
      </DialogFooter>
    </>
  );
}
