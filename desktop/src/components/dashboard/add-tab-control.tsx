import type { FC } from "react";
import { RiAddLine } from "@remixicon/react";
import type { IDockviewHeaderActionsProps } from "dockview-react";
import { Button } from "@/components/ui/button";

let nextPanelId = 0;

interface AddTabControlProps extends IDockviewHeaderActionsProps {
  component: string;
  titlePrefix: string;
}

/** Header action rendering a `+` button that appends a new panel next to this group. */
export const createAddTabControl = (
  component: string,
  titlePrefix: string,
): FC<IDockviewHeaderActionsProps> => {
  const AddTabControl: FC<AddTabControlProps> = (props) => {
    if (props.location?.type === "edge") {
      return null;
    }

    const onClick = () => {
      props.containerApi.addPanel({
        id: `${component}-${Date.now()}-${nextPanelId++}`,
        component,
        title: `${titlePrefix} ${nextPanelId}`,
        position: { referenceGroup: props.group },
      });
    };

    return (
      <div className="flex h-full items-center px-1">
        <Button
          variant="ghost"
          size="icon-sm"
          title="Add tab"
          onClick={onClick}
        >
          <RiAddLine />
        </Button>
      </div>
    );
  };

  return (props) => (
    <AddTabControl {...props} component={component} titlePrefix={titlePrefix} />
  );
};
