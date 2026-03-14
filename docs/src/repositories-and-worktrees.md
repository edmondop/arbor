# Repositories and Worktrees

## Repository Management

Arbor can track multiple repositories and list all linked worktrees under each one.

Repository-level capabilities include:

- add and remove repositories
- collapse or expand repository groups
- identify the primary checkout
- resolve GitHub repo slug and avatar when available
- resolve issue sources for GitHub and GitLab remotes
- load repo-local presets, scripts, processes, tasks, and branch rules from `arbor.toml`

## Worktree Management

Worktree capabilities include:

- create local worktrees from the create modal
- create managed worktrees from an issue or typed worktree name
- preview derived branch names and worktree paths before creation
- delete non-primary worktrees
- optionally delete the branch during worktree removal
- show last git activity and PR metadata
- maintain navigation history across worktrees

## Issue-Driven Worktrees

Arbor can create worktrees directly from provider issues. The issue surface currently:

- discovers open issues from the repository's GitHub or GitLab remote
- suggests a sanitized worktree name from the issue id and title
- detects already-linked branches and existing PRs / MRs
- opens the same managed-worktree flow in both the native and web UI

Managed worktree naming can also honor repo-level branch prefixes:

```toml
[branch]
prefix_mode = "github-user"
```

Supported `prefix_mode` values are:

- `none`
- `git-author`
- `github-user`
- `custom`

For `custom`, also set `prefix = "team-name"`.

## Worktree Lifecycle Hooks

Repo-level lifecycle automation can run during worktree create and delete:

- setup scripts run after a worktree is created
- teardown scripts run before a worktree is deleted
- if setup fails, Arbor rolls back the created worktree

The repo config lives in `<repo>/arbor.toml`.

Example:

```toml
[scripts]
setup = ["cp .env.example .env"]
teardown = ["rm -f .env"]
```
