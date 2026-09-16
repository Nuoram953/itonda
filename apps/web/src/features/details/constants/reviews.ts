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
    activeColor: "text-accent-gold",
    borderColor: "border-accent-gold/40",
    bgActive: "bg-surface-raised/90 ring-1 ring-accent-gold/40 border-accent-gold/40",
  },
  {
    id: "recommended",
    label: "Recommended",
    description: "Great game, thoroughly enjoyable and worth playing.",
    icon: ThumbsUp,
    activeColor: "text-emerald-300",
    borderColor: "border-emerald-500/40",
    bgActive: "bg-surface-raised/90 ring-1 ring-emerald-500/40 border-emerald-500/40",
  },
  {
    id: "neutral",
    label: "Neutral",
    description: "Decent or mixed feelings with noticeable caveats.",
    icon: Minus,
    activeColor: "text-amber-200/90",
    borderColor: "border-amber-400/30",
    bgActive: "bg-surface-raised/90 ring-1 ring-amber-400/30 border-amber-400/30",
  },
  {
    id: "do_not_recommend",
    label: "Do Not Recommend",
    description: "Disappointing, flawed, or frustrating.",
    icon: ThumbsDown,
    activeColor: "text-rose-300",
    borderColor: "border-rose-500/40",
    bgActive: "bg-surface-raised/90 ring-1 ring-rose-500/40 border-rose-500/40",
  },
];

export const THOUGHT_CATEGORIES: Array<{
  id: string;
  label: string;
  icon: typeof Music;
  color: string;
  dotColor: string;
  badgeStyle: string;
}> = [
  {
    id: "audio",
    label: "Music & Audio",
    icon: Music,
    color: "text-violet-400",
    dotColor: "bg-violet-400",
    badgeStyle: "bg-surface-raised/80 text-foreground/90 border-white/10",
  },
  {
    id: "gameplay",
    label: "Gameplay",
    icon: Gamepad2,
    color: "text-sky-400",
    dotColor: "bg-sky-400",
    badgeStyle: "bg-surface-raised/80 text-foreground/90 border-white/10",
  },
  {
    id: "story",
    label: "Story & Lore",
    icon: BookOpen,
    color: "text-amber-400",
    dotColor: "bg-amber-400",
    badgeStyle: "bg-surface-raised/80 text-foreground/90 border-white/10",
  },
  {
    id: "visuals",
    label: "Visuals & Art",
    icon: Eye,
    color: "text-emerald-400",
    dotColor: "bg-emerald-400",
    badgeStyle: "bg-surface-raised/80 text-foreground/90 border-white/10",
  },
  {
    id: "performance",
    label: "Performance",
    icon: Gauge,
    color: "text-cyan-400",
    dotColor: "bg-cyan-400",
    badgeStyle: "bg-surface-raised/80 text-foreground/90 border-white/10",
  },
  {
    id: "general",
    label: "General",
    icon: MessageSquare,
    color: "text-slate-400",
    dotColor: "bg-slate-400",
    badgeStyle: "bg-surface-raised/80 text-foreground/90 border-white/10",
  },
];
