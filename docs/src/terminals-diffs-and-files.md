# Terminals, Diffs, and Files

## Terminal Sessions

Arbor supports:

- embedded terminal sessions
- daemon-backed local sessions that survive GUI restarts
- optional Ghostty VT acceleration behind a feature flag
- multiple tabs per worktree
- signal handling for interrupt / terminate / kill
- bell-aware activity tracking and completion notifications
- attach / detach flows that let long-running commands survive UI restarts

## Managed Processes

Arbor can supervise long-running repo commands from both `Procfile` and `arbor.toml`.

Process support includes:

- start, stop, and restart actions
- process status (`running`, `restarting`, `crashed`, `stopped`)
- restart counts and resident memory metrics
- linkage between a managed process and its daemon-backed terminal session
- visibility in both the native UI and the web UI

## Diff and Change Inspection

For each worktree, Arbor can show:

- changed file list
- additions and deletions per file
- side-by-side diff view
- multiple diff tabs
- PR summary and detail cards in the changes pane
- native PR review comments, inline comment actions, and refresh controls

## File Tree

The right pane can switch between:

- changed files
- repository file tree
- issue lists and process state, depending on the surface and active pane

The file tree supports:

- directory expand / collapse
- keyboard-friendly browsing of selected entries
- opening file-view tabs
- worktree-local notes stored under `.arbor/notes.md`

## Command Palette Interaction

The command palette supports long lists and mixed result types:

- selection stays visible while moving with the keyboard
- `Escape` dismisses reliably
- the list shows overflow indication and result count
- commands have left-side icons for scanning
- issues, presets, and task templates can launch directly from the palette
