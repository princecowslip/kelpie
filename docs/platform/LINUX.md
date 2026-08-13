# Linux Integration

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Desktop standards

Kelpie is specified to follow Linux desktop standards for (kelpie.md §121):

- XDG paths
- Desktop file
- Wayland
- X11
- System theme
- File dialogs
- Notifications
- MPRIS
- Media keys
- Flatpak portals

The spec does not elaborate beyond this list — no specific desktop-file fields, MPRIS interface details, notification content, or portal names are given in §121. Treat each item as a named integration point to be designed when implementation begins, not as an already-specified behavior.

## Filesystem layout

The XDG Base Directory Specification governs Kelpie's on-disk storage layout (kelpie.md §122):

```
$XDG_CONFIG_HOME/kelpie/
    settings.toml

$XDG_DATA_HOME/kelpie/
    kelpie.sqlite
    providers/
    downloads/

$XDG_CACHE_HOME/kelpie/
    thumbnails/
    pages/
    media/
    responses/

$XDG_STATE_HOME/kelpie/
    logs/
```

- `$XDG_CONFIG_HOME/kelpie/` holds user configuration (`settings.toml`).
- `$XDG_DATA_HOME/kelpie/` holds the SQLite database, per-provider data, and downloaded media.
- `$XDG_CACHE_HOME/kelpie/` holds regenerable cached data: thumbnails, cached pages, cached media, and cached responses.
- `$XDG_STATE_HOME/kelpie/` holds logs.

## See also

- [`FLATPAK.md`](./FLATPAK.md) for Flatpak packaging and how it relates to the portals mentioned above.
- [`DEBIAN.md`](./DEBIAN.md), [`UBUNTU.md`](./UBUNTU.md), [`ARCH.md`](./ARCH.md) for distribution-specific packaging.
- [`../development/BUILDING.md`](../development/BUILDING.md) for build instructions.
