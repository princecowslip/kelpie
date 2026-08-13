// Light/dark theme state (kelpie.md §75 Visual Tokens). The user's preference
// ("system" | "light" | "dark") is persisted through the `get_settings` /
// `set_settings` Tauri commands into settings.toml; the *resolved* theme
// (what actually gets applied) tracks the OS preference live when "system"
// is selected.

import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";
import { getSettings, setSettings, type ThemePreference } from "../lib/tauri";

type ResolvedTheme = "light" | "dark";

interface ThemeContextValue {
  preference: ThemePreference;
  resolved: ResolvedTheme;
  setPreference: (preference: ThemePreference) => void;
}

const ThemeContext = createContext<ThemeContextValue | null>(null);

function prefersDark(): boolean {
  return (
    typeof window !== "undefined" &&
    window.matchMedia?.("(prefers-color-scheme: dark)").matches === true
  );
}

function resolveTheme(preference: ThemePreference): ResolvedTheme {
  return preference === "system" ? (prefersDark() ? "dark" : "light") : preference;
}

export function ThemeProvider({ children }: { children: ReactNode }) {
  const [preference, setPreferenceState] = useState<ThemePreference>("system");
  const [resolved, setResolved] = useState<ResolvedTheme>(() => resolveTheme("system"));
  const [loaded, setLoaded] = useState(false);

  // Load the persisted preference once on startup.
  useEffect(() => {
    let cancelled = false;
    getSettings().then((settings) => {
      if (cancelled) return;
      setPreferenceState(settings.theme);
      setResolved(resolveTheme(settings.theme));
      setLoaded(true);
    });
    return () => {
      cancelled = true;
    };
  }, []);

  // Track the OS preference live while "system" is selected.
  useEffect(() => {
    if (preference !== "system" || typeof window === "undefined") return;
    const media = window.matchMedia("(prefers-color-scheme: dark)");
    const listener = () => setResolved(resolveTheme("system"));
    media.addEventListener("change", listener);
    return () => media.removeEventListener("change", listener);
  }, [preference]);

  // Apply to the document root so CSS custom properties in tokens.css switch.
  useEffect(() => {
    document.documentElement.setAttribute("data-theme", resolved);
  }, [resolved]);

  const setPreference = useCallback((next: ThemePreference) => {
    setPreferenceState(next);
    setResolved(resolveTheme(next));
    void setSettings({ theme: next }).catch((error) => {
      console.error("Failed to persist theme preference", error);
    });
  }, []);

  const value = useMemo<ThemeContextValue>(
    () => ({ preference, resolved, setPreference }),
    [preference, resolved, setPreference],
  );

  // Avoid a flash of the wrong theme while the persisted preference loads.
  if (!loaded) {
    return null;
  }

  return <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>;
}

export function useTheme(): ThemeContextValue {
  const ctx = useContext(ThemeContext);
  if (!ctx) {
    throw new Error("useTheme must be used within a ThemeProvider");
  }
  return ctx;
}
