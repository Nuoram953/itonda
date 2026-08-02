import { LibraryProvider } from "./store/LibraryContext";
import { MediaGrid } from "./components/Grid";

export const Libary = () => {
  return (
    <LibraryProvider>
      <MediaGrid />
    </LibraryProvider>
  );
};
