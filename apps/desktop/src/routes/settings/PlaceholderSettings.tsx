// Shared stub for the settings sections (kelpie.md §130) whose actual
// controls belong to a feature area outside this unit's scope (Video, GIFs,
// Privacy, Network, Storage, Keyboard, Advanced, About, etc).
export function PlaceholderSettings({ title }: { title: string }) {
  return (
    <section className="settings-panel">
      <h1>{title}</h1>
      <p className="settings-panel__lede">
        This settings section is routed but not implemented yet — its controls belong to features
        that land in later phases (kelpie.md §130, §135-148 Implementation Plan).
      </p>
    </section>
  );
}
