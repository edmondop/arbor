# Automation and Repo Config

## `arbor.toml`

Repo-local automation is configured with `<repo>/arbor.toml`.

Arbor currently reads the following areas:

- `[[presets]]` for repo-specific commands
- `[[processes]]` for managed background processes
- `[scripts]` for worktree setup and teardown hooks
- `[branch]` for worktree branch-name prefixes
- `[agent]` for repo-local agent defaults and auto-checkpointing
- `[notifications]` for desktop/webhook event routing
- `[[tasks]]` for scheduled commands that run under the daemon

## Presets, Processes, Scripts, Branches, Agents, and Notifications

Example:

```toml
[[presets]]
name = "Review"
icon = "R"
command = "codex --prompt-file .arbor/tasks/review.md"

[[processes]]
name = "web"
command = "npm run dev"
working_dir = "app"
auto_start = true
auto_restart = true
restart_delay_ms = 2000

[scripts]
setup = ["cp .env.example .env"]
teardown = ["rm -f .env"]

[branch]
prefix_mode = "custom"
prefix = "team"

[agent]
default_preset = "codex"
auto_checkpoint = true

[notifications]
desktop = true
events = ["agent_started", "agent_finished", "agent_error"]
webhook_urls = ["https://example.com/hook"]
```

### Repo Presets

Repo presets appear in the UI and command palette. They let a repository define named commands without editing global Arbor config.

### Managed Processes

Processes configured in `arbor.toml` can be started, stopped, restarted, and observed through daemon APIs and process status streams.
Arbor also discovers sibling `Procfile` processes and shows both sources together.

### Worktree Scripts

`[scripts].setup` runs after worktree creation.
`[scripts].teardown` runs before worktree deletion.
If setup fails, Arbor rolls the worktree back instead of leaving partial state behind.

### Branch Prefixes

`[branch]` controls how Arbor derives branch names for managed worktrees.
Supported `prefix_mode` values are:

- `none`
- `git-author`
- `github-user`
- `custom`

Use `prefix = "team-name"` when `prefix_mode = "custom"`.

### Agent Defaults

`[agent].default_preset` selects the repo's preferred agent preset.
`[agent].auto_checkpoint` enables automatic checkpoint commits after supported agent runs complete on local worktrees.

## Task Templates

Command-palette task templates live under `<repo>/.arbor/tasks/*.md` by default.
Arbor loads Markdown files from that directory and lets frontmatter provide metadata such as name, description, and preferred agent.

You can override the template directory with:

```toml
[tasks]
directory = "prompts"
```

Current limitation: TOML does not let `[tasks]` and `[[tasks]]` coexist in the same file, so a custom task-template directory and scheduled tasks cannot share one `arbor.toml` yet.

## Scheduled Tasks

Arbor's daemon can also run scheduled commands defined as `[[tasks]]`.

Example:

```toml
[[tasks]]
name = "triage-prs"
schedule = "0 */30 * * * * *"
command = "./scripts/triage-prs"
working_dir = "."
enabled = true

[tasks.trigger]
on_exit_code = 0
on_stdout = true
agent = "codex"
prompt_template = "Review this output and prepare follow-up work:\n\n{stdout}"
```

Notes:

- `schedule` uses the cron syntax accepted by the daemon's `cron` crate, including a seconds field
- `working_dir` is relative to the repo root when provided
- supported trigger agents are currently `claude` and `codex`
- task history is exposed through the daemon, CLI, MCP, and task WebSocket stream

## Command Palette

The command palette can search and execute:

- built-in actions
- repositories
- worktrees
- issues
- agent presets
- repo presets
- task templates

Ranking also prefers:

- recent palette selections
- the active repository and worktree
- the currently selected agent preset
