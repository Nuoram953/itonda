export type HowLongToBeatData = {
  main_story?: number | null;
  main_extra?: number | null;
  completionist?: number | null;
  all_styles?: number | null;
  main_story_hours?: number | null;
  main_extra_hours?: number | null;
  completionist_hours?: number | null;
  all_styles_hours?: number | null;
};

export type NormalizedHowLongToBeat = {
  mainStory: number | null;
  mainExtra: number | null;
  completionist: number | null;
  allStyles: number | null;
};
