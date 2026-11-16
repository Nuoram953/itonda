import { useState } from "react";
import { Cpu } from "lucide-react";
import { useEmulators } from "../../api/emulators";
import { SettingsSection } from "./SettingsSection";
import { IntegrationRow } from "../rows/IntegrationRow";
import { EmulatorDrawer } from "../drawers/EmulatorDrawer";

export function EmulatorsSection() {
  const { data, isPending } = useEmulators();
  const [drawerOpen, setDrawerOpen] = useState(false);

  const count = data?.items?.length ?? 0;
  const metaText = isPending
    ? "Loading emulators..."
    : `${count} emulator${count === 1 ? "" : "s"} configured`;

  return (
    <>
      <SettingsSection
        title="Emulators"
        description="Configure emulators, default executables, and launch flags to attach to your games."
      >
        <IntegrationRow
          title="Emulators"
          mediaTypes={["game"]}
          description="Manage emulator executables, default arguments, and working directories."
          icon={<Cpu className="w-5 h-5" />}
          iconBgClass="bg-cyan-500/10 text-cyan-400 border-cyan-500/20"
          metaText={metaText}
          onOpenSheet={() => setDrawerOpen(true)}
        />
      </SettingsSection>

      <EmulatorDrawer open={drawerOpen} onOpenChange={setDrawerOpen} />
    </>
  );
}
