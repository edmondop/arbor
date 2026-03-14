# GitHub, Agents, and Git Actions

## GitHub and GitLab Context

Arbor surfaces GitHub information in the UI, including:

- PR number and link per worktree
- PR summary and detail cards in the native changes pane
- review-comment refresh and inline comment actions in native diff views
- GitHub auth state in the top bar
- issue discovery for GitHub and GitLab repositories
- linked-branch and linked-review detection during managed-worktree creation

GitHub rate-limit handling is also explicit now. Arbor preserves cached PR data during cooldown windows instead of blanking the UI and shows a visible notice while refreshes are deferred.

## Agent Visibility

Arbor tracks coding-agent activity and shows:

- working / waiting state
- per-worktree state indicators
- real-time updates through daemon-backed activity streams
- compatibility with legacy daemon session ids
- targeted clear events so stale sessions do not linger in the UI

Repo config can also steer agent behavior:

```toml
[agent]
default_preset = "codex"
auto_checkpoint = true
```

`default_preset` selects the repo's preferred agent preset.
`auto_checkpoint` lets Arbor create a local checkpoint commit when a supported agent run finishes with changes.

## Notification Routing

Arbor supports notification behavior for agent and process lifecycle events:

- native desktop notifications in the GUI for relevant agent state transitions
- daemon-side webhook POST delivery for `agent_started`
- daemon-side webhook POST delivery for `agent_finished`
- daemon-side webhook POST delivery for `agent_error`
- bounded retry/backoff for transient webhook delivery failures

Repo-level notification config:

```toml
[notifications]
desktop = true
events = ["agent_started", "agent_finished", "agent_error"]
webhook_urls = ["https://example.com/hook"]
```

## Git Actions

Arbor includes in-UI git actions for:

- commit
- push
- PR visibility

The commit flow includes:

- editable commit message in a modal
- fallback "Use Default" message path
- AI-generated commit message path through the shared prompt runner

The same daemon-backed git actions are also available to `arbor-cli` and `arbor-mcp`.
