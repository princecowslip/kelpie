import { useParams } from "react-router-dom";
import { PlaceholderPage } from "../components/PlaceholderPage";
import { EmptyState } from "../components/EmptyState";

// /library/saved, /library/series, /library/collections,
// /library/collection/:id, /library/following, /library/history,
// /library/downloads (kelpie.md §71 Route Model, §94-98 Library / Collections
// / Smart Collections / Following / History). Real persistence needs
// crates/database (out of scope for this unit), so each renders an empty
// state rather than a "not built" placeholder — this is genuinely what an
// empty library looks like.
export function LibraryPage({ title, description }: { title: string; description: string }) {
  return (
    <div>
      <div className="page-header">
        <div>
          <h1>{title}</h1>
          <p>{description}</p>
        </div>
      </div>
      <EmptyState
        title="Nothing here yet"
        message="This list is backed by the local database (kelpie.md §104), which lands in Phase 2. Nothing has been saved yet."
      />
    </div>
  );
}

export function LibraryCollectionPage() {
  const { id } = useParams<{ id: string }>();
  return (
    <PlaceholderPage
      title={`Collection ${id ?? ""}`}
      tag="collection"
      description="Individual collection detail (kelpie.md §95-96) lands once the library database layer exists."
    />
  );
}
