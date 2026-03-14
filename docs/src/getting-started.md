# Getting Started

## Install and Run

Homebrew on macOS:

```bash
brew install penso/arbor/arbor
```

From source:

```bash
git clone https://github.com/penso/arbor
cd arbor
just run
```

Useful local commands:

- `just run` starts `arbor-httpd` and the native GUI together
- `just run-httpd` starts only the daemon
- `just run-mcp` starts the daemon and MCP server together
- `cargo run -p arbor-cli -- health` talks to the daemon from a shell
- `just docs-build` builds this documentation book into `docs/book`
- `just changelog-unreleased` previews release notes from the current git history

## Main Binaries

- `Arbor`: native GPUI desktop app
- `arbor-httpd`: daemon, HTTP API, and bundled web dashboard
- `arbor-cli`: daemon-backed CLI for automation and scripts
- `arbor-mcp`: stdio MCP server backed by the daemon

## Main Concepts

- Repository: a tracked git root
- Worktree: one checkout belonging to a repository
- Managed worktree: a worktree whose path and branch are derived from an issue or typed name
- Issue source: provider-backed issue discovery for a repository, currently GitHub or GitLab
- Terminal session: an attached shell for a worktree
- Managed process: a `Procfile` or `arbor.toml` command supervised by Arbor
- Scheduled task: a `[[tasks]]` command from `arbor.toml`, optionally with an agent trigger
- Outpost: a remote worktree target over SSH / daemon access
- Daemon: `arbor-httpd`, which backs terminal persistence, remote access, the web UI, CLI, and MCP surface

## Core User Flows

- add a repository, then create or select a worktree
- browse issues for that repository and create a managed worktree from one
- open a terminal tab, review processes, or run scheduled tasks for the selected worktree
- inspect changed files, PR summaries, and diffs
- commit or push from the GUI or via the daemon-backed CLI
- launch agent presets or task templates
- use `Cmd+K` to jump to actions, repos, worktrees, issues, presets, and tasks

## Configuration Locations

- app config: `~/.config/arbor/config.toml`
- repo config: `<repo>/arbor.toml`
- repo-local tasks: `<repo>/.arbor/tasks/*.md`
- daemon session store: `~/.arbor/daemon/sessions.json`
- worktree notes: `<worktree>/.arbor/notes.md`
