import type { MediaItem } from "@kelpie/normalized-types";

// Card system (kelpie.md §72): visual preview, media indicator, title,
// source, secondary metadata. Phase 1's normalized MediaItem doesn't yet
// carry the richer per-kind metadata (duration, page count, etc.), so the
// badge is just the media kind rather than the full "21:43" / "Ch. 18 · 32p"
// style badges §72 specifies — those need fields this phase's shared type
// doesn't have yet.
export function MediaCard({ item }: { item: MediaItem }) {
  return (
    <article className="media-card">
      <div className="media-card__thumb">
        {item.thumbnailUrl ? (
          <img src={item.thumbnailUrl} alt="" loading="lazy" />
        ) : (
          <span>{item.kind.toUpperCase()}</span>
        )}
        <span className="media-card__badge">{item.kind}</span>
      </div>
      <div className="media-card__body">
        <span className="media-card__title" title={item.title}>
          {item.title}
        </span>
        <span className="media-card__meta">{item.providerLabel}</span>
      </div>
    </article>
  );
}
