import { createContext, useContext, useRef, type FC, type ReactNode } from "react";
import type { DockviewApi } from "dockview-react";

/**
 * Maps a top-level page panel id -> that page's inner DockviewApi, so a
 * global header action can add a panel to whichever page is active without
 * threading props through the outer/inner dockview boundary.
 */
interface WorkspaceRegistry {
  register: (pageId: string, api: DockviewApi) => void;
  unregister: (pageId: string) => void;
  get: (pageId: string) => DockviewApi | undefined;
}

const WorkspaceRegistryContext = createContext<WorkspaceRegistry | null>(null);

export const WorkspaceRegistryProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const registryRef = useRef(new Map<string, DockviewApi>());

  const value: WorkspaceRegistry = {
    register: (pageId, api) => registryRef.current.set(pageId, api),
    unregister: (pageId) => registryRef.current.delete(pageId),
    get: (pageId) => registryRef.current.get(pageId),
  };

  return (
    <WorkspaceRegistryContext.Provider value={value}>{children}</WorkspaceRegistryContext.Provider>
  );
};

export function useWorkspaceRegistry(): WorkspaceRegistry {
  const context = useContext(WorkspaceRegistryContext);
  if (!context) {
    throw new Error("useWorkspaceRegistry must be used within a WorkspaceRegistryProvider");
  }
  return context;
}
