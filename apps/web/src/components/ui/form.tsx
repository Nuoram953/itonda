import * as React from "react";
import { cva, type VariantProps } from "class-variance-authority";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { cn } from "@/lib/utils";

const formItemVariants = cva("group/form-item flex w-full", {
  variants: {
    orientation: {
      vertical: "flex-col gap-2.5",
      horizontal: "flex-row items-center gap-3",
    },
  },
  defaultVariants: {
    orientation: "vertical",
  },
});

interface FormItemProps
  extends React.ComponentProps<"div">,
    VariantProps<typeof formItemVariants> {}

const FormItem = React.forwardRef<HTMLDivElement, FormItemProps>(
  ({ className, orientation, ...props }, ref) => {
    return (
      <div
        ref={ref}
        data-slot="form-item"
        className={cn(formItemVariants({ orientation }), className)}
        {...props}
      />
    );
  }
);
FormItem.displayName = "FormItem";

const formLabelVariants = cva("flex items-center gap-2 select-none", {
  variants: {
    variant: {
      default:
        "text-xs font-semibold uppercase tracking-wider text-text-muted leading-none",
      standard: "text-sm font-medium text-foreground leading-none",
    },
  },
  defaultVariants: {
    variant: "default",
  },
});

interface FormLabelProps
  extends React.ComponentProps<typeof Label>,
    VariantProps<typeof formLabelVariants> {}

const FormLabel = React.forwardRef<
  React.ComponentRef<typeof Label>,
  FormLabelProps
>(({ className, variant, ...props }, ref) => {
  return (
    <Label
      ref={ref}
      data-slot="form-label"
      className={cn(formLabelVariants({ variant }), className)}
      {...props}
    />
  );
});
FormLabel.displayName = "FormLabel";

const FormControl = React.forwardRef<
  HTMLDivElement,
  React.ComponentProps<"div">
>(({ className, ...props }, ref) => {
  return (
    <div
      ref={ref}
      data-slot="form-control"
      className={cn("relative w-full", className)}
      {...props}
    />
  );
});
FormControl.displayName = "FormControl";

const FormDescription = React.forwardRef<
  HTMLParagraphElement,
  React.ComponentProps<"p">
>(({ className, ...props }, ref) => {
  return (
    <p
      ref={ref}
      data-slot="form-description"
      className={cn("text-xs text-text-muted", className)}
      {...props}
    />
  );
});
FormDescription.displayName = "FormDescription";

const FormMessage = React.forwardRef<
  HTMLParagraphElement,
  React.ComponentProps<"p">
>(({ className, ...props }, ref) => {
  return (
    <p
      ref={ref}
      data-slot="form-message"
      role="alert"
      className={cn("text-xs font-medium text-destructive", className)}
      {...props}
    />
  );
});
FormMessage.displayName = "FormMessage";

const FormSelectionGroup = React.forwardRef<
  HTMLDivElement,
  React.ComponentProps<"div">
>(({ className, ...props }, ref) => {
  return (
    <div
      ref={ref}
      data-slot="form-selection-group"
      className={cn("grid w-full gap-2.5 sm:gap-3", className)}
      {...props}
    />
  );
});
FormSelectionGroup.displayName = "FormSelectionGroup";

const formSelectionVariants = cva(
  "relative flex items-center rounded-xl border transition-all duration-200 cursor-pointer text-xs font-medium focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/50 disabled:pointer-events-none disabled:opacity-50 select-none",
  {
    variants: {
      selected: {
        true: "border-primary bg-primary/10 text-primary shadow-xs ring-1 ring-primary/30",
        false:
          "border-white/10 bg-surface/50 text-foreground/70 hover:bg-surface-hover hover:border-white/20 hover:text-foreground",
      },
      size: {
        default: "p-3",
        sm: "p-2 text-xs",
        lg: "p-4 text-sm",
      },
    },
    defaultVariants: {
      selected: false,
      size: "default",
    },
  }
);

interface FormSelectionProps
  extends React.ComponentProps<"button">,
    VariantProps<typeof formSelectionVariants> {
  selected?: boolean;
}

const FormSelection = React.forwardRef<HTMLButtonElement, FormSelectionProps>(
  ({ className, selected = false, size, type = "button", ...props }, ref) => {
    return (
      <button
        ref={ref}
        type={type}
        aria-pressed={selected}
        data-slot="form-selection"
        data-selected={selected}
        className={cn(formSelectionVariants({ selected, size }), className)}
        {...props}
      />
    );
  }
);
FormSelection.displayName = "FormSelection";

const formInputVariants = cva(
  "h-10 rounded-xl bg-surface/50 text-sm focus-visible:border-primary/50 focus-visible:ring-1 focus-visible:ring-primary/40",
  {
    variants: {
      variant: {
        default: "bg-surface/50",
        raised: "bg-surface-raised/80",
      },
      inputSize: {
        default: "h-10 text-sm",
        sm: "h-9 text-xs",
        lg: "h-11 text-base",
      },
    },
    defaultVariants: {
      variant: "default",
      inputSize: "default",
    },
  }
);

interface FormInputProps
  extends React.ComponentProps<typeof Input>,
    VariantProps<typeof formInputVariants> {}

const FormInput = React.forwardRef<
  React.ComponentRef<typeof Input>,
  FormInputProps
>(({ className, variant, inputSize, ...props }, ref) => {
  return (
    <Input
      ref={ref}
      data-slot="form-input"
      className={cn(formInputVariants({ variant, inputSize }), className)}
      {...props}
    />
  );
});
FormInput.displayName = "FormInput";

const formTextareaVariants = cva(
  "w-full rounded-xl border border-input bg-surface/50 px-3.5 py-2.5 text-sm text-foreground placeholder:text-muted-foreground focus-visible:border-primary/50 focus-visible:ring-1 focus-visible:ring-primary/40 focus:outline-none transition-all disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 resize-none",
  {
    variants: {
      variant: {
        default: "bg-surface/50",
        raised: "bg-surface-raised/80",
      },
    },
    defaultVariants: {
      variant: "default",
    },
  }
);

interface FormTextareaProps
  extends React.ComponentProps<"textarea">,
    VariantProps<typeof formTextareaVariants> {}

const FormTextarea = React.forwardRef<
  HTMLTextAreaElement,
  FormTextareaProps
>(({ className, variant, ...props }, ref) => {
  return (
    <textarea
      ref={ref}
      data-slot="form-textarea"
      className={cn(formTextareaVariants({ variant }), className)}
      {...props}
    />
  );
});
FormTextarea.displayName = "FormTextarea";

const FormRoot = React.forwardRef<
  HTMLFormElement,
  React.ComponentProps<"form">
>(({ className, ...props }, ref) => {
  return (
    <form
      ref={ref}
      data-slot="form"
      className={cn("space-y-4", className)}
      {...props}
    />
  );
});
FormRoot.displayName = "Form";

const Form = Object.assign(FormRoot, {
  Field: FormItem,
  Item: FormItem,
  Label: FormLabel,
  Control: FormControl,
  Input: FormInput,
  Textarea: FormTextarea,
  Description: FormDescription,
  Message: FormMessage,
  Selection: FormSelection,
  SelectionGroup: FormSelectionGroup,
});

export {
  Form,
  FormItem,
  FormItem as FormField,
  FormLabel,
  FormControl,
  FormInput,
  FormTextarea,
  FormDescription,
  FormMessage,
  FormSelection,
  FormSelectionGroup,
};
