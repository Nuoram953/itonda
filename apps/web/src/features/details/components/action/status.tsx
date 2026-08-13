import type { components } from "@/api/generated.d";
import { ChevronDown, Check } from "lucide-react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { usePatchMediaStatus } from "../../api/patch-media-status";

type MediaStatus = components["schemas"]["MediaStatus"];

type StatusProps = {
  mediaId: string;
  currentStatus: MediaStatus;
};

const STATUS_OPTIONS: MediaStatus[] = [
  "not_started",
  "in_progress",
  "completed",
  "abandoned",
  "paused",
];

const STATUS_LABELS: Record<MediaStatus, string> = {
  not_started: "Not Started",
  in_progress: "In Progress",
  completed: "Completed",
  abandoned: "Abandoned",
  paused: "Paused",
};

export const Status = ({ mediaId, currentStatus }: StatusProps) => {
  const patchStatusMutation = usePatchMediaStatus();

  const handleSelectStatus = (statusId: MediaStatus) => {
    if (statusId === currentStatus) return;
    patchStatusMutation.mutate({ mediaId, statusId });
  };

  const statusLabel = STATUS_LABELS[currentStatus] || currentStatus;

  return (
    <DropdownMenu>
      <DropdownMenuTrigger
        render={
          <button
            type="button"
            aria-label={currentStatus}
            className="inline-flex items-center gap-2 px-3.5 py-2 rounded-xl bg-surface-hover/80 hover:bg-surface-hover border border-white/15 text-xs font-semibold text-white shadow-md transition-all cursor-pointer"
          >
            <span className="text-text-muted">Status</span>
            <span className="text-white font-medium">{statusLabel}</span>
            <ChevronDown className="w-3.5 h-3.5 text-text-muted" />
          </button>
        }
      />
      <DropdownMenuContent className="bg-surface border border-white/10 text-white rounded-xl shadow-xl p-1 min-w-40">
        <DropdownMenuGroup>
          {STATUS_OPTIONS.map((status) => (
            <DropdownMenuItem
              key={status}
              onClick={() => handleSelectStatus(status)}
              className="flex items-center justify-between gap-2 px-3 py-2 text-xs rounded-lg hover:bg-white/10 cursor-pointer"
            >
              <span>{STATUS_LABELS[status] || status}</span>
              <span className="sr-only">{status}</span>
              {currentStatus === status && (
                <Check className="h-3.5 w-3.5 text-accent-gold" />
              )}
            </DropdownMenuItem>
          ))}
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
};
