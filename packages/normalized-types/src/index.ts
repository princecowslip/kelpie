// @kelpie/normalized-types: shared TypeScript types mirroring the normalized media
// model (kelpie.md §12-16 Core Media Taxonomy / Unified MediaItem). Minimal Phase 1
// shape — the shared contract between the frontend shell (Stage B unit 1, read-only)
// and the fake provider feed (Stage B unit 6, may extend with additional optional
// fields as needed).

export type MediaKind =
  | "video"
  | "image"
  | "gif"
  | "manga"
  | "comic"
  | "literature"
  | "audio";

export interface MediaItem {
  uid: string;
  title: string;
  kind: MediaKind;
  thumbnailUrl?: string;
  providerLabel: string;
  /** Optional short synopsis (kelpie.md §16 `MediaItem.description`). Added by
   * the fake provider feed unit; existing required fields are unchanged. */
  description?: string;
}
