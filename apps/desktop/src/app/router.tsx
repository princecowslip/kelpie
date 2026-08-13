import { HashRouter, Navigate, Route, Routes } from "react-router-dom";
import { AppShell } from "../components/AppShell";
import { FeedPage } from "../routes/FeedPage";
import { ContinuePage } from "../routes/ContinuePage";
import { SearchPage } from "../routes/SearchPage";
import { SourcePage } from "../routes/SourcePage";
import { ItemPage } from "../routes/ItemPage";
import { SeriesPage } from "../routes/SeriesPage";
import { ChapterPage } from "../routes/ChapterPage";
import { LibraryPage, LibraryCollectionPage } from "../routes/LibraryPage";
import { BrowserPage } from "../routes/BrowserPage";
import { MediaKindPage } from "../routes/MediaKindPage";
import { SourcesIndexPage, SourcesCategoryPage, AddSourcePage } from "../routes/SourcesIndexPage";
import { SettingsLayout } from "../routes/settings/SettingsLayout";
import { GeneralSettings } from "../routes/settings/GeneralSettings";
import { AppearanceSettings } from "../routes/settings/AppearanceSettings";
import { PlaceholderSettings } from "../routes/settings/PlaceholderSettings";
import { settingsNavItems } from "../lib/nav";

// Route Model (kelpie.md §71), plus a handful of nav-support routes not
// literally in §71 — see routes/MediaKindPage.tsx and
// routes/SourcesIndexPage.tsx for why. HashRouter is used because Tauri
// serves the SPA from a single entry point with no server-side rewrite rules
// for arbitrary deep paths.
export function AppRouter() {
  return (
    <HashRouter>
      <Routes>
        <Route element={<AppShell />}>
          <Route path="/" element={<FeedPage title="Home" description="Your global feed (kelpie.md §60 Home Screen)." />} />
          <Route
            path="/discover"
            element={<FeedPage title="Discover" description="Explore beyond your usual sources (kelpie.md §59 Feed Modes)." />}
          />
          <Route path="/continue" element={<ContinuePage />} />

          <Route path="/search" element={<SearchPage />} />
          <Route path="/search/:savedSearch" element={<SearchPage />} />

          <Route path="/source/:provider" element={<SourcePage />} />
          <Route path="/source/:provider/search" element={<SourcePage withSearch />} />

          <Route path="/item/:uid" element={<ItemPage />} />

          <Route path="/series/:uid" element={<SeriesPage />} />
          <Route path="/chapter/:uid" element={<ChapterPage />} />

          <Route
            path="/library/saved"
            element={<LibraryPage title="Saved" description="Items you've saved for later (kelpie.md §94)." />}
          />
          <Route
            path="/library/series"
            element={<LibraryPage title="Series" description="Series you're tracking (kelpie.md §20-22)." />}
          />
          <Route
            path="/library/collections"
            element={<LibraryPage title="Collections" description="Your custom collections (kelpie.md §95)." />}
          />
          <Route path="/library/collection/:id" element={<LibraryCollectionPage />} />
          <Route
            path="/library/following"
            element={<LibraryPage title="Following" description="Creators and series you follow (kelpie.md §97)." />}
          />
          <Route
            path="/library/history"
            element={<LibraryPage title="History" description="Your recently viewed items (kelpie.md §98)." />}
          />
          <Route
            path="/library/downloads"
            element={<LibraryPage title="Downloads" description="Offline media (kelpie.md §108-109)." />}
          />
          <Route
            path="/library/local-files"
            element={<LibraryPage title="Local Files" description="Media imported from disk." />}
          />

          <Route path="/browser/:tab" element={<BrowserPage />} />

          <Route path="/media/:kind" element={<MediaKindPage />} />

          <Route path="/sources" element={<SourcesIndexPage />} />
          <Route path="/sources/add" element={<AddSourcePage />} />
          <Route path="/sources/:category" element={<SourcesCategoryPage />} />

          <Route path="/settings" element={<SettingsLayout />}>
            <Route index element={<Navigate to="general" replace />} />
            <Route path="general" element={<GeneralSettings />} />
            <Route path="appearance" element={<AppearanceSettings />} />
            {settingsNavItems
              .filter((item) => item.slug !== "general" && item.slug !== "appearance")
              .map((item) => (
                <Route
                  key={item.slug}
                  path={item.slug}
                  element={<PlaceholderSettings title={item.label} />}
                />
              ))}
          </Route>

          <Route path="*" element={<Navigate to="/" replace />} />
        </Route>
      </Routes>
    </HashRouter>
  );
}
