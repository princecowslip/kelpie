import { useParams } from "react-router-dom";
import { PlaceholderPage } from "../components/PlaceholderPage";

// /chapter/:uid (kelpie.md §71 Route Model, §81-83 Manga/Comic Reader).
export function ChapterPage() {
  const { uid } = useParams<{ uid: string }>();
  return (
    <PlaceholderPage
      title={`Chapter ${uid ?? ""}`}
      tag="chapter"
      description="The manga/comic reader (kelpie.md §81-83) lands in Phase 6."
    />
  );
}
