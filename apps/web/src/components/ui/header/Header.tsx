import { Bell, Plus, Search } from "lucide-react";

export const Header = () => {
  return (
    <header className="sticky top-0 z-40 flex h-16 shrink-0 items-center justify-between border-b border-white/10 bg-surface/80 px-6 backdrop-blur-xl shadow-sm text-foreground">
      <div className="hidden md:flex items-center gap-2.5 bg-surface-raised/50 border border-white/10 focus-within:border-primary/50 focus-within:ring-1 focus-within:ring-primary/40 rounded-xl px-3.5 py-1.5 w-72 lg:w-96 text-sm text-text-muted transition-all duration-200">
        <Search className="w-4 h-4 text-text-muted shrink-0" />
        <input
          type="text"
          placeholder="Search media, games, movies..."
          className="w-full bg-transparent border-none outline-none text-foreground placeholder:text-text-muted/60 text-xs"
        />
        <kbd className="hidden lg:inline-flex items-center gap-0.5 rounded bg-surface border border-white/10 px-1.5 text-xs text-text-muted font-mono">
          ⌘K
        </kbd>
      </div>

      <div className="flex items-center gap-3">
        <button
          type="button"
          className="hidden sm:flex items-center gap-1.5 px-3 py-1.5 rounded-xl bg-primary text-primary-foreground text-xs font-medium hover:bg-primary-hover active:bg-primary-active transition-colors shadow-sm cursor-pointer"
        >
          <Plus className="w-3.5 h-3.5" />
          <span>Add Media</span>
        </button>

        <button
          type="button"
          className="relative p-2 rounded-xl bg-surface/70 border border-white/10 text-text-muted hover:text-foreground hover:bg-surface-hover transition-colors cursor-pointer"
          title="Notifications"
          aria-label="Notifications"
        >
          <Bell className="w-4 h-4" />
          <span className="absolute top-1.5 right-1.5 w-2 h-2 rounded-full bg-primary" />
        </button>
      </div>
    </header>
  );
};
