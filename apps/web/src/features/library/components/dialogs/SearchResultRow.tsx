import type { components } from "@/api/generated.d";
import { Search, Check } from "lucide-react";
import { cn } from "@/lib/utils";

type MediaSearchResult = components["schemas"]["MediaSearchResult"];

type SearchResultRowProps = {
  result: MediaSearchResult;
  isSelected: boolean;
  onSelect: () => void;
};

export function SearchResultRow({ result, isSelected, onSelect }: SearchResultRowProps) {
  return (
    <div
      onClick={onSelect}
      className={cn(
        "flex items-center gap-3 p-2.5 rounded-xl border text-left cursor-pointer transition-all duration-150",
        isSelected
          ? "border-primary bg-primary/10 ring-1 ring-primary/20 shadow-xs"
          : "border-white/5 bg-surface/40 hover:bg-surface-hover hover:border-white/10",
      )}
    >
      {result.cover_url ? (
        <img
          src={result.cover_url}
          alt={result.title}
          className="w-10 h-14 rounded-md object-cover shrink-0 bg-black/20 shadow-xs"
          loading="lazy"
        />
      ) : (
        <div className="w-10 h-14 rounded-md bg-surface-hover flex items-center justify-center shrink-0 text-text-muted">
          <Search className="w-4 h-4" />
        </div>
      )}

      <div className="flex-1 min-w-0 py-0.5">
        <div className="flex items-center gap-2">
          <span className="text-xs font-semibold text-foreground truncate">
            {result.title}
          </span>
          {result.year && (
            <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-white/10 text-text-muted shrink-0">
              {result.year}
            </span>
          )}
        </div>
        {result.summary && (
          <p className="text-[11px] text-text-muted line-clamp-2 mt-1 leading-relaxed">
            {result.summary}
          </p>
        )}
      </div>

      <div className="shrink-0 pl-1">
        {isSelected && (
          <div className="w-5 h-5 rounded-full bg-primary text-primary-foreground flex items-center justify-center shrink-0 shadow-xs">
            <Check className="w-3 h-3 stroke-3" />
          </div>
        )}
      </div>
    </div>
  );
}
