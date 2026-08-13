import { useState } from "react";
import { Funnel, Search as SearchIcon, X } from "lucide-react";

import { Workspace } from "@/components/workspace/Workspace";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Sheet,
  SheetClose,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet";
import { useLibrary } from "../../hooks/useLibrary";

export const Filters = () => {
  const { search, setSearch } = useLibrary();
  const [value, setValue] = useState(search);

  const handleClear = () => setValue("");

  const handleReset = () => {
    setValue("");
    setSearch("");
  };

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setSearch(value);
  };

  return (
    <Sheet>
      <SheetTrigger
        render={<Workspace.Action icon={Funnel}>Filters</Workspace.Action>}
      />

      <SheetContent className="bg-surface/95 backdrop-blur-2xl border-l border-white/10 text-foreground p-6 shadow-2xl flex flex-col justify-between">
        <form
          onSubmit={handleSubmit}
          className="flex flex-col h-full justify-between gap-6"
        >
          <div className="space-y-6">
            <SheetHeader className="p-0 space-y-2">
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 rounded-2xl bg-linear-to-br from-primary via-primary-hover to-primary-active text-primary-foreground flex items-center justify-center shadow-lg shadow-primary/20 shrink-0">
                  <SearchIcon className="w-5 h-5" />
                </div>
                <div>
                  <SheetTitle className="text-base font-bold text-foreground">
                    Search library
                  </SheetTitle>
                  <SheetDescription className="text-xs text-text-muted">
                    Search your media collection.
                  </SheetDescription>
                </div>
              </div>
            </SheetHeader>

            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <Label
                  htmlFor="search"
                  className="text-xs font-semibold text-text-muted"
                >
                  Title
                </Label>
                {value && (
                  <button
                    type="button"
                    onClick={handleClear}
                    className="text-xs text-primary hover:underline cursor-pointer"
                  >
                    Clear Text
                  </button>
                )}
              </div>

              <div className="relative flex items-center">
                <SearchIcon className="absolute left-3.5 w-4 h-4 text-text-muted pointer-events-none" />
                <Input
                  id="search"
                  value={value}
                  onChange={(event) => setValue(event.target.value)}
                  placeholder="Search by title"
                  className="pl-10 pr-10 py-2.5 bg-surface-raised/80 border-white/10 text-foreground placeholder:text-text-muted/60 focus:border-primary/50 focus:ring-1 focus:ring-primary/40 rounded-xl text-xs"
                />
                {value && (
                  <button
                    type="button"
                    onClick={handleClear}
                    className="absolute right-3 p-1 rounded-md text-text-muted hover:text-foreground hover:bg-surface-hover transition-colors"
                  >
                    <X className="w-3.5 h-3.5" />
                  </button>
                )}
              </div>
            </div>
          </div>

          <SheetFooter className="p-0 gap-2 flex-col mt-auto">
            {search && (
              <button
                type="button"
                onClick={handleReset}
                className="w-full py-2.5 rounded-xl border border-white/10 bg-surface/60 text-text-muted text-xs font-medium hover:bg-surface-hover hover:text-foreground transition-colors cursor-pointer"
              >
                Reset Search Filter
              </button>
            )}

            <SheetClose
              type="submit"
              className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl bg-primary text-primary-foreground font-semibold text-xs hover:bg-primary-hover active:bg-primary-active transition-colors shadow-md cursor-pointer"
            >
              Search
            </SheetClose>
          </SheetFooter>
        </form>
      </SheetContent>
    </Sheet>
  );
};
