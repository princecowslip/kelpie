// Thin wrappers around the Tauri commands this unit owns or consumes.
// `get_settings` / `set_settings` are implemented (real persistence) in
// `apps/desktop/src-tauri/src/commands/settings.rs` by this same unit.
// `get_fake_feed` is a stub owned by another Stage B unit — it currently
// always returns an empty array, so callers must render an empty state
// without erroring.

import { invoke } from "@tauri-apps/api/core";
import type { MediaItem } from "@kelpie/normalized-types";

export type ThemePreference = "system" | "light" | "dark";

export interface Settings {
  theme: ThemePreference;
}

export const DEFAULT_SETTINGS: Settings = { theme: "system" };

export async function getSettings(): Promise<Settings> {
  try {
    const settings = await invoke<Settings>("get_settings");
    return settings ?? DEFAULT_SETTINGS;
  } catch (error) {
    console.error("get_settings failed; falling back to defaults", error);
    return DEFAULT_SETTINGS;
  }
}

export async function setSettings(settings: Settings): Promise<void> {
  await invoke("set_settings", { settings });
}

// The backend stub (`commands/feed.rs`) serializes its Rust struct fields
// as-is (snake_case), which does not match the camelCase `MediaItem`
// contract from `@kelpie/normalized-types`. Since that stub is owned by a
// different parallel unit and may change field naming when it's filled in,
// normalize defensively here rather than assuming either casing.
interface RawFeedItem {
  uid?: string;
  title?: string;
  kind?: string;
  thumbnailUrl?: string;
  thumbnail_url?: string;
  providerLabel?: string;
  provider_label?: string;
}

function normalizeFeedItem(raw: RawFeedItem): MediaItem | null {
  if (!raw.uid || !raw.title || !raw.kind) {
    return null;
  }
  return {
    uid: raw.uid,
    title: raw.title,
    kind: raw.kind as MediaItem["kind"],
    thumbnailUrl: raw.thumbnailUrl ?? raw.thumbnail_url,
    providerLabel: raw.providerLabel ?? raw.provider_label ?? "Unknown source",
  };
}

export async function getFakeFeed(): Promise<MediaItem[]> {
  const raw = await invoke<RawFeedItem[]>("get_fake_feed");
  return raw.map(normalizeFeedItem).filter((item): item is MediaItem => item !== null);
}
