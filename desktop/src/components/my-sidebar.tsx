import {
  RiCommandFill,
  RiLineChartFill,
  RiPulseAiFill,
} from "@remixicon/react";

export const MySidebar = () => {
  return (
    <div className="w-12 contain-content h-screen bg-accent p-2 border-r">
      <div id="sidebar-icons" className="flex flex-col justify-start gap-4">
        <RiCommandFill />
        <RiPulseAiFill strokeWidth={0.1} />
        <RiLineChartFill />
      </div>
    </div>
  );
};
