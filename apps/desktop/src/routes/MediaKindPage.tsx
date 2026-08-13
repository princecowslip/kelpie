import { useParams } from "react-router-dom";
import { PlaceholderPage } from "../components/PlaceholderPage";

// /media/:kind — supports the MEDIA nav group (kelpie.md §70). Not literally
// in §71's Route Model (which only defines detail routes like /item/:uid,
// not per-kind listing routes), added so each MEDIA nav item has a real
// destination. Filtered feed-by-kind views land alongside the feed/search
// engine (Phase 4).
export function MediaKindPage() {
  const { kind } = useParams<{ kind: string }>();
  return (
    <PlaceholderPage
      title={kind ? kind[0].toUpperCase() + kind.slice(1) : "Media"}
      tag="media kind"
      description={`A feed filtered to "${kind}" (kelpie.md §61 Feed Filtering) will render here once feed filtering is implemented in Phase 4.`}
    />
  );
}
