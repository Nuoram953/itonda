import { useState } from "react";
import {
  MoreVertical,
  RefreshCw,
  Copy,
  Check,
  LoaderCircle,
  ExternalLink,
} from "lucide-react";
import type { components } from "@/api/generated.d";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Switch } from "@/components/ui/switch";
import { useRefreshSingleMedia } from "../../api/post-media-refresh";
import { useNotification } from "@/hooks/use-notification";

type MoreOptionsProps = {
  media: components["schemas"]["Media"];
};

export const MoreOptions = ({ media }: MoreOptionsProps) => {
  const { notify } = useNotification();
  const [copied, setCopied] = useState(false);
  const [force, setForce] = useState(false);

  const refreshMutation = useRefreshSingleMedia({
    mutationConfig: {
      onError: (err) => {
        notify.error({
          title: "Refresh Failed",
          description:
            err instanceof Error
              ? err.message
              : "Failed to queue refresh for media",
        });
      },
    },
  });

  const handleRefresh = (forceRefresh: boolean) => {
    refreshMutation.mutate({
      mediaId: media.id,
      force: forceRefresh,
    });
  };

  const handleCopyId = async () => {
    try {
      await navigator.clipboard.writeText(media.id);
      setCopied(true);
      notify.info({
        title: "Copied to Clipboard",
        description: "Media ID copied to clipboard",
      });
      setTimeout(() => setCopied(false), 2000);
    } catch {
      notify.error({
        title: "Copy Failed",
        description: "Could not copy media ID to clipboard",
      });
    }
  };

  const steamStorefront = media.storefronts?.find(
    (s) => s.storefront_id === "steam" && s.external_id,
  );

  return (
    <DropdownMenu>
      <DropdownMenuTrigger
        render={
          <button
            type="button"
            title="More options"
            aria-label="More options"
            className="p-2.5 rounded-xl bg-surface/70 border border-white/10 text-white/80 hover:text-white hover:bg-white/10 transition-colors shadow-lg cursor-pointer disabled:opacity-50"
          >
            {refreshMutation.isPending ? (
              <LoaderCircle className="w-4 h-4 animate-spin text-accent-gold" />
            ) : (
              <MoreVertical className="w-4 h-4" />
            )}
          </button>
        }
      />
      <DropdownMenuContent
        align="end"
        sideOffset={8}
        className="bg-surface border border-white/10 text-white rounded-xl shadow-xl p-1.5 min-w-56"
      >
        <DropdownMenuGroup>
          <DropdownMenuLabel className="px-2 py-1 text-[11px] font-semibold text-text-muted uppercase tracking-wider">
            Sync & Refresh
          </DropdownMenuLabel>
          <div className="flex items-center justify-between gap-2 px-2.5 py-1.5 text-xs rounded-lg hover:bg-white/5 transition-colors">
            <button
              type="button"
              onClick={() => handleRefresh(force)}
              disabled={refreshMutation.isPending}
              className="flex items-center gap-2.5 text-left font-medium text-foreground hover:text-white flex-1 cursor-pointer disabled:opacity-50"
            >
              <RefreshCw
                className={`w-3.5 h-3.5 ${force ? "text-accent-gold" : "text-text-muted"} ${refreshMutation.isPending ? "animate-spin" : ""}`}
              />
              <span>{force ? "Force Refresh" : "Refresh"}</span>
            </button>
            <div
              className="flex items-center gap-1.5 pl-2 border-l border-white/10"
              onClick={(e) => e.stopPropagation()}
            >
              <label
                htmlFor="force-refresh-toggle"
                className="text-[11px] font-medium text-text-muted hover:text-foreground cursor-pointer select-none"
              >
                Force
              </label>
              <Switch
                id="force-refresh-toggle"
                size="sm"
                checked={force}
                onCheckedChange={setForce}
                disabled={refreshMutation.isPending}
                aria-label="Force refresh"
              />
            </div>
          </div>
        </DropdownMenuGroup>

        <DropdownMenuSeparator className="bg-white/10 my-1" />

        <DropdownMenuGroup>
          <DropdownMenuLabel className="px-2 py-1 text-[11px] font-semibold text-text-muted uppercase tracking-wider">
            Quick Actions
          </DropdownMenuLabel>

          {steamStorefront && (
            <DropdownMenuItem
              onClick={() => {
                window.open(
                  `https://store.steampowered.com/app/${steamStorefront.external_id}`,
                  "_blank",
                  "noopener,noreferrer",
                );
              }}
              className="flex items-center justify-between gap-2.5 px-2.5 py-2 text-xs rounded-lg hover:bg-white/10 cursor-pointer"
            >
              <div className="flex items-center gap-2.5">
                <ExternalLink className="w-3.5 h-3.5 text-text-muted" />
                <span className="font-medium text-foreground">
                  View on Steam
                </span>
              </div>
            </DropdownMenuItem>
          )}

          <DropdownMenuItem
            onClick={handleCopyId}
            className="flex items-center justify-between gap-2.5 px-2.5 py-2 text-xs rounded-lg hover:bg-white/10 cursor-pointer"
          >
            <div className="flex items-center gap-2.5">
              {copied ? (
                <Check className="w-3.5 h-3.5 text-emerald-400" />
              ) : (
                <Copy className="w-3.5 h-3.5 text-text-muted" />
              )}
              <span className="font-medium text-foreground">Copy Media ID</span>
            </div>
            <span className="text-[10px] text-text-muted font-mono truncate max-w-16">
              {media.id.slice(0, 8)}...
            </span>
          </DropdownMenuItem>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
};
