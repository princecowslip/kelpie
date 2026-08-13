# Packaging

Linux packaging scripts and configuration (kelpie.md §123 Packaging). Phase 1
(§135) targets `.deb` and AppImage only (Stage B unit 5); Arch/AUR and Flatpak
packaging are out of scope until later phases.

## Phase 1: .deb and AppImage

Both formats are produced by Tauri's own bundler, so there is no separate
build script here yet — the configuration lives in:

- `apps/desktop/src-tauri/tauri.conf.json` — the `bundle` key: sets
  `targets: ["deb", "appimage"]` plus the `category`, `shortDescription`,
  and `longDescription` metadata Tauri uses to populate the `.deb` control
  file, and reuses the icons already generated under
  `apps/desktop/src-tauri/icons/`.
- `.github/workflows/release.yml` — CI that runs `npm run tauri build` on
  `ubuntu-latest` and uploads the resulting `.deb` and `.AppImage` as
  workflow artifacts. Triggered manually (`workflow_dispatch`) or on `v*`
  tag pushes, not on every PR/commit.

Local build: from `apps/desktop`, run `npm run tauri build` (a real, non-debug
build — the bundler step is skipped under `--debug`). Artifacts land under
`apps/desktop/src-tauri/target/release/bundle/{deb,appimage}/`.

This directory is reserved for future helper scripts (e.g. a Debian
`postinst`/`postrm`, or AppImage-specific tooling) if Tauri's built-in
bundler configuration stops being sufficient.
