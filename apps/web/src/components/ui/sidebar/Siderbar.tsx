import { Link } from "@tanstack/react-router";
import { ChevronDown, Download, Settings } from "lucide-react";
import { useState } from "react";

export const Sidebar = () => {
  return (
    <aside className="w-64 shrink-0 flex flex-col h-full border-r border-border-strong bg-surface">
      <div className="flex-1 overflow-y-auto py-4 px-2">
        <nav className="flex flex-col gap-2">
          <LinkItem label="Home" path={"/"} />
          <LinkItem label="Rankings" path={"/rankings"} />
          <LinkItem
            label="Media"
            path={"/media"}
            children={[
              { label: "Games", path: "/games" },
              { label: "Movies", path: "/movies" },
              { label: "TV Series", path: "/series" },
              { label: "Books", path: "/books" },
              { label: "Music", path: "/music" },
            ]}
          />
        </nav>
      </div>

      <div className="flex items-center justify-around py-4 border-t border-border-strong">
        <Link
          to="/downloads"
          className="flex text-2xl w-full items-center mx-auto justify-center content-center text-text-muted hover:text-primary-hover transition-colors [&.active]:text-primary-active"
          title="Downloads"
        >
          <Download />
        </Link>

        <div className="h-6 w-px bg-border-strong" />

        <Link
          to="/settings"
          className="flex text-2xl w-full items-center mx-auto justify-center content-center text-text-muted hover:text-primary-hover transition-colors [&.active]:text-primary-active"
          title="Settings"
        >
          <Settings />
        </Link>
      </div>
    </aside>
  );
};

type LinkItemProp = {
  label: string;
  path: string;
  children?: LinkItemProp[];
};

const LinkItem = ({ label, path, children }: LinkItemProp) => {
  const [open, setOpen] = useState(true);

  const toggleOpen = () => {
    setOpen((prev) => !prev);
  };

  return (
    <div className="flex flex-col gap-1">
      <Link
        to={path}
        className="[&.active]:font-bold text-foreground [&.active]:text-primary-active w-full"
      >
        <div className="flex p-2 items-center justify-between hover:text-primary-hover hover:bg-surface-hover rounded-md transition-colors cursor-pointer">
          <div className="flex gap-3 items-center">
            <span>{label}</span>
          </div>
          {children && (
            <ChevronDown onClick={toggleOpen} aria-label={`toggle`} />
          )}
        </div>
      </Link>

      {children && open && (
        <div className="pl-4 border-l border-border-strong ml-4 mt-1">
          {children.map((subItem) => (
            <LinkItem key={subItem.path} {...subItem} />
          ))}
        </div>
      )}
    </div>
  );
};
