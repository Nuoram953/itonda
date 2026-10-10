import type { components } from "@/api/generated.d";
import type { TabId } from "../navigation/DetailsTabs";
import { FeaturedMedia } from "./FeaturedMedia";
import { ScreenshotCarousel } from "./ScreenshotCarousel";
import { AboutSection } from "./AboutSection";

type OverviewTabProps = {
  media: components["schemas"]["Media"];
  onNavigateTab: (tab: TabId) => void;
};

export function OverviewTab({ media, onNavigateTab }: OverviewTabProps) {
  return (
    <div className="space-y-8 animate-in fade-in duration-300 pb-16">
      <FeaturedMedia media={media} />

      <ScreenshotCarousel
        media={media}
        onViewGallery={() => onNavigateTab("gallery")}
      />

      <AboutSection media={media} />
    </div>
  );
}
