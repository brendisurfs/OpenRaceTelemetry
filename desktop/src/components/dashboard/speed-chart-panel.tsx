import type { FC } from "react";
import { CartesianGrid, Line, LineChart, XAxis, YAxis } from "recharts";
import {
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
  type ChartConfig,
} from "@/components/ui/chart";
import { IDockviewPanelProps } from "dockview-react";

export type SpeedSample = {
  time: string;
  speedKph: number;
};

/** Placeholder series until this binds to a real telemetry speed stream. */
const PLACEHOLDER_SPEED_DATA: SpeedSample[] = Array.from(
  { length: 30 },
  (_, i) => {
    const t = i * 2;
    const speedKph = 140 + 90 * Math.sin(t / 6) - (i % 7 === 0 ? 60 : 0);
    return {
      time: `${String(Math.floor(t / 60)).padStart(2, "0")}:${String(t % 60).padStart(2, "0")}`,
      speedKph: Math.max(0, Math.round(speedKph)),
    };
  },
);

const speedChartConfig = {
  speedKph: {
    label: "Speed (km/h)",
    color: "var(--chart-2)",
  },
} satisfies ChartConfig;

interface SpeedChartPanelProps {
  data?: SpeedSample[];
}

export const SpeedChartPanel: FC<IDockviewPanelProps<SpeedChartPanelProps>> = ({
  params: { data = PLACEHOLDER_SPEED_DATA },
}) => {
  return (
    <div className="flex h-full flex-col p-2">
      <ChartContainer config={speedChartConfig} className="h-full w-full">
        <LineChart data={data}>
          <CartesianGrid vertical={false} />
          <XAxis dataKey="time" tickLine={false} axisLine={false} />
          <YAxis tickLine={false} axisLine={false} width={32} />
          <ChartTooltip content={<ChartTooltipContent />} />
          <Line
            dataKey="speedKph"
            type="monotone"
            stroke="var(--color-speedKph)"
            strokeWidth={2}
            dot={false}
          />
        </LineChart>
      </ChartContainer>
    </div>
  );
};

export default SpeedChartPanel;
