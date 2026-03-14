# Remote Access, Daemon, and MCP

## `arbor-httpd`

The daemon provides:

- terminal session persistence
- the bundled web dashboard
- remote GUI access
- issue, process, task, log, and agent activity endpoints
- webhook notification delivery
- the API surface used by `arbor-cli` and the MCP server

## Remote Access

Remote daemon access can be authenticated with a bearer token from:

```toml
[daemon]
auth_token = "replace-me"
```

The GUI can connect to remote daemons and use them for terminal and worktree operations.
The same daemon can also serve the bundled web UI, answer `arbor-cli`, and back `arbor-mcp`.

Useful entry points:

- `just run-httpd`
- `Arbor --daemon`
- `ARBOR_HTTPD_BIND=0.0.0.0:8787 cargo +nightly-2025-11-30 run -p arbor-httpd`

Loopback callers are allowed without a token.
Non-loopback callers must send `Authorization: Bearer <token>`.

## Remote Outposts

Arbor also supports remote outposts over SSH and daemon-backed access, with:

- host management
- remote worktree creation
- remote terminal sessions
- availability tracking

## Web UI and CLI

`arbor-httpd` serves the web dashboard from `/` and the HTTP API from `/api/v1`.
`arbor-cli` can talk to the same daemon for:

- health and repository listing
- worktree create / delete / commit / push
- terminal session control
- managed process control
- scheduled task listing and manual execution

## MCP

`arbor-mcp` exposes Arbor over stdio for MCP clients. It depends on a reachable daemon and supports:

- tools for repositories, worktrees, terminals, processes, tasks, and agent activity
- daemon-backed resources
- prompts for Arbor workflows

Use:

```bash
just run-mcp
```

or:

```bash
ARBOR_DAEMON_URL=http://127.0.0.1:8787 cargo run -p arbor-mcp
```
