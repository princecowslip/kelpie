import { useTheme } from "../../theme/ThemeProvider";
import { ThemeToggle } from "../../components/ThemeToggle";

// Settings → Appearance (kelpie.md §130, §74-75 Visual Design / Visual
// Tokens). This is the real read/write path through get_settings/set_settings
// — ThemeToggle both drives and reflects the persisted preference.
export function AppearanceSettings() {
  const { preference, resolved } = useTheme();

  return (
    <section className="settings-panel">
      <h1>Appearance</h1>
      <p className="settings-panel__lede">
        Theme preference is written to settings.toml via the set_settings command and reloaded on
        startup via get_settings.
      </p>
      <div className="field-row">
        <label>Theme</label>
        <ThemeToggle />
        <span className="field-hint">
          Preference: {preference} · Currently applied: {resolved}
        </span>
      </div>
    </section>
  );
}
