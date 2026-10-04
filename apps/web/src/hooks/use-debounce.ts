import { useState, useEffect } from "react";

/**
 * Custom hook to debounce a rapidly changing value (e.g. search input).
 *
 * @param value The value to debounce
 * @param delayMs Delay in milliseconds before updating debounced value (default: 400ms)
 * @returns The debounced value
 */
export function useDebounce<T>(value: T, delayMs: number = 400): T {
  const [debouncedValue, setDebouncedValue] = useState<T>(value);

  useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedValue(value);
    }, delayMs);

    return () => {
      clearTimeout(handler);
    };
  }, [value, delayMs]);

  return debouncedValue;
}
