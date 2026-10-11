import { Bell } from "lucide-react";
import { GlobalSearch } from "./GlobalSearch";

export const Header = () => {
  return (
    <header className="sticky top-0 z-40 flex h-16 shrink-0 items-center justify-between border-b border-white/10 bg-surface/80 px-6 backdrop-blur-xl shadow-sm text-foreground">
      <div className="hidden md:flex items-center">
        <GlobalSearch />
      </div>

      <div className="flex items-center gap-3">

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
