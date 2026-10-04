import { useState } from "react";
import { Workspace } from "@/components/workspace/Workspace";
import { Plus } from "lucide-react";
import type { ButtonHTMLAttributes } from "react";
import { AddMediaDialog } from "../dialogs/AddMediaDialog";

type AddMediaProps = ButtonHTMLAttributes<HTMLButtonElement>;

export const AddMedia = ({ onClick, ...props }: AddMediaProps) => {
  const [open, setOpen] = useState(false);

  const handleClick = (e: React.MouseEvent<HTMLButtonElement>) => {
    onClick?.(e);
    if (!e.defaultPrevented) {
      setOpen(true);
    }
  };

  return (
    <>
      <Workspace.Action icon={Plus} onClick={handleClick} {...props}>
        Add Media
      </Workspace.Action>
      <AddMediaDialog open={open} onOpenChange={setOpen} />
    </>
  );
};
