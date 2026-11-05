import type { DockviewTheme } from "dockview-react";

/** Maps dockview's CSS variables onto the app's shadcn OKLCH tokens (see `.dockview-theme-app` in App.css). */
export const dockviewAppTheme: DockviewTheme = {
  name: "app",
  className: "dockview-theme-app py-1",
  dndTabIndicator: "line",
  tabGroupIndicator: "wrap",
  gap: 4,
};
