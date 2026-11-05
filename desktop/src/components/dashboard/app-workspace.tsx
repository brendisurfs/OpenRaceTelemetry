import { type FC } from "react";
import { DockviewReact, type DockviewReadyEvent } from "dockview-react";
import { PageWorkspace } from "./page-workspace";
import { CustomTabRenderComponent } from "./custom-tab-render";
import { dockviewAppTheme } from "./dockview-theme";

const components = {
  page: PageWorkspace,
};

function onReady(event: DockviewReadyEvent) {
  event.api.addPanel({
    id: "overview",
    title: "Overview",
    component: "page",
    params: { seedOverview: true },
  });
}

export const AppWorkspace: FC = () => {
  return (
    <DockviewReact
      className="h-full w-full"
      defaultTabComponent={CustomTabRenderComponent}
      components={components}
      theme={dockviewAppTheme}
      onReady={onReady}
    />
  );
};

export default AppWorkspace;
