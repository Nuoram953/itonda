import type { components } from "@/api/generated.d";
import { Workspace } from "@/components/workspace/Workspace";
import { CircleDot, Check } from "lucide-react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { usePatchMediaStatus } from "../../api/patch-media-status";

export type MediaStatus = components["schemas"]["MediaStatus"];

type StatusProps = {
  mediaId?: string;
  currentStatus?: MediaStatus;
};

export const STATUS_OPTIONS: MediaStatus[] = [
  "not_started",
  "in_progress",
  "completed",
  "abandoned",
  "paused",
];

export const Status = ({ mediaId, currentStatus }: StatusProps) => {
  const patchStatusMutation = usePatchMediaStatus();

  const handleSelectStatus = (statusId: MediaStatus) => {
    if (!mediaId || statusId === currentStatus) return;
    patchStatusMutation.mutate({ mediaId, statusId });
  };

  return (
    <DropdownMenu>
      <DropdownMenuTrigger
        render={
          <Workspace.Action icon={CircleDot}>
            {currentStatus ?? "Status"}
          </Workspace.Action>
        }
      />
      <DropdownMenuContent>
        <DropdownMenuGroup>
          {STATUS_OPTIONS.map((status) => (
            <DropdownMenuItem
              key={status}
              onClick={() => handleSelectStatus(status)}
              className="flex items-center justify-between gap-2 cursor-pointer"
            >
              <span>{status}</span>
              {currentStatus === status && <Check className="h-4 w-4" />}
            </DropdownMenuItem>
          ))}
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
};

