import { useMemo, useState } from "react";
import { FolderOpen } from "lucide-react";
import { AgentFileBrowserDialog } from "./AgentFileBrowserDialog";
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
import { Badge } from "@/components/ui/badge";
import { useAgents } from "@/api/get-agents";
import { useEmulators, type Emulator } from "@/features/settings/api/emulators";
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

type OptionValue = boolean | string;

function formatOptionTokens(flag: string, value: OptionValue): string[] {
  if (typeof value === "boolean") {
    return value ? parseArgsString(flag) : [];
  }
  const trimmed = value.trim();
  if (!trimmed) return [];
  if (flag.includes("{value}")) {
    return parseArgsString(flag.replace(/\{value\}/g, trimmed));
  }
  return [...parseArgsString(flag), trimmed];
}

function buildInitialOptionsState(
  emulator: Emulator | undefined,
  profile: Launch | null,
): Record<string, OptionValue> {
  if (!emulator) return {};
  const state: Record<string, OptionValue> = {};
  const profileArgs = profile?.arguments ?? [];
  for (const opt of emulator.options ?? []) {
    const optType = opt.type ?? "flag";
    if (optType === "flag") {
      if (profile) {
        const flagTokens = parseArgsString(opt.flag);
        state[opt.key] =
          flagTokens.length > 0 &&
          flagTokens.every((tok) => profileArgs.includes(tok));
      } else {
        state[opt.key] = Boolean(opt.default_enabled);
      }
    } else if (profile) {
      let matched = "";
      if (opt.flag.includes("{value}")) {
        const [prefix, suffix] = opt.flag.split("{value}");
        const found = profileArgs.find(
          (a) => a.startsWith(prefix) && a.endsWith(suffix),
        );
        if (found) {
          matched = found.slice(
            prefix.length,
            suffix.length ? -suffix.length : undefined,
          );
        }
      } else {
        const flagTokens = parseArgsString(opt.flag);
        const idx = profileArgs.findIndex((a) => a === flagTokens[0]);
        if (idx !== -1 && profileArgs[idx + flagTokens.length]) {
          matched = profileArgs[idx + flagTokens.length];
        }
      }
      state[opt.key] = matched;
    } else {
      state[opt.key] = opt.default_value ?? "";
    }
  }
  return state;
}

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

  const initialEmulatorId = profile?.emulator_id ?? emulators[0]?.id ?? "";
  const initialEmulator = emulators.find((e) => e.id === initialEmulatorId);

  const [emulatorId, setEmulatorId] = useState<string>(initialEmulatorId);
  const [name, setName] = useState(
    profile?.name ?? initialEmulator?.name ?? "",
  );
  const [gameLocation, setGameLocation] = useState(
    profile?.game_location ?? "",
  );
  const [optionStates, setOptionStates] = useState<Record<string, OptionValue>>(
    () => buildInitialOptionsState(initialEmulator, profile),
  );
  const [program, setProgram] = useState(profile?.program ?? "");
  const [argsText, setArgsText] = useState(() => {
    if (!profile) return "";
    // Strip option tokens already represented by option controls so they don't duplicate
    const initialOpts = buildInitialOptionsState(initialEmulator, profile);
    const optionTokens = new Set(
      (initialEmulator?.options ?? []).flatMap((opt) =>
        formatOptionTokens(opt.flag, initialOpts[opt.key] ?? ""),
      ),
    );
    const remaining = (profile.arguments ?? []).filter(
      (arg) => !optionTokens.has(arg),
    );
    return formatArgsString(remaining);
  });
  const [argumentMode, setArgumentMode] = useState<"append" | "replace">(
    profile?.argument_mode === "replace" ? "replace" : "append",
  );
  const [workingDirectory, setWorkingDirectory] = useState(
    profile?.working_directory ?? "",
  );
  const [agentId, setAgentId] = useState<string>(profile?.agent_id ?? "");
  const [isDefault, setIsDefault] = useState(profile?.is_default ?? false);
  const [fileBrowserOpen, setFileBrowserOpen] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const selectedEmulator = emulators.find((e) => e.id === emulatorId) ?? null;
  const isEmulatorMode = Boolean(emulatorId);

  const handleEmulatorChange = (nextEmulatorId: string) => {
    const prevEmu = emulators.find((e) => e.id === emulatorId);
    const nextEmu = emulators.find((e) => e.id === nextEmulatorId);
    setEmulatorId(nextEmulatorId);
    setOptionStates(buildInitialOptionsState(nextEmu, null));
    if (!profile && (!name || name === prevEmu?.name)) {
      setName(nextEmu?.name ?? "");
    }
  };

  const selectedOptionArgs = useMemo(() => {
    if (!selectedEmulator) return [];
    const tokens: string[] = [];
    for (const opt of selectedEmulator.options ?? []) {
      const val = optionStates[opt.key];
      if (val !== undefined) {
        tokens.push(...formatOptionTokens(opt.flag, val));
      }
    }
    return tokens;
  }, [selectedEmulator, optionStates]);

  const commandPreview = useMemo(() => {
    const exe =
      program.trim() || selectedEmulator?.program || "<executable>";
    const extra = parseArgsString(argsText);
    const launchArgs = [...selectedOptionArgs, ...extra];
    const allArgs: string[] = [];
    if (isEmulatorMode && argumentMode === "append" && !profile) {
      let replacedArgs = false;
      for (const arg of selectedEmulator?.arguments ?? []) {
        if (arg === "{args}") {
          allArgs.push(...launchArgs);
          replacedArgs = true;
        } else if (arg.includes("{args}")) {
          const replaced = arg.replace("{args}", launchArgs.join(" "));
          if (replaced) {
            allArgs.push(replaced);
          }
          replacedArgs = true;
        } else {
          allArgs.push(arg);
        }
      }
      if (!replacedArgs) {
        allArgs.push(...launchArgs);
      }
    } else {
      allArgs.push(...launchArgs);
    }
    const loc = gameLocation.trim();
    if (loc) {
      let replaced = false;
      for (let i = 0; i < allArgs.length; i++) {
        if (allArgs[i].includes("{game}") || allArgs[i].includes("{rom}")) {
          allArgs[i] = allArgs[i]
            .replace("{game}", loc)
            .replace("{rom}", loc);
          replaced = true;
        }
      }
      if (!replaced && !allArgs.includes(loc)) {
        allArgs.push(loc);
      }
    }
    return [exe, ...allArgs].join(" ");
  }, [
    program,
    selectedEmulator,
    argsText,
    isEmulatorMode,
    argumentMode,
    profile,
    selectedOptionArgs,
    gameLocation,
  ]);

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
      setError(
        "Game location (ROM/ISO path) is required for emulator launches.",
      );
      return;
    }
    if (!isEmulatorMode && !trimmedProgram) {
      setError("Executable / program path is required for custom launches.");
      return;
    }

    const parsedExtraArgs = parseArgsString(argsText);
    const combinedLaunchArgs = [...selectedOptionArgs, ...parsedExtraArgs];
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
            arguments: combinedLaunchArgs,
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
            arguments: combinedLaunchArgs,
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

  const emulatorOptions = selectedEmulator?.options ?? [];

  return (
    <>
      <DialogHeader>
        <DialogTitle>
          {profile ? `Edit "${profile.name}"` : "Add Launch Configuration"}
        </DialogTitle>
        <DialogDescription>
          Configure an emulator or custom executable for this game and toggle
          launch options.
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
            placeholder="e.g. Dolphin, PCSX2 (Fullscreen)"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>

        <div className="space-y-1.5">
          <Label htmlFor="lp-game-location">
            Game Location {isEmulatorMode ? "(Required)" : "(Optional)"}
          </Label>
          <div className="flex items-center gap-2">
            <Input
              id="lp-game-location"
              placeholder="e.g. /roms/ps2/Final Fantasy X.iso"
              value={gameLocation}
              onChange={(e) => setGameLocation(e.target.value)}
            />
            <Button
              type="button"
              variant="outline"
              size="sm"
              className="shrink-0"
              onClick={() => setFileBrowserOpen(true)}
            >
              <FolderOpen className="h-4 w-4 mr-1.5" />
              Browse
            </Button>
          </div>
        </div>

        {isEmulatorMode && emulatorOptions.length > 0 && (
          <div className="rounded-xl border border-border/60 bg-surface-raised/30 p-4 space-y-3">
            <div className="text-xs font-semibold uppercase tracking-wider text-text-muted">
              Options
            </div>
            {emulatorOptions.map((opt) => {
              const optType = opt.type ?? "flag";
              if (optType === "flag") {
                return (
                  <label
                    key={opt.key}
                    className="flex items-center justify-between gap-3 py-1 cursor-pointer"
                  >
                    <div className="min-w-0">
                      <div className="flex items-center gap-2">
                        <span className="text-sm font-medium text-foreground">
                          {opt.label}
                        </span>
                        <Badge
                          variant="outline"
                          className="font-mono text-[10px]"
                        >
                          {opt.flag}
                        </Badge>
                      </div>
                      {opt.description && (
                        <p className="text-xs text-text-muted">
                          {opt.description}
                        </p>
                      )}
                    </div>
                    <Switch
                      checked={Boolean(optionStates[opt.key])}
                      onCheckedChange={(checked) =>
                        setOptionStates((prev) => ({
                          ...prev,
                          [opt.key]: checked,
                        }))
                      }
                    />
                  </label>
                );
              }

              if (optType === "select") {
                return (
                  <div key={opt.key} className="space-y-1">
                    <div className="flex items-center gap-2">
                      <Label htmlFor={`opt-${opt.key}`}>{opt.label}</Label>
                      <Badge
                        variant="outline"
                        className="font-mono text-[10px]"
                      >
                        {opt.flag}
                      </Badge>
                    </div>
                    {opt.description && (
                      <p className="text-xs text-text-muted">
                        {opt.description}
                      </p>
                    )}
                    <select
                      id={`opt-${opt.key}`}
                      className={SELECT_CLASS}
                      value={String(optionStates[opt.key] ?? "")}
                      onChange={(e) =>
                        setOptionStates((prev) => ({
                          ...prev,
                          [opt.key]: e.target.value,
                        }))
                      }
                    >
                      <option value="">Default / Unset</option>
                      {(opt.choices ?? []).map((choice) => (
                        <option key={choice} value={choice}>
                          {choice}
                        </option>
                      ))}
                    </select>
                  </div>
                );
              }

              return (
                <div key={opt.key} className="space-y-1">
                  <div className="flex items-center gap-2">
                    <Label htmlFor={`opt-${opt.key}`}>{opt.label}</Label>
                    <Badge variant="outline" className="font-mono text-[10px]">
                      {opt.flag}
                    </Badge>
                  </div>
                  {opt.description && (
                    <p className="text-xs text-text-muted">
                      {opt.description}
                    </p>
                  )}
                  <Input
                    id={`opt-${opt.key}`}
                    placeholder={opt.default_value ?? "Leave blank to omit"}
                    value={String(optionStates[opt.key] ?? "")}
                    onChange={(e) =>
                      setOptionStates((prev) => ({
                        ...prev,
                        [opt.key]: e.target.value,
                      }))
                    }
                  />
                </div>
              );
            })}
          </div>
        )}

        <div className="rounded-xl border border-border/60 bg-surface-raised/30 p-4 space-y-3">
          <div className="text-xs font-semibold uppercase tracking-wider text-text-muted">
            {isEmulatorMode ? "Overrides (Optional)" : "Execution Settings"}
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
              {isEmulatorMode ? "Extra Arguments (Optional)" : "Arguments"}
            </Label>
            <Input
              id="lp-args"
              placeholder={
                selectedEmulator && selectedEmulator.arguments.length > 0
                  ? `Emulator defaults: ${formatArgsString(selectedEmulator.arguments)}`
                  : "e.g. --batch"
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
                  : "Defaults to game folder"
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

        <div className="rounded-xl border border-border/60 bg-background/60 p-3 space-y-1">
          <div className="text-[11px] font-semibold uppercase tracking-wider text-text-muted">
            Command Preview
          </div>
          <div className="text-xs font-mono text-foreground break-all">
            {commandPreview}
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

      <AgentFileBrowserDialog
        open={fileBrowserOpen}
        onOpenChange={setFileBrowserOpen}
        agentId={agentId || undefined}
        initialPath={gameLocation}
        onSelect={(path) => setGameLocation(path)}
      />
    </>
  );
}
