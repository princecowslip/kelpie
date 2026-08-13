import { useEffect, useState } from "react";
import type { MediaItem } from "@kelpie/normalized-types";
import { getFakeFeed } from "../lib/tauri";
import { MediaCard } from "../components/MediaCard";
import { EmptyState } from "../components/EmptyState";

// Shared implementation for the Home (/) and Discover (/discover) routes —
// both are feed views over `get_fake_feed` (kelpie.md §60 Home Screen, §58-59
// Global Feed Architecture / Feed Modes), differing only in heading copy this
// phase since the provider-backed fake feed (Stage B unit 6) isn't filled in
// yet.
export function FeedPage({ title, description }: { title: string; description: string }) {
  const [items, setItems] = useState<MediaItem[] | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    getFakeFeed()
      .then((feed) => {
        if (!cancelled) setItems(feed);
      })
      .catch((err) => {
        console.error("get_fake_feed failed", err);
        if (!cancelled) {
          setError(String(err));
          setItems([]);
        }
      });
    return () => {
      cancelled = true;
    };
  }, []);

  return (
    <div>
      <div className="page-header">
        <div>
          <h1>{title}</h1>
          <p>{description}</p>
        </div>
      </div>

      {items === null ? (
        <p className="status-line">Loading feed…</p>
      ) : items.length === 0 ? (
        <EmptyState
          title="No items yet"
          message={
            error
              ? "The feed command returned an error, so nothing could be loaded."
              : "No providers have produced any fake feed items yet. Once the fake provider feed is wired up, items will appear here automatically."
          }
        />
      ) : (
        <div className="media-grid">
          {items.map((item) => (
            <MediaCard item={item} key={item.uid} />
          ))}
        </div>
      )}
      {error ? <p className="status-line is-error">{error}</p> : null}
    </div>
  );
}
