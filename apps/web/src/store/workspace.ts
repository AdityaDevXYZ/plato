import { create } from 'zustand';

interface WorkspaceState {
  currentWorkspaceId: string | null;
  setWorkspace: (id: string) => void;
}

export const useWorkspaceStore = create<WorkspaceState>((set) => ({
  currentWorkspaceId: null,
  setWorkspace: (id) => set({ currentWorkspaceId: id }),
}));
