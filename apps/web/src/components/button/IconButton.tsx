import type { IconType } from "react-icons";

type IconButtonProps = {
  icon: IconType;
} & HTMLButtonElement;

export const IconButton = (props: IconButtonProps) => {
  const Icon = props.icon;
  return (
    <button className="flex flex-col gap-2 p-1 rounded-md items-center text-text-muted hover:bg-surface-hover hover:text-primary-hover transition-colors [&.active]:text-primary-active">
      <Icon />
      {props.title}
    </button>
  );
};
