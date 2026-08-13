import { useState } from "react";
import { NavLink, Outlet } from "react-router-dom";
import { settingsNavItems } from "../../lib/nav";

// /settings/* (kelpie.md §71 Route Model, §130 Settings). §130 lists the
// settings sections and says "Provide settings search" — the filter box
// below is that search, scoped to section names since individual setting
// fields don't exist yet for most sections.
export function SettingsLayout() {
  const [filter, setFilter] = useState("");
  const items = settingsNavItems.filter((item) =>
    item.label.toLowerCase().includes(filter.trim().toLowerCase()),
  );

  return (
    <div>
      <div className="page-header">
        <div>
          <h1>Settings</h1>
          <p>General application preferences (kelpie.md §130).</p>
        </div>
      </div>
      <div className="settings-layout">
        <div>
          <input
            className="settings-search"
            placeholder="Search settings…"
            value={filter}
            onChange={(e) => setFilter(e.currentTarget.value)}
            aria-label="Search settings"
          />
          <nav className="settings-nav" aria-label="Settings sections">
            {items.map((item) => (
              <NavLink
                key={item.slug}
                to={`/settings/${item.slug}`}
                className={({ isActive }) => `nav-link${isActive ? " is-active" : ""}`}
              >
                {item.label}
              </NavLink>
            ))}
            {items.length === 0 ? (
              <p className="field-hint" style={{ padding: "6px 8px" }}>
                No sections match "{filter}".
              </p>
            ) : null}
          </nav>
        </div>
        <Outlet />
      </div>
    </div>
  );
}
