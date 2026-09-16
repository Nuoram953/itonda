import { useState, useMemo } from "react";
import { Search, Plus, X, MessageSquareDashed } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { cn } from "@/lib/utils";
import type { GameThought } from "../../types/reviews";
import { ThoughtCard } from "./ThoughtCard";
import { ThoughtDialog } from "./ThoughtDialog";
import { THOUGHT_CATEGORIES } from "../../constants/reviews";

type SessionThoughtsSectionProps = {
  thoughts: GameThought[];
  defaultPlaytimeMinutes?: number;
  onCreateThought: (data: {
    title: string;
    content: string;
    category: string;
    playtime_minutes?: number;
  }) => Promise<void>;
  onUpdateThought: (
    thoughtId: string,
    data: {
      title: string;
      content: string;
      category: string;
    },
  ) => Promise<void>;
  onDeleteThought: (thoughtId: string) => Promise<void>;
};

export function SessionThoughtsSection({
  thoughts,
  defaultPlaytimeMinutes,
  onCreateThought,
  onUpdateThought,
  onDeleteThought,
}: SessionThoughtsSectionProps) {
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedCategory, setSelectedCategory] = useState<string>("all");
  const [dialogOpen, setDialogOpen] = useState(false);
  const [editingThought, setEditingThought] = useState<GameThought | null>(null);

  const filteredThoughts = useMemo(() => {
    return thoughts.filter((t) => {
      const matchesCategory =
        selectedCategory === "all" || t.category === selectedCategory;

      const q = searchQuery.toLowerCase().trim();
      const matchesSearch =
        !q ||
        t.title.toLowerCase().includes(q) ||
        t.content.toLowerCase().includes(q) ||
        t.category.toLowerCase().includes(q);

      return matchesCategory && matchesSearch;
    });
  }, [thoughts, selectedCategory, searchQuery]);

  const categoryCounts = useMemo(() => {
    const counts: Record<string, number> = { all: thoughts.length };
    for (const t of thoughts) {
      counts[t.category] = (counts[t.category] || 0) + 1;
    }
    return counts;
  }, [thoughts]);

  const handleOpenCreate = () => {
    setEditingThought(null);
    setDialogOpen(true);
  };

  const handleOpenEdit = (thought: GameThought) => {
    setEditingThought(thought);
    setDialogOpen(true);
  };

  const handleSaveThought = async (data: {
    title: string;
    content: string;
    category: string;
    playtime_minutes?: number;
  }) => {
    if (editingThought) {
      await onUpdateThought(editingThought.id, {
        title: data.title,
        content: data.content,
        category: data.category,
      });
    } else {
      await onCreateThought(data);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-white/10 pb-4">
        <div>
          <h3 className="text-xl font-bold text-foreground tracking-tight flex items-center gap-2.5">
            <span>Session Thoughts & Notes</span>
            <span className="text-xs font-semibold px-2.5 py-0.5 rounded-full bg-surface/80 border border-white/10 text-text-muted">
              {thoughts.length}
            </span>
          </h3>
          <p className="text-xs sm:text-sm text-text-muted mt-1">
            Jot down thoughts and highlights after sessions so you can easily find them later.
          </p>
        </div>

        <Button
          onClick={handleOpenCreate}
          className="bg-accent-gold text-background hover:bg-accent-gold/90 font-semibold gap-1.5 shrink-0 cursor-pointer shadow-md self-start sm:self-auto"
        >
          <Plus className="w-4 h-4" />
          <span>Add Thought</span>
        </Button>
      </div>

      {thoughts.length > 0 && (
        <div className="flex flex-col md:flex-row items-stretch md:items-center justify-between gap-3">
          <div className="relative flex-1 max-w-md">
            <Search className="absolute left-3.5 top-1/2 -translate-y-1/2 w-4 h-4 text-text-muted pointer-events-none" />
            <Input
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search thoughts (e.g. music, combat, boss)..."
              className="pl-9 pr-8 bg-surface/60 border-white/10 text-xs sm:text-sm rounded-xl focus:ring-accent-gold/50"
            />
            {searchQuery && (
              <button
                type="button"
                onClick={() => setSearchQuery("")}
                className="absolute right-2.5 top-1/2 -translate-y-1/2 text-text-muted hover:text-white p-0.5 cursor-pointer"
              >
                <X className="w-3.5 h-3.5" />
              </button>
            )}
          </div>

          <div className="flex items-center gap-1.5 overflow-x-auto no-scrollbar py-1">
            <button
              type="button"
              onClick={() => setSelectedCategory("all")}
              className={cn(
                "inline-flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-semibold transition-colors cursor-pointer shrink-0",
                selectedCategory === "all"
                  ? "bg-accent-gold text-background"
                  : "bg-surface/50 border border-white/10 text-text-muted hover:text-white hover:bg-white/5",
              )}
            >
              <span>All</span>
              <span
                className={cn(
                  "text-[10px] px-1.5 py-0.2 rounded-full",
                  selectedCategory === "all"
                    ? "bg-background/20 text-background font-bold"
                    : "bg-white/10 text-text-muted",
                )}
              >
                {categoryCounts.all || 0}
              </span>
            </button>

            {THOUGHT_CATEGORIES.map((cat) => {
              const count = categoryCounts[cat.id] || 0;
              const isSelected = selectedCategory === cat.id;
              const Icon = cat.icon;

              return (
                <button
                  key={cat.id}
                  type="button"
                  onClick={() => setSelectedCategory(cat.id)}
                  className={cn(
                    "inline-flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-semibold border transition-colors cursor-pointer shrink-0",
                    isSelected
                      ? cn(cat.badgeStyle, "ring-1 ring-white/20")
                      : "border-white/10 bg-surface/40 text-text-muted hover:text-white hover:bg-white/5",
                  )}
                >
                  <Icon className={cn("w-3 h-3", isSelected ? cat.color : "")} />
                  <span>{cat.label}</span>
                  {count > 0 && (
                    <span
                      className={cn(
                        "text-[10px] px-1.5 py-0.2 rounded-full font-bold",
                        isSelected ? "bg-white/10" : "bg-white/5 text-text-muted",
                      )}
                    >
                      {count}
                    </span>
                  )}
                </button>
              );
            })}
          </div>
        </div>
      )}

      {thoughts.length === 0 ? (
        <div className="rounded-2xl border border-dashed border-white/15 bg-surface/30 p-10 text-center flex flex-col items-center justify-center space-y-4">
          <div className="p-4 rounded-2xl bg-accent-gold/10 border border-accent-gold/20 text-accent-gold">
            <MessageSquareDashed className="w-8 h-8" />
          </div>
          <div className="space-y-1 max-w-sm">
            <h4 className="text-base font-bold text-foreground">
              No session thoughts yet
            </h4>
            <p className="text-xs sm:text-sm text-text-muted">
              Did something catch your ear or challenge your playstyle? Record quick impressions and find them whenever you want.
            </p>
          </div>
          <Button
            onClick={handleOpenCreate}
            className="bg-accent-gold text-background hover:bg-accent-gold/90 font-semibold gap-1.5 cursor-pointer shadow-md"
          >
            <Plus className="w-4 h-4" />
            <span>Add Your First Thought</span>
          </Button>
        </div>
      ) : filteredThoughts.length === 0 ? (
        <div className="rounded-2xl border border-white/10 bg-surface/30 p-8 text-center space-y-3">
          <p className="text-sm text-text-muted">
            No thoughts match your current search or category filter.
          </p>
          <Button
            variant="outline"
            size="sm"
            onClick={() => {
              setSearchQuery("");
              setSelectedCategory("all");
            }}
            className="border-white/10 text-xs"
          >
            Clear Filters
          </Button>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {filteredThoughts.map((thought) => (
            <ThoughtCard
              key={thought.id}
              thought={thought}
              onEdit={handleOpenEdit}
              onDelete={onDeleteThought}
            />
          ))}
        </div>
      )}

      <ThoughtDialog
        open={dialogOpen}
        onOpenChange={setDialogOpen}
        initialThought={editingThought}
        defaultPlaytimeMinutes={defaultPlaytimeMinutes}
        onSave={handleSaveThought}
      />
    </div>
  );
}
