import type { FC } from "react";
import {
  PolarAngleAxis,
  PolarGrid,
  Radar,
  RadarChart,
} from "recharts";
import {
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
  type ChartConfig,
} from "@/components/ui/chart";
import {
  Table,
  TableBody,
  TableCell,
  TableRow,
} from "@/components/ui/table";
import type { ImuDataDto } from "@/bindings";

const PLACEHOLDER_IMU: ImuDataDto = {
  accelX: 120,
  accelY: -430,
  accelZ: 16240,
  tempRaw: 3120,
  gyroX: -85,
  gyroY: 210,
  gyroZ: 12,
};

/** Placeholder raw->display conversion. Real scale factors come from the sensor datasheet later. */
function deriveMotionSummary(imu: ImuDataDto) {
  const gForce =
    Math.sqrt(imu.accelX ** 2 + imu.accelY ** 2 + imu.accelZ ** 2) / 16384;
  const leanAngle =
    (Math.atan2(imu.accelY, imu.accelZ) * 180) / Math.PI;

  return { gForce, leanAngle };
}

const radarChartConfig = {
  value: {
    label: "Motion",
    color: "var(--chart-1)",
  },
} satisfies ChartConfig;

interface ImuPanelProps {
  imu?: ImuDataDto;
}

export const ImuPanel: FC<ImuPanelProps> = ({ imu = PLACEHOLDER_IMU }) => {
  const { gForce, leanAngle } = deriveMotionSummary(imu);

  const radarData = [
    { axis: "G-Force", value: Math.min(gForce * 25, 100) },
    { axis: "Lean Angle", value: Math.min(Math.abs(leanAngle), 100) },
    { axis: "Gyro X", value: Math.min(Math.abs(imu.gyroX) / 10, 100) },
    { axis: "Gyro Y", value: Math.min(Math.abs(imu.gyroY) / 10, 100) },
    { axis: "Gyro Z", value: Math.min(Math.abs(imu.gyroZ) / 10, 100) },
  ];

  const rows: Array<[label: string, value: number]> = [
    ["accelX", imu.accelX],
    ["accelY", imu.accelY],
    ["accelZ", imu.accelZ],
    ["gyroX", imu.gyroX],
    ["gyroY", imu.gyroY],
    ["gyroZ", imu.gyroZ],
    ["tempRaw", imu.tempRaw],
  ];

  return (
    <div className="flex h-full flex-col overflow-auto">
      <Table>
        <TableBody>
          {rows.map(([label, value]) => (
            <TableRow key={label}>
              <TableCell className="text-muted-foreground">{label}</TableCell>
              <TableCell className="text-right font-mono tabular-nums">
                {value}
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>

      <div className="flex flex-col gap-1 border-t p-2">
        <div className="flex justify-between text-xs text-muted-foreground">
          <span>G-Force: {gForce.toFixed(2)}g</span>
          <span>Lean: {leanAngle.toFixed(1)}°</span>
        </div>
        <ChartContainer config={radarChartConfig} className="mx-auto aspect-square max-h-52">
          <RadarChart data={radarData}>
            <ChartTooltip content={<ChartTooltipContent />} />
            <PolarGrid />
            <PolarAngleAxis dataKey="axis" />
            <Radar
              dataKey="value"
              fill="var(--color-value)"
              fillOpacity={0.5}
              stroke="var(--color-value)"
            />
          </RadarChart>
        </ChartContainer>
      </div>
    </div>
  );
};

export default ImuPanel;
