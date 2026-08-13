// Generic stub for routes that are in scope for navigation/routing this
// phase but whose real content lands in a later phase (kelpie.md §135-148).
export function PlaceholderPage({
  title,
  description,
  tag,
}: {
  title: string;
  description?: string;
  tag?: string;
}) {
  return (
    <div>
      <div className="page-header">
        <h1>{title}</h1>
      </div>
      <div className="placeholder-panel">
        {tag ? <p style={{ marginBottom: 10 }}><span className="tag">{tag}</span></p> : null}
        <p>
          {description ??
            "This screen is routed and reachable, but its real content isn't built yet in Phase 1 (kelpie.md §135)."}
        </p>
      </div>
    </div>
  );
}
