import { useParams } from "react-router-dom";
import { PlaceholderPage } from "../components/PlaceholderPage";

// /browser/:tab (kelpie.md §71 Route Model, §91 Embedded Browser). The
// isolated browser fallback is Phase 7 — no live/embedded webview content
// belongs here in Phase 1 (see CLAUDE.md content boundaries: no scraping).
export function BrowserPage() {
  const { tab } = useParams<{ tab: string }>();
  return (
    <PlaceholderPage
      title={`Browser — ${tab ?? "tab"}`}
      tag="browser"
      description="The isolated embedded browser fallback (kelpie.md §91, Phase 7) is not built yet."
    />
  );
}
