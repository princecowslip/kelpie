import { NavLink } from "react-router-dom";
import { navGroups } from "../lib/nav";

export function NavSidebar() {
  // "Empty groups disappear" (kelpie.md §70). Every group is populated today,
  // but this filter is what actually enforces that rule as the nav grows.
  const visibleGroups = navGroups.filter((group) => group.items.length > 0);

  return (
    <nav className="app-sidebar" aria-label="Primary">
      <div className="app-sidebar__brand">
        <span className="app-sidebar__brand-mark" aria-hidden="true" />
        <span>Kelpie</span>
      </div>

      {visibleGroups.map((group) => (
        <div className="nav-group" key={group.label}>
          <div className="nav-group__label">{group.label}</div>
          {group.items.map((item) => (
            <NavLink
              key={item.to}
              to={item.to}
              end={item.to === "/"}
              className={({ isActive }) => `nav-link${isActive ? " is-active" : ""}`}
            >
              {item.label}
            </NavLink>
          ))}
        </div>
      ))}
    </nav>
  );
}
