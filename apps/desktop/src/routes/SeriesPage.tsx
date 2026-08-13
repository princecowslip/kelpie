import { useParams } from "react-router-dom";
import { PlaceholderPage } from "../components/PlaceholderPage";

// /series/:uid (kelpie.md §71 Route Model, §20-22 Series Model / Series
// Object / Sequence Numbers, §85 Series Page).
export function SeriesPage() {
  const { uid } = useParams<{ uid: string }>();
  return (
    <PlaceholderPage
      title={`Series ${uid ?? ""}`}
      tag="series"
      description="The series page (kelpie.md §85) — chapters, sequencing, cross-source availability — lands in a later phase."
    />
  );
}
