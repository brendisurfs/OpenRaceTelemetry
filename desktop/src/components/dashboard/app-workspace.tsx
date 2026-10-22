import { useState, type FC } from "react";
import { DockviewReact, type DockviewApi, type DockviewReadyEvent } from "dockview-react";
import { PageWorkspace } from "./page-workspace";
import { CustomTabRenderComponent } from "./custom-tab-render";
import { dockviewAppTheme } from "./dockview-theme";
import { createAddTabControl } from "./add-tab-control";
import { WorkspaceRegistryProvider } from "./workspace-registry";
import { WorkspaceHeader } from "./workspace-header";

const components = {
  page: PageWorkspace,
};

const AddPageControl = createAddTabControl("page", "Page");

function onReady(event: DockviewReadyEvent) {
  event.api.addPanel({
    id: "overview",
    title: "Overview",
    component: "page",
    params: { seedOverview: true },
  });
}

export const AppWorkspace: FC = () => {
  const [appApi, setAppApi] = useState<DockviewApi>();

  const handleReady = (event: DockviewReadyEvent) => {
    onReady(event);
    setAppApi(event.api);
  };

  return (
    <WorkspaceRegistryProvider>
      <div className="flex h-full w-full flex-col">
        <WorkspaceHeader appApi={appApi} />
        <div className="min-h-0 flex-1">
          <DockviewReact
            className="h-full w-full"
            defaultTabComponent={CustomTabRenderComponent}
            leftHeaderActionsComponent={AddPageControl}
            components={components}
            theme={dockviewAppTheme}
            onReady={handleReady}
          />
        </div>
      </div>
    </WorkspaceRegistryProvider>
  );
};

export default AppWorkspace;
