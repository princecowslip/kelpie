import { useParams } from "react-router-dom";
import { PlaceholderPage } from "../components/PlaceholderPage";

// /sources and /sources/:category — support the SOURCES nav group (kelpie.md
// §70). Not literally in §71's Route Model, which only has /source/:provider
// (a single already-known provider) rather than category listing pages.
// Added so "All Sources" and each SOURCES sub-item have a real destination;
// real content needs the provider registry (Phase 3).
export function SourcesIndexPage() {
  return (
    <PlaceholderPage
      title="All Sources"
      tag="sources"
      description="The provider registry browser (kelpie.md §34-57) lands in Phase 3."
    />
  );
}

export function SourcesCategoryPage() {
  const { category } = useParams<{ category: string }>();
  return (
    <PlaceholderPage
      title={category ?? "Sources"}
      tag="sources · category"
      description="Category-filtered source browsing lands alongside the provider registry (kelpie.md §34-57, Phase 3)."
    />
  );
}

export function AddSourcePage() {
  return (
    <PlaceholderPage
      title="Add Source"
      tag="sources · add"
      description="The custom source builder (kelpie.md §55-57 Custom Source Builder / Visual Site Builder / Manga Source Builder) lands in a later phase."
    />
  );
}
