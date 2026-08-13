// Navigation model (kelpie.md §70 Navigation). Groups and items are data, not
// hard-coded JSX, so "empty groups disappear" (§70's closing line) is a real
// property of the renderer (see components/NavSidebar.tsx) rather than
// something asserted only in this phase's fixed list.
//
// The Route Model (§71) does not define per-category listing routes for the
// MEDIA and SOURCES nav groups (it only has per-item detail routes like
// /source/:provider once a specific provider is known). This unit adds a
// small number of index/listing routes not literally in §71
// (/media/:kind, /sources, /sources/:category, /library/local-files) so
// every nav item has somewhere to go; they render placeholder panels.

export interface NavItem {
  label: string;
  to: string;
}

export interface NavGroup {
  label: string;
  items: NavItem[];
}

export const navGroups: NavGroup[] = [
  {
    label: "Home",
    items: [
      { label: "Home", to: "/" },
      { label: "Discover", to: "/discover" },
      { label: "Continue", to: "/continue" },
      { label: "Search", to: "/search" },
    ],
  },
  {
    label: "Media",
    items: [
      { label: "Video", to: "/media/video" },
      { label: "GIFs", to: "/media/gif" },
      { label: "Images", to: "/media/image" },
      { label: "Manga", to: "/media/manga" },
      { label: "Comics", to: "/media/comic" },
      { label: "Cartoons", to: "/media/cartoon" },
      { label: "Literature", to: "/media/literature" },
      { label: "Audio", to: "/media/audio" },
    ],
  },
  {
    label: "Sources",
    items: [
      { label: "All Sources", to: "/sources" },
      { label: "Video", to: "/sources/video" },
      { label: "Images & Booru", to: "/sources/images-booru" },
      { label: "Manga & Comics", to: "/sources/manga-comics" },
      { label: "Literature", to: "/sources/literature" },
      { label: "Audio", to: "/sources/audio" },
      { label: "Creator Platforms", to: "/sources/creator-platforms" },
      { label: "Live", to: "/sources/live" },
      { label: "+ Add Source", to: "/sources/add" },
    ],
  },
  {
    label: "Library",
    items: [
      { label: "Saved", to: "/library/saved" },
      { label: "Series", to: "/library/series" },
      { label: "Collections", to: "/library/collections" },
      { label: "Following", to: "/library/following" },
      { label: "History", to: "/library/history" },
      { label: "Downloads", to: "/library/downloads" },
      { label: "Local Files", to: "/library/local-files" },
    ],
  },
  {
    label: "System",
    items: [{ label: "Settings", to: "/settings" }],
  },
];

// Settings sub-navigation (kelpie.md §130 Settings).
export interface SettingsNavItem {
  label: string;
  slug: string;
}

export const settingsNavItems: SettingsNavItem[] = [
  { label: "General", slug: "general" },
  { label: "Appearance", slug: "appearance" },
  { label: "Home", slug: "home" },
  { label: "Feeds", slug: "feeds" },
  { label: "Sources", slug: "sources" },
  { label: "Search", slug: "search" },
  { label: "Video", slug: "video" },
  { label: "GIFs", slug: "gifs" },
  { label: "Images", slug: "images" },
  { label: "Manga & Comics", slug: "manga-comics" },
  { label: "Literature", slug: "literature" },
  { label: "Audio", slug: "audio" },
  { label: "Library", slug: "library" },
  { label: "Downloads", slug: "downloads" },
  { label: "Privacy", slug: "privacy" },
  { label: "Network", slug: "network" },
  { label: "Storage", slug: "storage" },
  { label: "Keyboard", slug: "keyboard" },
  { label: "Advanced", slug: "advanced" },
  { label: "About", slug: "about" },
];
