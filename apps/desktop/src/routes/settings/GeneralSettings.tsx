import { useEffect, useState } from "react";
import { getSettings, type Settings } from "../../lib/tauri";

// Settings → General (kelpie.md §130). The only persisted setting this phase
// is `theme` (see Appearance), so this section surfaces the raw persisted
// state as a sanity check that get_settings/set_settings round-trip through
// settings.toml, plus notes on what General will eventually hold.
export function GeneralSettings() {
  const [settings, setSettingsState] = useState<Settings | null>(null);

  useEffect(() => {
    let cancelled = false;
    getSettings().then((s) => {
      if (!cancelled) setSettingsState(s);
    });
    return () => {
      cancelled = true;
    };
  }, []);

  return (
    <section className="settings-panel">
      <h1>General</h1>
      <p className="settings-panel__lede">
        Application-wide behavior. Home/Feeds/Sources/Search preferences (kelpie.md §130) land
        alongside the features they configure in later phases.
      </p>
      <div className="field-row">
        <label>Persisted settings (settings.toml)</label>
        <span className="field-hint">
          {settings ? JSON.stringify(settings) : "Loading…"}
        </span>
      </div>
    </section>
  );
}
