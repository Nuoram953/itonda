import { useState } from "react";
import { ArrowLeft, ChevronDown, ChevronRight, Cpu, Pencil, Plus, Trash2 } from "lucide-react";
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
import { Switch } from "@/components/ui/switch";
import { Form } from "@/components/ui/form";
import { ConfirmDialog } from "@/components/feedback/ConfirmDialog";
import { SettingRow } from "../cards/SettingRow";
import { formatArgsString, parseArgsString } from "@/utils/launch-args";
import {
  useCreateEmulator,
  useDeleteEmulator,
  useEmulatorDefinitions,
  useEmulators,
  useUpdateEmulator,
  type Emulator,
  type EmulatorDefinition,
} from "../../api/emulators";
import type { components } from "@/api/generated.d";

type EmulatorOption = components["schemas"]["EmulatorOption"];

type EmulatorDrawerProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
};

export function EmulatorDrawer({ open, onOpenChange }: EmulatorDrawerProps) {
  const [editingEmulator, setEditingEmulator] = useState<
    Emulator | "new" | null
  >(null);

  const handleOpenChange = (nextOpen: boolean) => {
    if (!nextOpen) {
      setEditingEmulator(null);
    }
    onOpenChange(nextOpen);
  };

  return (
    <Sheet open={open} onOpenChange={handleOpenChange}>
      <SheetContent
        side="right"
        className="w-full data-[side=right]:sm:max-w-2xl data-[side=right]:lg:max-w-3xl bg-surface border-l border-white/10 p-0 flex flex-col justify-between overflow-hidden shadow-2xl"
      >
        {open &&
          (editingEmulator === null ? (
            <EmulatorListView
              onAdd={() => setEditingEmulator("new")}
              onEdit={(emu) => setEditingEmulator(emu)}
              onClose={() => handleOpenChange(false)}
            />
          ) : (
            <EmulatorFormView
              key={editingEmulator === "new" ? "new" : editingEmulator.id}
              emulator={editingEmulator === "new" ? null : editingEmulator}
              onBack={() => setEditingEmulator(null)}
            />
          ))}
      </SheetContent>
    </Sheet>
  );
}

