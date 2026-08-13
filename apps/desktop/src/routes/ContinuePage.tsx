import { EmptyState } from "../components/EmptyState";

// /continue (kelpie.md §71 Route Model, §59 Feed Modes — "continue" is one
// of the feed modes alongside global/following/etc).
export function ContinuePage() {
  return (
    <div>
      <div className="page-header">
        <div>
          <h1>Continue</h1>
          <p>Pick up where you left off (kelpie.md §59 Feed Modes, §84 Reading Progress).</p>
        </div>
      </div>
      <EmptyState
        title="Nothing in progress"
        message="Reading/watch progress tracking (kelpie.md §84) is backed by the local database, which lands in Phase 2."
      />
    </div>
  );
}
