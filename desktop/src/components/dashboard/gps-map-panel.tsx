import type { FC } from "react";
import { RiMapPin2Line } from "@remixicon/react";
import { IDockviewPanelProps } from "dockview-react";

/**
 * Shaped to be trivially derivable from `GPGGASentenceDto` once wired up:
 * `latitude`/`longitude` map directly; sign fields fold into the numeric value.
 */
export type GpsCoordinate = {
  latitude: number;
  longitude: number;
  timestamp: number;
};

const PLACEHOLDER_COORDINATE: GpsCoordinate = {
  latitude: 29.184046,
  longitude: -81.070132,
  timestamp: Date.now(),
};

interface GpsMapPanelProps {
  coordinate?: GpsCoordinate;
}

export const GpsMapPanel: FC<IDockviewPanelProps<GpsMapPanelProps>> = ({
  params: { coordinate = PLACEHOLDER_COORDINATE },
}) => {
  return (
    <div className="flex h-full flex-col items-center justify-center gap-2 bg-muted/30 p-4 text-center">
      <RiMapPin2Line className="size-8 text-muted-foreground" />
      <p className="text-sm font-medium">Map view — React Leaflet pending</p>
      <p className="text-xs text-muted-foreground">
        Tile caching will be handled by the backend
      </p>
      <div className="mt-2 flex gap-4 font-mono text-xs tabular-nums text-muted-foreground">
        <span>lat: {coordinate.latitude.toFixed(6)}</span>
        <span>lon: {coordinate.longitude.toFixed(6)}</span>
      </div>
    </div>
  );
};

export default GpsMapPanel;
