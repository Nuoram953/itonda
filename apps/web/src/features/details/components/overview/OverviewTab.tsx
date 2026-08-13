import type { components } from "@/api/generated.d";
import type { TabId } from "../navigation/DetailsTabs";
import { WhatIsThisGameSection } from "./WhatIsThisGameSection";
import { HowDoesItPlaySection } from "./HowDoesItPlaySection";
import { SeeItInActionSection } from "./SeeItInActionSection";

type OverviewTabProps = {
  media: components["schemas"]["Media"];
  onNavigateTab: (tab: TabId) => void;
};

export function OverviewTab({ media, onNavigateTab }: OverviewTabProps) {
  return (
    <div className="space-y-6 sm:space-y-8 pb-16 animate-in fade-in duration-500">
      <WhatIsThisGameSection media={media} />
      <HowDoesItPlaySection media={media} />
      <SeeItInActionSection media={media} onNavigateTab={onNavigateTab} />
    </div>
  );
}
