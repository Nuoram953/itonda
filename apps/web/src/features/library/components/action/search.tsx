import { Workspace } from "@/components/workspace/Workspace";
import { Search as SearchIcon } from "lucide-react";

export const Search = () => {
  return <Workspace.Action icon={SearchIcon}>Search</Workspace.Action>;
};
