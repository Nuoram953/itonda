import { useState, useEffect } from "react";

function getIsAppFocused(): boolean {
  if (typeof document === "undefined") return true;
  if (document.visibilityState === "hidden") return false;
  if (typeof document.hasFocus === "function") {
    return document.hasFocus();
  }
  return true;
}

export function useAppFocus(): boolean {
  const [isFocused, setIsFocused] = useState<boolean>(getIsAppFocused);

  useEffect(() => {
    if (typeof window === "undefined" || typeof document === "undefined") {
      return;
    }

    const handleFocus = () => setIsFocused(true);
    const handleBlur = () => setIsFocused(false);
    const handleVisibilityChange = () => {
      setIsFocused(getIsAppFocused());
    };

    window.addEventListener("focus", handleFocus);
    window.addEventListener("blur", handleBlur);
    document.addEventListener("visibilitychange", handleVisibilityChange);

    // Initial sync
    setIsFocused(getIsAppFocused());

    return () => {
      window.removeEventListener("focus", handleFocus);
      window.removeEventListener("blur", handleBlur);
      document.removeEventListener("visibilitychange", handleVisibilityChange);
    };
  }, []);

  return isFocused;
}
