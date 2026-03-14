# Arbor MCP

Arbor ships a dedicated `arbor-mcp` binary from the `arbor-mcp` crate. It exposes Arbor's daemon-backed state over stdio using the Model Context Protocol.

## Requirements

`arbor-mcp` talks to `arbor-httpd`, so the daemon must be reachable first.

Relevant environment variables:

- `ARBOR_DAEMON_URL`: daemon base URL. Default: `http://127.0.0.1:8787`
- `ARBOR_DAEMON_AUTH_TOKEN`: bearer token for remote authenticated daemons

The stdio server is enabled by the crate's default `stdio-server` feature.

## Enabling the Server

Build the MCP server with default features:

```bash
cargo build -p arbor-mcp
```

Run it directly against a daemon:

```bash
ARBOR_DAEMON_URL=http://127.0.0.1:8787 cargo run -p arbor-mcp
```

## Local Development

Run Arbor's daemon and MCP server together:

```bash
just run-mcp
```

## Client Example

```json
{
  "mcpServers": {
    "arbor": {
      "command": "/path/to/arbor-mcp",
      "env": {
        "ARBOR_DAEMON_URL": "http://127.0.0.1:8787"
      }
    }
  }
}
```

## Remote Access and Auth

`arbor-mcp` does not implement a second auth layer. It forwards requests to `arbor-httpd`, and the daemon enforces remote auth.

Daemon behavior:

- Loopback clients (`127.0.0.1`, `::1`, `localhost`) are allowed without a token
- Non-loopback clients require a configured `[daemon] auth_token`
- When `[daemon] auth_token` is configured, the daemon binds remotely by default on `0.0.0.0:8787`
- `ARBOR_HTTPD_BIND` can override the bind address in either mode

To enable remote MCP access:

1. On the machine running `arbor-httpd`, set an auth token in `~/.config/arbor/config.toml`:

```toml
[daemon]
auth_token = "replace-me"
```

2. Start or restart `arbor-httpd`.
3. In the MCP client environment, point `arbor-mcp` at the remote daemon and provide the same token:

```json
{
  "mcpServers": {
    "arbor": {
      "command": "/path/to/arbor-mcp",
      "env": {
        "ARBOR_DAEMON_URL": "http://remote-host:8787",
        "ARBOR_DAEMON_AUTH_TOKEN": "replace-me"
      }
    }
  }
}
```

`arbor-mcp` sends the token as `Authorization: Bearer <token>`. If the token is missing or wrong, the daemon rejects the request.

## Features

Arbor's MCP server exposes:

- Tools for repositories, worktrees, changed files, git commit/push, terminals, processes, tasks, and agent activity
- Resources for daemon snapshots such as `arbor://health`, `arbor://processes`, and `arbor://tasks`
- Prompts for common Arbor workflows such as reviewing a worktree or stabilizing a process

Today the MCP surface focuses on daemon-backed worktree, terminal, process, and task flows.
Issue browsing and managed-worktree previews still live in the GUI and web surfaces.

## Disabling the Binary

The stdio server binary is cargo-feature gated:

```bash
cargo build -p arbor-mcp --no-default-features
```

To make the feature explicit, build it with:

```bash
cargo build -p arbor-mcp --features stdio-server
```
