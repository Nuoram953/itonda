import { Workspace } from "@/components/workspace/Workspace";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Search as SearchIcon } from "lucide-react";

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
import { useState } from "react";

export const Search = () => {
  const { search, setSearch } = useLibrary();
  const [value, setValue] = useState(search);

  return (
    <Sheet>
      <SheetTrigger
        render={<Workspace.Action icon={SearchIcon}>Search</Workspace.Action>}
      />

      <SheetContent>
        <form
          onSubmit={(event) => {
            event.preventDefault();
            setSearch(value);
          }}
        >
          <SheetHeader>
            <SheetTitle>Search library</SheetTitle>
            <SheetDescription>Search your media collection.</SheetDescription>
          </SheetHeader>

          <div className="grid gap-3 px-4">
            <Label htmlFor="search">Title</Label>

            <Input
              id="search"
              value={value}
              onChange={(event) => setValue(event.target.value)}
              placeholder="Search by title"
            />
          </div>

          <SheetFooter>
            <SheetClose type="submit" className={"w-full"}>
              Search
            </SheetClose>
          </SheetFooter>
        </form>
      </SheetContent>
    </Sheet>
  );
};
