import { Workspace } from "@/components/workspace/Workspace";
import { Plus } from "lucide-react";
import type { ButtonHTMLAttributes } from "react";

type AddMediaProps = ButtonHTMLAttributes<HTMLButtonElement>;

export const AddMedia = (props: AddMediaProps) => {
  return (
    <Workspace.Action icon={Plus} {...props}>
      Add Media
    </Workspace.Action>
  );
};
