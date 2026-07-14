export function RootErrorFallback({
  error,
  resetErrorBoundary,
}: {
  error: Error;
  resetErrorBoundary: () => void;
}) {
  return (
    <div className="">
      {error.message}
      <button onClick={resetErrorBoundary} className="">
        Reload Application
      </button>
    </div>
  );
}
