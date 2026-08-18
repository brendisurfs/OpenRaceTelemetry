import type { FC } from "react";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";

interface ViewerSquareProps {
  title?: string;
}

const ViewerSquare: FC<ViewerSquareProps> = ({ title }) => {
  return (
    <div className="grow  m-4 h-screen">
      <ResizablePanelGroup orientation="horizontal">
        <ResizablePanel minSize="25">One</ResizablePanel>
        <ResizableHandle />
        <ResizablePanel minSize="25">Two</ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
};

export default ViewerSquare;
