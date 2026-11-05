import { FC } from "react";

import { IDockviewPanelHeaderProps, DockviewDefaultTab } from "dockview-react";
import { cn } from "@/lib/utils";

export const CustomTabRenderComponent: FC<IDockviewPanelHeaderProps> = (props) => {
  const onContextMenu = (event: React.MouseEvent) => {
    event.preventDefault();
    console.log("context menu", props.api.id);
  };

  return (
    <DockviewDefaultTab
      onContextMenu={onContextMenu}
      hideClose
      className={cn(props.api.isFocused && "border-b")}
      {...props}
    />
  );
};
