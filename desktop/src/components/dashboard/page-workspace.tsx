import { type FC } from "react";
import { DockviewReact, type DockviewReadyEvent, type IDockviewPanelProps } from "dockview-react";
import { RiLayoutGridLine } from "@remixicon/react";
import { ImuPanel } from "./imu-panel";
import { SpeedChartPanel } from "./speed-chart-panel";
import { GpsMapPanel } from "./gps-map-panel";
import { dockviewAppTheme } from "./dockview-theme";
import { CustomTabRenderComponent } from "./custom-tab-render";

const components = {
  imu: ImuPanel,
  gps: GpsMapPanel,
  speed: SpeedChartPanel,
};

const EmptyPageWatermark: FC = () => (
  <div className="flex h-full flex-col items-center justify-center gap-2 text-center text-muted-foreground">
    <RiLayoutGridLine className="size-8" />
    <p className="text-sm font-medium">Empty page</p>
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

/** A single top-level page's dockable panel grid. */
export const PageWorkspace: FC<IDockviewPanelProps<PageWorkspaceParams>> = ({ params }) => {
  const handleReady = (event: DockviewReadyEvent) => {
    if (params?.seedOverview) {
      seedOverviewPanels(event);
    }
  };

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
