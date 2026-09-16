import {
  Music,
  Gamepad2,
  BookOpen,
  Eye,
  Gauge,
  MessageSquare,
  Sparkles,
  ThumbsUp,
  Minus,
  ThumbsDown,
} from "lucide-react";
import type { ReviewVerdict } from "../types/reviews";

export const VERDICT_OPTIONS: Array<{
  id: ReviewVerdict;
  label: string;
  description: string;
  icon: typeof Sparkles;
  activeColor: string;
  borderColor: string;
  bgActive: string;
}> = [
  {
    id: "masterpiece",
    label: "Masterpiece",
    description: "An exceptional, benchmark gaming experience.",
    icon: Sparkles,
    activeColor: "text-amber-300",
    borderColor: "border-amber-500/50",
    bgActive: "bg-amber-500/15 ring-1 ring-amber-400/30",
  },
  {
    id: "recommended",
    label: "Recommended",
    description: "Great game, thoroughly enjoyable and worth playing.",
    icon: ThumbsUp,
    activeColor: "text-emerald-300",
    borderColor: "border-emerald-500/50",
    bgActive: "bg-emerald-500/15 ring-1 ring-emerald-400/30",
  },
  {
    id: "neutral",
    label: "Neutral",
    description: "Decent or mixed feelings with noticeable caveats.",
    icon: Minus,
    activeColor: "text-amber-200",
    borderColor: "border-amber-400/40",
    bgActive: "bg-amber-400/15 ring-1 ring-amber-300/30",
  },
  {
    id: "do_not_recommend",
    label: "Do Not Recommend",
    description: "Disappointing, flawed, or frustrating.",
    icon: ThumbsDown,
    activeColor: "text-rose-300",
    borderColor: "border-rose-500/50",
    bgActive: "bg-rose-500/15 ring-1 ring-rose-400/30",
  },
];

export const THOUGHT_CATEGORIES: Array<{
  id: string;
  label: string;
  icon: typeof Music;
  color: string;
  badgeStyle: string;
}> = [
  {
    id: "audio",
    label: "Music & Audio",
    icon: Music,
    color: "text-purple-400",
    badgeStyle: "bg-purple-500/15 text-purple-300 border-purple-500/30",
  },
  {
    id: "gameplay",
    label: "Gameplay",
    icon: Gamepad2,
    color: "text-sky-400",
    badgeStyle: "bg-sky-500/15 text-sky-300 border-sky-500/30",
  },
  {
    id: "story",
    label: "Story & Lore",
    icon: BookOpen,
    color: "text-amber-400",
    badgeStyle: "bg-amber-500/15 text-amber-300 border-amber-500/30",
  },
  {
    id: "visuals",
    label: "Visuals & Art",
    icon: Eye,
    color: "text-emerald-400",
    badgeStyle: "bg-emerald-500/15 text-emerald-300 border-emerald-500/30",
  },
  {
    id: "performance",
    label: "Performance",
    icon: Gauge,
    color: "text-cyan-400",
    badgeStyle: "bg-cyan-500/15 text-cyan-300 border-cyan-500/30",
  },
  {
    id: "general",
    label: "General",
    icon: MessageSquare,
    color: "text-slate-300",
    badgeStyle: "bg-slate-500/15 text-slate-200 border-slate-500/30",
  },
];
