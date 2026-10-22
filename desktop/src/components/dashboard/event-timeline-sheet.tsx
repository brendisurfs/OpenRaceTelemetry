import type { FC } from "react";
import { CartesianGrid, Scatter, ScatterChart, XAxis, YAxis, ZAxis } from "recharts";
import { Sheet, SheetContent, SheetHeader, SheetTitle, SheetTrigger } from "@/components/ui/sheet";
import { Button } from "@/components/ui/button";
import {
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
  type ChartConfig,
} from "@/components/ui/chart";
import type { ReadEvent } from "@/bindings";

type TimestampedEvent = {
  event: ReadEvent["event"];
  time: number;
};

/** Placeholder timeline shaped like the real `ReadEvent` channel payloads. */
const PLACEHOLDER_EVENTS: TimestampedEvent[] = [
  { event: "usbFound", time: 0 },
  { event: "started", time: 2 },
  { event: "progress", time: 8 },
  { event: "progress", time: 14 },
  { event: "progress", time: 21 },
  { event: "finished", time: 27 },
];

const EVENT_ROW: Record<ReadEvent["event"], number> = {
  usbFound: 0,
  started: 1,
  progress: 2,
  finished: 3,
};

const eventChartConfig = {
  event: {
    label: "Events",
    color: "var(--chart-3)",
  },
} satisfies ChartConfig;

interface EventTimelineSheetProps {
  events?: TimestampedEvent[];
}

export const EventTimelineSheet: FC<EventTimelineSheetProps> = ({
  events = PLACEHOLDER_EVENTS,
}) => {
  const data = events.map((e) => ({
    time: e.time,
    row: EVENT_ROW[e.event],
    event: e.event,
  }));

  return (
    <Sheet>
      <div className="flex h-9 items-center justify-between border-t bg-card px-3">
        <span className="text-xs text-muted-foreground">Events ({events.length})</span>
        <SheetTrigger render={<Button variant="ghost" size="sm" />}>Timeline</SheetTrigger>
      </div>
      <SheetContent side="bottom" className="h-64">
        <SheetHeader>
          <SheetTitle>Telemetry Event Timeline</SheetTitle>
        </SheetHeader>
        <div className="flex-1 p-4 pt-0">
          <ChartContainer config={eventChartConfig} className="h-52 w-full">
            <ScatterChart>
              <CartesianGrid />
              <XAxis
                type="number"
                dataKey="time"
                name="time"
                unit="s"
                tickLine={false}
                axisLine={false}
              />
              <YAxis
                type="number"
                dataKey="row"
                name="event"
                tickLine={false}
                axisLine={false}
                width={0}
                tick={false}
              />
              <ZAxis range={[80, 80]} />
              <ChartTooltip cursor={{ strokeDasharray: "3 3" }} content={<ChartTooltipContent />} />
              <Scatter data={data} fill="var(--color-event)" />
            </ScatterChart>
          </ChartContainer>
        </div>
      </SheetContent>
    </Sheet>
  );
};

export default EventTimelineSheet;
