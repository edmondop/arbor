import type { Repository, Worktree, TerminalSession, ChangedFile, ProcessInfo } from "./types";
import { fetchRepositories, fetchWorktrees, fetchTerminals, fetchChangedFiles, fetchProcesses } from "./api";

export type AppState = {
  repositories: Repository[];
  worktrees: Worktree[];
  sessions: TerminalSession[];
  changedFiles: ChangedFile[];
  processes: ProcessInfo[];

  selectedRepoRoot: string | null;
  selectedWorktreePath: string | null;
  activeSessionId: string | null;

  loading: boolean;
  error: string | null;
};

export function createInitialState(): AppState {
  return {
    repositories: [],
    worktrees: [],
    sessions: [],
    changedFiles: [],
    processes: [],
    selectedRepoRoot: null,
    selectedWorktreePath: null,
    activeSessionId: null,
    loading: true,
    error: null,
  };
}

type Listener = () => void;

const listeners = new Set<Listener>();

export function subscribe(listener: Listener): () => void {
  listeners.add(listener);
  return () => { listeners.delete(listener); };
}

export function notify(): void {
  for (const listener of listeners) {
    listener();
  }
}

export let state = createInitialState();

export function updateState(partial: Partial<AppState>): void {
  Object.assign(state, partial);
  notify();
}

let refreshInFlight = false;

export async function refresh(): Promise<void> {
  if (refreshInFlight) return;
  refreshInFlight = true;
  updateState({ loading: true, error: null });

  try {
    const [repositories, worktrees, sessions, processes] = await Promise.all([
      fetchRepositories(),
      fetchWorktrees(),
      fetchTerminals(),
      fetchProcesses().catch(() => [] as ProcessInfo[]),
    ]);

    // Validate selections still exist, auto-select on first load
    let selectedRepoRoot =
      state.selectedRepoRoot !== null &&
      repositories.some((r) => r.root === state.selectedRepoRoot)
        ? state.selectedRepoRoot
        : null;

    // Auto-select first repo on initial load
    if (selectedRepoRoot === null && repositories.length > 0) {
      selectedRepoRoot = repositories[0].root;
    }

    let selectedWorktreePath =
      state.selectedWorktreePath !== null &&
      worktrees.some((w) => w.path === state.selectedWorktreePath)
        ? state.selectedWorktreePath
        : null;

    // Auto-select primary worktree (or first) for the selected repo on initial load
    if (selectedWorktreePath === null && selectedRepoRoot !== null) {
      const repoWorktrees = worktrees.filter((w) => w.repo_root === selectedRepoRoot);
      const primary = repoWorktrees.find((w) => w.is_primary_checkout);
      const first = primary ?? repoWorktrees[0];
      if (first !== undefined) {
        selectedWorktreePath = first.path;
      }
    }

    let activeSessionId =
      state.activeSessionId !== null &&
      sessions.some((s) => s.session_id === state.activeSessionId)
        ? state.activeSessionId
        : null;

    // Auto-select first running terminal for the selected worktree
    const visibleSessions = selectedWorktreePath !== null
      ? sessions.filter((s) => s.workspace_id === selectedWorktreePath || s.cwd === selectedWorktreePath)
      : sessions;

    // Clear active session if it doesn't belong to the selected worktree
    if (activeSessionId !== null && selectedWorktreePath !== null) {
      const belongs = visibleSessions.some((s) => s.session_id === activeSessionId);
      if (!belongs) {
        activeSessionId = null;
      }
    }

    if (activeSessionId === null && visibleSessions.length > 0) {
      const running = visibleSessions.find((s) => s.state === "running");
      const first = running ?? visibleSessions[0];
      if (first !== undefined) {
        activeSessionId = first.session_id;
      }
    }

    updateState({
      repositories,
      worktrees,
      sessions,
      processes,
      selectedRepoRoot,
      selectedWorktreePath,
      activeSessionId,
      loading: false,
    });

    // Fetch changed files for selected worktree
    if (selectedWorktreePath !== null) {
      refreshChangedFiles(selectedWorktreePath);
    } else {
      updateState({ changedFiles: [] });
    }
  } catch (error) {
    updateState({
      loading: false,
      error: error instanceof Error ? error.message : "unknown request failure",
    });
  } finally {
    refreshInFlight = false;
  }
}

export function refreshChangedFiles(worktreePath: string): void {
  fetchChangedFiles(worktreePath)
    .then((changedFiles) => {
      if (state.selectedWorktreePath === worktreePath) {
        updateState({ changedFiles });
      }
    })
    .catch(() => {
      // Silently ignore change detection failures
    });
}

export function selectWorktree(path: string | null): void {
  const newPath = state.selectedWorktreePath === path ? null : path;

  // Auto-select a terminal for this worktree
  let activeSessionId = state.activeSessionId;
  if (newPath !== null) {
    const wtSessions = state.sessions.filter(
      (s) => s.workspace_id === newPath || s.cwd === newPath,
    );
    const running = wtSessions.find((s) => s.state === "running");
    const first = running ?? wtSessions[0];
    activeSessionId = first?.session_id ?? null;
  }

  updateState({ selectedWorktreePath: newPath, changedFiles: [], activeSessionId });
  if (newPath !== null) {
    refreshChangedFiles(newPath);
  }
}

export function setActiveSession(sessionId: string | null): void {
  updateState({ activeSessionId: sessionId });
}

export function filteredSessions(): TerminalSession[] {
  if (state.selectedWorktreePath === null) {
    return state.sessions;
  }
  return state.sessions.filter(
    (s) => s.workspace_id === state.selectedWorktreePath || s.cwd === state.selectedWorktreePath,
  );
}
