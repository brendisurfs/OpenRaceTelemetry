import { useEffect, type FC } from "react";
import { DockviewReact, type DockviewReadyEvent, type IDockviewPanelProps } from "dockview-react";
import { RiLayoutGridLine } from "@remixicon/react";
import { ImuPanel } from "./imu-panel";
import { SpeedChartPanel } from "./speed-chart-panel";
import { GpsMapPanel } from "./gps-map-panel";
import { dockviewAppTheme } from "./dockview-theme";
import { CustomTabRenderComponent } from "./custom-tab-render";
import { useWorkspaceRegistry } from "./workspace-registry";

const components = {
  imu: ImuPanel,
  gps: GpsMapPanel,
  speed: SpeedChartPanel,
};

const EmptyPageWatermark: FC = () => (
  <div className="flex h-full flex-col items-center justify-center gap-2 text-center text-muted-foreground">
    <RiLayoutGridLine className="size-8" />
    <p className="text-sm font-medium">Empty page</p>
    <p className="text-xs">Use the header's Add Panel button to get started</p>
  </div>
);

function seedOverviewPanels(event: DockviewReadyEvent) {
  const imuPanel = event.api.addPanel({
    id: "imu",
    component: "imu",
    title: "IMU Table",
    inactive: true,
  });

  event.api.addPanel({
    id: "speed",
    component: "speed",
    title: "Time Series",
    position: { referencePanel: imuPanel, direction: "below" },
  });

  event.api.addPanel({
    id: "gps",
    component: "gps",
    title: "GPS Map",
    position: { direction: "left" },
  });
}

interface PageWorkspaceParams {
  seedOverview?: boolean;
}

/**
 * A single top-level page's dockable panel grid. Registers its inner
 * DockviewApi under its own outer panel id so the global header can target
 * "add a panel to the active page" regardless of which page is active.
 */
export const PageWorkspace: FC<IDockviewPanelProps<PageWorkspaceParams>> = ({ api, params }) => {
  const registry = useWorkspaceRegistry();

  const handleReady = (event: DockviewReadyEvent) => {
    if (params?.seedOverview) {
      seedOverviewPanels(event);
    }
    registry.register(api.id, event.api);
  };

  useEffect(() => {
    return () => registry.unregister(api.id);
  }, [api.id, registry]);

  return (
    <DockviewReact
      defaultTabComponent={CustomTabRenderComponent}
      watermarkComponent={EmptyPageWatermark}
      className="h-full w-full"
      theme={dockviewAppTheme}
      components={components}
      onReady={handleReady}
    />
  );
};

export default PageWorkspace;
