import { useParams } from "react-router-dom";
import { PlaceholderPage } from "../components/PlaceholderPage";

// /source/:provider and /source/:provider/search (kelpie.md §71 Route Model,
// §23-33 Provider Model). The provider platform itself is a later phase.
export function SourcePage({ withSearch = false }: { withSearch?: boolean }) {
  const { provider } = useParams<{ provider: string }>();

  return (
    <PlaceholderPage
      title={provider ?? "Source"}
      tag={withSearch ? "source · search" : "source"}
      description={`Provider-scoped browsing${withSearch ? " and search" : ""} for "${provider}" will render here once the provider platform (kelpie.md §23-33) is implemented.`}
    />
  );
}
