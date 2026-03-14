# Arbor Documentation

Arbor is a native Rust + GPUI workspace for agentic coding across local repositories, remote daemons, and companion tools such as `arbor-cli` and `arbor-mcp`.

This book documents the current product surface across both user-facing UIs, `arbor-gui` and `arbor-web-ui`, plus the daemon APIs they share.

## What Arbor Covers

Arbor currently includes:

- multi-repository workspaces and linked worktrees
- issue-driven managed worktree creation and repo-aware branch naming
- embedded terminals, daemon session restore, managed processes, and scheduled tasks
- changed files, file trees, side-by-side diffs, PR summaries, and native PR review comments
- real-time coding-agent activity, notifications, notes, and command palette workflows
- remote daemons, SSH / mosh outposts, the bundled web UI, CLI, and MCP server
- repo-local automation through `arbor.toml` and app-wide settings through `config.toml`

## Read This Book In Order If You Are New

1. [Getting Started](./getting-started.md)
2. [Workspace Model](./workspace-model.md)
3. [Repositories and Worktrees](./repositories-and-worktrees.md)
4. [Terminals, Diffs, and Files](./terminals-diffs-and-files.md)
5. [GitHub, Agents, and Git Actions](./github-agents-and-git.md)
6. [Automation and Repo Config](./automation-and-repo-config.md)
7. [Remote Access, Daemon, and MCP](./remote-daemon-and-mcp.md)
8. [Themes, Settings, and Notifications](./themes-settings-and-notifications.md)

Use [QA Checklist](./qa-checklist.md) for a focused regression sweep of the highest-risk workflows.
