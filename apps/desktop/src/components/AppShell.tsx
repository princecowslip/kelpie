import { Outlet, useLocation } from "react-router-dom";
import { NavSidebar } from "./NavSidebar";
import { ThemeToggle } from "./ThemeToggle";
import { navGroups } from "../lib/nav";

function currentTitle(pathname: string): string {
  for (const group of navGroups) {
    for (const item of group.items) {
      if (item.to === "/" ? pathname === "/" : pathname.startsWith(item.to)) {
        return item.label;
      }
    }
  }
  return "Kelpie";
}

export function AppShell() {
  const location = useLocation();

  return (
    <div className="app-shell">
      <NavSidebar />
      <header className="app-topbar">
        <span className="app-topbar__title">{currentTitle(location.pathname)}</span>
        <ThemeToggle />
      </header>
      <main className="app-main">
        <Outlet />
      </main>
    </div>
  );
}
