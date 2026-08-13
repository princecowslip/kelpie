import { useParams } from "react-router-dom";
import { PlaceholderPage } from "../components/PlaceholderPage";

// /item/:uid (kelpie.md §71 Route Model, §16 Unified MediaItem). The
// kind-specific viewers (§76-87) are later phases.
export function ItemPage() {
  const { uid } = useParams<{ uid: string }>();
  return (
    <PlaceholderPage
      title={`Item ${uid ?? ""}`}
      tag="item"
      description="The specialized viewers (video, GIF, image, manga/comic, literature, audio — kelpie.md §76-87) land in later phases."
    />
  );
}
