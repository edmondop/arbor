# Arbor HTTP API (v1)

Base URL defaults to:

- `http://127.0.0.1:8787` when remote auth is disabled
- `http://0.0.0.0:8787` when `[daemon] auth_token` is configured

You can override the bind address with `ARBOR_HTTPD_BIND`.

## Auth

Remote auth is enforced by the daemon, not by the MCP layer.

- Loopback callers are allowed without authentication
- Non-loopback callers require `[daemon] auth_token` in `~/.config/arbor/config.toml`
- Remote requests must send `Authorization: Bearer <token>`
- If `[daemon] auth_token` is configured, the default bind address becomes `0.0.0.0:8787`

Example:

```http
GET /api/v1/health HTTP/1.1
Host: remote-host:8787
Authorization: Bearer replace-me
```

## Endpoints

### `GET /api/v1/health`

Returns daemon health and version.

### `GET /api/v1/repositories`

Returns known repository roots from `~/.arbor/repositories.json`.

### `GET /api/v1/worktrees`

Returns worktrees across known repositories.

Query params:

- `repo_root` (optional): filter to one repository root path.

### `POST /api/v1/worktrees`

Creates a worktree.

Request body:

```json
{
  "repo_root": "/Users/penso/code/arbor",
  "path": "/Users/penso/.arbor/worktrees/arbor/feature-docs",
  "branch": "feature-docs",
  "detach": false,
  "force": false
}
```

### `POST /api/v1/worktrees/delete`

Deletes a non-primary worktree.

Request body:

```json
{
  "repo_root": "/Users/penso/code/arbor",
  "path": "/Users/penso/.arbor/worktrees/arbor/feature-docs",
  "delete_branch": true,
  "force": false
}
```

### `GET /api/v1/worktrees/changes`

Returns changed files for one worktree.

Query params:

- `path` (required): absolute worktree path.

### `POST /api/v1/worktrees/commit`

Creates a commit in a worktree.

Request body:

```json
{
  "path": "/Users/penso/.arbor/worktrees/arbor/feature-docs",
  "message": "docs: refine MCP guide"
}
```

`message` is optional. When omitted, Arbor generates a commit message from the changed files.

### `POST /api/v1/worktrees/push`

Pushes the current branch of a worktree to `origin`.

Request body:

```json
{
  "path": "/Users/penso/.arbor/worktrees/arbor/feature-docs"
}
```

### `GET /api/v1/terminals`

Returns merged terminal session records from the daemon runtime and `~/.arbor/daemon/sessions.json`.

### `POST /api/v1/terminals`

Creates or attaches a terminal session.

Request body:

```json
{
  "session_id": "daemon-1",
  "workspace_id": "/Users/penso/code/arbor",
  "cwd": "/Users/penso/code/arbor",
  "shell": "/bin/zsh",
  "cols": 120,
  "rows": 35,
  "title": "term-arbor"
}
```

`session_id` is optional, the daemon will generate one when omitted.

### `GET /api/v1/terminals/:session_id/snapshot`

Returns output tail and terminal state for one session.

Query params:

- `max_lines` (optional, default `180`, max `2000`)

### `POST /api/v1/terminals/:session_id/write`

Writes raw terminal input bytes to a session.

Request body:

- Raw bytes with `Content-Type: application/octet-stream`

Example:

```text
ls -la
```

### `POST /api/v1/terminals/:session_id/resize`

Resizes a terminal grid.

Request body:

```json
{
  "cols": 120,
  "rows": 35
}
```

### `POST /api/v1/terminals/:session_id/signal`

Sends a signal to a terminal session.

Request body:

```json
{
  "signal": "interrupt"
}
```

Allowed values: `interrupt`, `terminate`, `kill`.

### `POST /api/v1/terminals/:session_id/detach`

Detaches the current client from a daemon-managed terminal session without killing it.

### `DELETE /api/v1/terminals/:session_id`

Kills and removes a daemon-managed terminal session.

### `GET /api/v1/terminals/:session_id/ws`

WebSocket stream for interactive terminal I/O.

Client messages:

- `{"type":"input","data":"echo hi\n"}`
- `{"type":"resize","cols":140,"rows":40}`
- `{"type":"signal","signal":"interrupt"}`
- `{"type":"detach"}`

Server messages:

- `{"type":"snapshot", ...}`
- `{"type":"output", ...}`
- `{"type":"exit", ...}`
- `{"type":"error", ...}`

### `GET /api/v1/agent/activity`

Returns the current agent activity snapshot.

### `POST /api/v1/agent/notify`

Updates agent activity state from a hook callback.

### `GET /api/v1/agent/activity/ws`

WebSocket stream for real-time agent activity updates.

### `GET /api/v1/processes`

Returns managed processes loaded from `arbor.toml`.

### `POST /api/v1/processes/start-all`

Starts all configured processes.

### `POST /api/v1/processes/stop-all`

Stops all running processes.

### `POST /api/v1/processes/:name/start`

Starts one named process.

### `POST /api/v1/processes/:name/stop`

Stops one named process.

### `POST /api/v1/processes/:name/restart`

Restarts one named process.

### `GET /api/v1/processes/ws`

WebSocket stream for real-time managed process updates.

### `POST /api/v1/shutdown`

Requests daemon shutdown. This is limited to localhost callers.
