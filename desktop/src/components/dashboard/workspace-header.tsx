import { useEffect, useState, type FC } from "react";
import { RiAddLine } from "@remixicon/react";
import type { DockviewApi } from "dockview-react";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { useWorkspaceRegistry } from "./workspace-registry";

const PANEL_OPTIONS = [
  { component: "imu", label: "IMU Table" },
  { component: "speed", label: "Time Series" },
  { component: "gps", label: "GPS Map" },
] as const;

let nextPanelId = 0;

interface WorkspaceHeaderProps {
  appApi: DockviewApi | undefined;
}

export const WorkspaceHeader: FC<WorkspaceHeaderProps> = ({ appApi }) => {
  const registry = useWorkspaceRegistry();
  const [activePageId, setActivePageId] = useState<string | undefined>();

  useEffect(() => {
    if (!appApi) return;

    setActivePageId(appApi.activePanel?.id);

    const disposable = appApi.onDidActivePanelChange((event) => {
      setActivePageId(event.panel?.id);
    });

    return () => disposable.dispose();
  }, [appApi]);

  const activeWorkspaceApi = activePageId ? registry.get(activePageId) : undefined;

  const addPanel = (component: string, label: string) => {
    activeWorkspaceApi?.addPanel({
      id: `${component}-${Date.now()}-${nextPanelId++}`,
      component,
      title: label,
    });
  };

  return (
    <div className="flex h-9 items-center justify-between border-b bg-muted px-3">
      <span className="text-xs font-medium text-muted-foreground">Open Race Telemetry</span>
      <DropdownMenu>
        <DropdownMenuTrigger
          render={<Button variant="outline" size="sm" disabled={!activeWorkspaceApi} />}
        >
          <RiAddLine className="size-3.5" />
          Add Panel
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          {PANEL_OPTIONS.map((option) => (
            <DropdownMenuItem
              key={option.component}
              onClick={() => addPanel(option.component, option.label)}
            >
              {option.label}
            </DropdownMenuItem>
          ))}
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
};

export default WorkspaceHeader;
