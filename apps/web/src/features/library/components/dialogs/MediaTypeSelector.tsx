import type { components } from "@/api/generated.d";
import { Gamepad2, Film, Tv } from "lucide-react";
import { cn } from "@/lib/utils";
import { Form } from "@/components/ui/form";

type MediaType = components["schemas"]["MediaType"];

export const MEDIA_TYPES: { type: MediaType; label: string; icon: typeof Gamepad2 }[] = [
  { type: "game", label: "Game", icon: Gamepad2 },
  { type: "movie", label: "Movie", icon: Film },
  { type: "tv_show", label: "TV Show", icon: Tv },
];

type MediaTypeSelectorProps = {
  selected: MediaType | null;
  onSelect: (type: MediaType) => void;
  label?: React.ReactNode;
  className?: string;
};

export function MediaTypeSelector({
  selected,
  onSelect,
  label = "1. Select Media Type",
  className,
}: MediaTypeSelectorProps) {
  return (
    <Form.Field className={className}>
      {label && <Form.Label>{label}</Form.Label>}
      <Form.SelectionGroup className="grid grid-cols-3 gap-2.5 sm:gap-3">
        {MEDIA_TYPES.map(({ type, label: itemLabel, icon: Icon }) => {
          const isSelected = selected === type;
          return (
            <Form.Selection
              key={type}
              selected={isSelected}
              onClick={() => onSelect(type)}
              className="flex flex-col items-center justify-center gap-1.5 p-3 min-h-[68px]"
            >
              <Icon className={cn("w-5 h-5", isSelected ? "text-primary" : "text-text-muted")} />
              <span>{itemLabel}</span>
            </Form.Selection>
          );
        })}
      </Form.SelectionGroup>
    </Form.Field>
  );
}

