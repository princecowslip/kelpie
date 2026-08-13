import { useState } from "react";
import { useParams } from "react-router-dom";
import { EmptyState } from "../components/EmptyState";

// Route stub for /search and /search/:savedSearch (kelpie.md §71 Route Model,
// §65-67 Search Architecture / Query Language / Saved Searches). The query
// engine is a later phase; this just gives the route somewhere real to live.
export function SearchPage() {
  const { savedSearch } = useParams<{ savedSearch?: string }>();
  const [query, setQuery] = useState("");

  return (
    <div>
      <div className="page-header">
        <div>
          <h1>Search</h1>
          <p>
            {savedSearch
              ? `Saved search: ${savedSearch}`
              : "Unified search across your sources (kelpie.md §65)."}
          </p>
        </div>
      </div>
      <div className="field-row" style={{ maxWidth: 420 }}>
        <input
          className="settings-search"
          placeholder="Search titles, tags, creators…"
          value={query}
          onChange={(e) => setQuery(e.currentTarget.value)}
        />
      </div>
      <EmptyState
        title="Search isn't wired up yet"
        message="The search index and query language (kelpie.md §65-67) land in a later phase. This route exists so navigation and saved-search links resolve correctly."
      />
    </div>
  );
}