function EmulatorListView({
  onAdd,
  onEdit,
  onClose,
}: {
  onAdd: () => void;
  onEdit: (emulator: Emulator) => void;
  onClose: () => void;
}) {
  const { data, isPending } = useEmulators();
  const deleteMutation = useDeleteEmulator();
  const [deletingEmulator, setDeletingEmulator] = useState<Emulator | null>(
    null,
  );

  const emulators = data?.items ?? [];

  const handleConfirmDelete = async () => {
    if (!deletingEmulator) return;
    await deleteMutation.mutateAsync(deletingEmulator.id);
    setDeletingEmulator(null);
  };

  return (
    <>
      <SheetHeader className="p-6 border-b border-white/10 bg-surface-raised/40">
        <div className="flex items-center justify-between gap-3 pr-6">
          <div className="flex items-center gap-3.5">
            <div className="flex items-center justify-center size-12 rounded-2xl bg-cyan-500/10 border border-cyan-500/20 text-cyan-400 shadow-inner">
              <Cpu className="w-6 h-6" />
            </div>
            <div>
              <SheetTitle className="text-lg font-bold text-foreground">
                Emulators
              </SheetTitle>
              <SheetDescription className="text-xs text-text-muted mt-0.5">
                Add, edit, and remove emulator configurations.
              </SheetDescription>
            </div>
          </div>

          <Button size="sm" onClick={onAdd} className="gap-1.5 shrink-0">
            <Plus className="w-4 h-4" />
            Add Emulator
          </Button>
        </div>
      </SheetHeader>

      <div className="flex-1 overflow-y-auto p-6 space-y-3">
        {isPending && (
          <div className="py-10 text-center text-sm text-text-muted">
            Loading emulators...
          </div>
        )}

        {!isPending && emulators.length === 0 && (
          <div className="p-8 text-center text-sm text-text-muted rounded-2xl border border-dashed border-white/10">
            No emulators configured yet. Click <strong>Add Emulator</strong> to
            create one.
          </div>
        )}

        {emulators.map((emu) => (
          <div
            key={emu.id}
            className="p-4 rounded-2xl border border-white/10 bg-surface-raised/40 flex items-center justify-between gap-3"
          >
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

            <div className="flex items-center gap-1 shrink-0">
              <Button
                type="button"
                variant="ghost"
                size="icon-sm"
                aria-label={`Edit ${emu.name}`}
                onClick={() => onEdit(emu)}
              >
                <Pencil className="w-4 h-4" />
              </Button>
              <Button
                type="button"
                variant="ghost"
                size="icon-sm"
                aria-label={`Delete ${emu.name}`}
                onClick={() => setDeletingEmulator(emu)}
                className="text-destructive hover:text-destructive"
              >
                <Trash2 className="w-4 h-4" />
              </Button>
            </div>
          </div>
        ))}
      </div>

      <SheetFooter className="p-6 border-t border-white/10 bg-surface-raised/20 flex-row justify-end">
        <Button type="button" variant="ghost" onClick={onClose}>
          Done
        </Button>
      </SheetFooter>

      <ConfirmDialog
        open={Boolean(deletingEmulator)}
        onOpenChange={(nextOpen) => {
          if (!nextOpen) setDeletingEmulator(null);
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

function EmulatorFormView({
  emulator,
  onBack,
}: {
  emulator: Emulator | null;
  onBack: () => void;
}) {
  const { data: definitionsData } = useEmulatorDefinitions();
  const createMutation = useCreateEmulator();
  const updateMutation = useUpdateEmulator();

  const definitions = definitionsData?.items ?? [];

  const [selectedDefId, setSelectedDefId] = useState("");
  const [name, setName] = useState(emulator?.name ?? "");
  const [program, setProgram] = useState(emulator?.program ?? "");
  const [argsText, setArgsText] = useState(
    emulator ? formatArgsString(emulator.arguments) : "",
  );
  const [options, setOptions] = useState<EmulatorOption[]>(
    emulator?.options ?? [],
  );
  const [collapsedKeys, setCollapsedKeys] = useState<Record<string, boolean>>(
    () => Object.fromEntries((emulator?.options ?? []).map((o) => [o.key, true])),
  );
  const [workingDirectory, setWorkingDirectory] = useState(
    emulator?.working_directory ?? "",
  );
  const [error, setError] = useState<string | null>(null);

  const isSubmitting = createMutation.isPending || updateMutation.isPending;

  const applyDefinition = (def: EmulatorDefinition) => {
    setName(def.name);
    setProgram(def.default_program);
    setArgsText(formatArgsString(def.default_arguments));
    setOptions(def.options);
    setCollapsedKeys(
      Object.fromEntries(def.options.map((o) => [o.key, true])),
    );
  };

  const toggleOptionCollapsed = (key: string) => {
    setCollapsedKeys((prev) => ({ ...prev, [key]: !prev[key] }));
  };

  const handleDefinitionChange = (defId: string) => {
    setSelectedDefId(defId);
    const def = definitions.find((d) => d.id === defId);
    if (def) {
      applyDefinition(def);
    }
  };

  const addOption = (preset?: Partial<EmulatorOption>) => {
    const key = `opt_${Date.now()}_${options.length}`;
    setOptions((prev) => [
      ...prev,
      {
        key: preset?.key ?? key,
        label: preset?.label ?? "",
        flag: preset?.flag ?? "",
        default_enabled: preset?.default_enabled ?? false,
      },
    ]);
  };

  const updateOption = (index: number, patch: Partial<EmulatorOption>) => {
    setOptions((prev) =>
      prev.map((opt, i) => (i === index ? { ...opt, ...patch } : opt)),
    );
  };

  const removeOption = (index: number) => {
    setOptions((prev) => prev.filter((_, i) => i !== index));
  };

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

    const cleanedOptions = options
      .map((opt) => ({
        ...opt,
        key: opt.key.trim() || opt.label.trim().toLowerCase().replace(/\s+/g, "_"),
        label: opt.label.trim(),
        flag: opt.flag.trim(),
      }))
      .filter((opt) => opt.label.length > 0 && opt.flag.length > 0);

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
            options: cleanedOptions,
            working_directory: trimmedWorkingDir,
          },
        });
      } else {
        await createMutation.mutateAsync({
          name: trimmedName,
          program: trimmedProgram,
          arguments: parsedArgs,
          options: cleanedOptions,
          working_directory: trimmedWorkingDir,
        });
      }
      onBack();
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
          <Button
            type="button"
            variant="ghost"
            size="icon-sm"
            onClick={onBack}
            aria-label="Back to emulators list"
          >
            <ArrowLeft className="w-4 h-4" />
          </Button>
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

        {definitions.length > 0 && (
          <div className="space-y-2">
            <Label htmlFor="emulator-preset">
              {emulator ? "Reload from Definition File" : "Emulator Preset"}
            </Label>
            <select
              id="emulator-preset"
              className="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm text-foreground shadow-xs transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
              value={selectedDefId}
              onChange={(e) => handleDefinitionChange(e.target.value)}
            >
              <option value="">
                {emulator ? "Select a definition to apply..." : "Custom Emulator"}
              </option>
              {definitions.map((def) => (
                <option key={def.id} value={def.id}>
                  {def.name} ({def.options.length} options)
                </option>
              ))}
            </select>
            <p className="text-xs text-text-muted">
              Loaded from <code className="text-foreground">emulators/*.toml</code>{" "}
              definition files in your config folder.
            </p>
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
            location should be inserted, and{" "}
            <code className="text-foreground">{"{args}"}</code> where launch
            profile arguments should be inserted. If omitted, they are appended
            at the end.
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

        <div className="space-y-3 pt-1">
          <div className="flex items-center justify-between gap-2">
            <div>
              <Label>Toggleable Options</Label>
              <p className="text-xs text-text-muted">
                Expose flags as checkboxes when adding this emulator to a game.
              </p>
            </div>
            <div className="flex items-center gap-1.5 shrink-0">
              <Button
                type="button"
                variant="outline"
                size="sm"
                onClick={() => addOption()}
              >
                <Plus className="w-3.5 h-3.5 mr-1" />
                Add Option
              </Button>
            </div>
          </div>

          <div className="space-y-3">
            {options.map((opt, index) => {
              const optionType = opt.type ?? "flag";
              const isCollapsed = Boolean(collapsedKeys[opt.key]);
              const labelId = `opt-${opt.key}-label`;
              const flagId = `opt-${opt.key}-flag`;
              const choicesId = `opt-${opt.key}-choices`;
              const defaultId = `opt-${opt.key}-default`;

              return (
                <div
                  key={opt.key}
                  className="rounded-xl border border-white/10 bg-surface-raised/30 overflow-hidden"
                >
                  <button
                    type="button"
                    onClick={() => toggleOptionCollapsed(opt.key)}
                    aria-expanded={!isCollapsed}
                    className="w-full flex items-center justify-between gap-3 p-4 text-left hover:bg-white/[0.02] transition-colors cursor-pointer"
                  >
                    <div className="flex items-center gap-2.5 min-w-0">
                      {isCollapsed ? (
                        <ChevronRight className="w-4 h-4 text-text-muted shrink-0" />
                      ) : (
                        <ChevronDown className="w-4 h-4 text-text-muted shrink-0" />
                      )}
                      <span className="text-sm font-semibold text-foreground truncate">
                        {opt.label.trim() || `Option ${index + 1}`}
                      </span>
                    </div>
                    <div className="flex items-center gap-2 shrink-0">
                      {opt.flag.trim() && (
                        <code className="text-xs font-mono text-text-muted bg-black/20 px-2 py-0.5 rounded max-w-[180px] truncate">
                          {opt.flag.trim()}
                        </code>
                      )}
                      <span className="text-[11px] uppercase tracking-wider font-medium text-text-muted">
                        {optionType}
                      </span>
                    </div>
                  </button>

                  {!isCollapsed && (
                    <div className="px-4 pb-4 pt-3 border-t border-white/10 space-y-4">
                      <Form.Field>
                        <Form.Label htmlFor={labelId}>Label</Form.Label>
                        <Form.Input
                          id={labelId}
                          placeholder="e.g. Fullscreen"
                          value={opt.label}
                          onChange={(e) =>
                            updateOption(index, { label: e.target.value })
                          }
                        />
                      </Form.Field>

                      <Form.Field>
                        <Form.Label>Type</Form.Label>
                        <Form.SelectionGroup className="grid-cols-3">
                          {(
                            [
                              { value: "flag", label: "Flag" },
                              { value: "value", label: "Value" },
                              { value: "select", label: "Select" },
                            ] as const
                          ).map((item) => (
                            <Form.Selection
                              key={item.value}
                              selected={optionType === item.value}
                              onClick={() =>
                                updateOption(index, { type: item.value })
                              }
                              className="justify-center"
                            >
                              {item.label}
                            </Form.Selection>
                          ))}
                        </Form.SelectionGroup>
                      </Form.Field>

                      <Form.Field>
                        <Form.Label htmlFor={flagId}>Flag / Argument</Form.Label>
                        <Form.Input
                          id={flagId}
                          placeholder="e.g. -fullscreen"
                          value={opt.flag}
                          onChange={(e) =>
                            updateOption(index, { flag: e.target.value })
                          }
                          className="font-mono text-xs"
                        />
                      </Form.Field>

                      {optionType === "select" && (
                        <Form.Field>
                          <Form.Label htmlFor={choicesId}>
                            Choices (comma-separated)
                          </Form.Label>
                          <Form.Input
                            id={choicesId}
                            placeholder="e.g. Vulkan, OGL, Software"
                            value={(opt.choices ?? []).join(", ")}
                            onChange={(e) =>
                              updateOption(index, {
                                choices: e.target.value
                                  .split(",")
                                  .map((c) => c.trim())
                                  .filter(Boolean),
                              })
                            }
                          />
                        </Form.Field>
                      )}

                      {optionType === "flag" && (
                        <SettingRow
                          label="Enabled by default"
                          description="Automatically enable this flag when adding the emulator to a game."
                          htmlFor={defaultId}
                          className="pt-1 pb-0"
                        >
                          <Switch
                            id={defaultId}
                            checked={Boolean(opt.default_enabled)}
                            onCheckedChange={(checked) =>
                              updateOption(index, { default_enabled: checked })
                            }
                            aria-label={`Default enabled for ${opt.label || "option"}`}
                          />
                        </SettingRow>
                      )}

                      <div className="flex justify-end pt-3 border-t border-white/10">
                        <Button
                          type="button"
                          variant="ghost"
                          size="sm"
                          onClick={() => removeOption(index)}
                          className="text-xs text-destructive hover:text-destructive"
                          aria-label="Remove option"
                        >
                          <Trash2 className="w-3.5 h-3.5 mr-1.5" />
                          Remove
                        </Button>
                      </div>
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </div>
      </form>

      <SheetFooter className="p-6 border-t border-white/10 bg-surface-raised/20 flex-row justify-end gap-2">
        <Button
          type="button"
          variant="ghost"
          onClick={onBack}
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
