import { useState, useRef, useEffect } from "react";
import { Eye, EyeOff, Check, Copy, ExternalLink, Loader2 } from "lucide-react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";

type SecretInputProps = {
  id?: string;
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  portalUrl?: string;
  portalLabel?: string;
  onTest?: () => void;
  isTesting?: boolean;
  disabled?: boolean;
  className?: string;
};

export function SecretInput({
  id,
  value,
  onChange,
  placeholder = "Enter API key...",
  portalUrl,
  portalLabel = "Get API key",
  onTest,
  isTesting = false,
  disabled = false,
  className,
}: SecretInputProps) {
  const [visible, setVisible] = useState(false);
  const [copied, setCopied] = useState(false);
  const copyTimeoutRef = useRef<number | null>(null);

  const handleCopy = async () => {
    if (!value) return;
    try {
      await navigator.clipboard.writeText(value);
      setCopied(true);
      if (copyTimeoutRef.current) {
        clearTimeout(copyTimeoutRef.current);
      }
      copyTimeoutRef.current = setTimeout(() => setCopied(false), 2000);
    } catch {
      // Ignore clipboard error
    }
  };

  useEffect(() => {
    return () => {
      if (copyTimeoutRef.current) {
        clearTimeout(copyTimeoutRef.current);
      }
    };
  }, []);

  return (
    <div className={cn("space-y-1.5 w-full", className)}>
      <div className="relative flex items-center">
        <Input
          id={id}
          type={visible ? "text" : "password"}
          value={value}
          onChange={(e) => onChange(e.target.value)}
          placeholder={placeholder}
          disabled={disabled}
          autoComplete="off"
          spellCheck={false}
          className="pr-18 font-mono text-xs bg-surface/80 border-white/10 text-foreground placeholder:text-text-muted/50 focus-visible:border-primary/50"
        />
        <div className="absolute right-1 flex items-center gap-0.5">
          <Button
            type="button"
            variant="ghost"
            size="icon-xs"
            onClick={() => setVisible((prev) => !prev)}
            disabled={disabled}
            className="text-text-muted hover:text-foreground cursor-pointer"
            title={visible ? "Hide API key" : "Show API key"}
            aria-label={visible ? "Hide API key" : "Show API key"}
          >
            {visible ? (
              <EyeOff className="w-3.5 h-3.5" />
            ) : (
              <Eye className="w-3.5 h-3.5" />
            )}
          </Button>
          <Button
            type="button"
            variant="ghost"
            size="icon-xs"
            onClick={handleCopy}
            disabled={!value || disabled}
            className="text-text-muted hover:text-foreground cursor-pointer"
            title={copied ? "Copied!" : "Copy to clipboard"}
            aria-label={copied ? "Copied!" : "Copy to clipboard"}
          >
            {copied ? (
              <Check className="w-3.5 h-3.5 text-success" />
            ) : (
              <Copy className="w-3.5 h-3.5" />
            )}
          </Button>
        </div>
      </div>

      <div className="flex items-center justify-between gap-2 text-xs">
        {portalUrl ? (
          <a
            href={portalUrl}
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center gap-1 text-primary hover:text-primary-hover hover:underline transition-colors cursor-pointer"
          >
            <span>{portalLabel}</span>
            <ExternalLink className="w-3 h-3" />
          </a>
        ) : (
          <span />
        )}

        {onTest && (
          <Button
            type="button"
            variant="outline"
            size="xs"
            onClick={onTest}
            disabled={!value || isTesting || disabled}
            className="ml-auto text-xs border-white/10 hover:bg-surface-hover hover:text-foreground cursor-pointer"
          >
            {isTesting ? (
              <>
                <Loader2 className="w-3 h-3 animate-spin" />
                <span>Testing...</span>
              </>
            ) : (
              <span>Test Key</span>
            )}
          </Button>
        )}
      </div>
    </div>
  );
}
