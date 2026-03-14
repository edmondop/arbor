# Workspace Model

Arbor is organized around repository groups, worktrees, and daemon-backed runtime state.

## Repository Groups

A repository group is the primary container shown in the left pane. Arbor can manage multiple repositories at once, each with:

- a root path
- zero or more linked worktrees
- optional GitHub metadata
- provider-backed issue discovery for GitHub or GitLab remotes
- repo-local presets and automation config from `arbor.toml`

## Worktree-Centered UI

Most of the UI updates around the currently selected worktree:

- terminal tabs belong to the selected worktree
- managed processes and scheduled tasks resolve from the selected worktree's repo config
- changed files and diffs are scoped to the selected worktree
- PR status, notes, agent state, and notifications are derived from that worktree
- the native window title can include the selected branch and daemon host context

## Side Panes and Shared State

Across the native and web UI surfaces, Arbor keeps the same high-level model:

- repositories and worktrees on the left
- terminal sessions in the center
- changes, issues, processes, and PR context on the right

The details differ slightly by surface, but the daemon state is shared. The web UI and companion binaries operate on the same worktrees, terminals, processes, and agent sessions as the desktop app.

## Navigation Patterns

Arbor supports:

- direct selection from the sidebar
- back / forward history between worktrees
- keyboard-driven action switching through the command palette
- issue-driven worktree creation from the issue panel and command palette

## Persistence

Arbor persists window and UI state such as pane sizes, selection, notes, and visibility.
The daemon separately persists terminal session metadata, managed-process runtime linkage, and long-lived agent activity state for reconnect and restore flows.
