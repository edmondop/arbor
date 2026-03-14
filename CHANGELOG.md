# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Arbor release tags follow `YYYYMMDD.NN`.
## [Unreleased]
### Added
- Add white VSCode inspired themes
- Add branch info to Arbor title
- Add bell alerts and bd doc fix
- Add Procfile-backed process management
- Add arbor.toml processes to native UI


### Changed
- Polish native issue flows and restore UI state
- Address remaining review comments
- Include primary checkout in issue linking
- Move create modal previews off render path
- Separate PR cache entries by worktree mode
- Improve embedded terminal waiting and defaults
- Emit targeted agent clear events
- Keep agent activity working until all sessions wait
- Align web agent activity merge with desktop
- Preserve legacy agent activity websocket compatibility
- Atomically gate terminal bell activity on exit
- Serialize terminal activity websocket events
- Parse top-level GitHub rate-limit errors
- Preserve cached PR metadata during cooldown
- Refine Procfile tab UI
- Align Procfile process session titles


### Fixed
- Fix native sidebar issue list and ordering
- Fix workspace hooks on Windows
- Fix remaining issue workflow review comments
- [release] Avoid case-insensitive binary name collision on macOS
- [ci] Use POSIX awk to parse [[bin]] stanzas robustly
- Set MACOSX_DEPLOYMENT_TARGET to support older macOS versions
- Use portable version comparison in CI deployment target check
- Fix theme modal scrolling alignment
- Fix agent waiting transition gating
- Fix terminal activity review feedback
- Prune stale agent activity sessions
- Backfill legacy agent session ids in daemon client
- Ignore post-exit terminal bell activity
- Handle GitHub rate limit cooldown in TUI
- Restore merged refresh fixes after rebase
- Clear stale PR rows during cooldown
- Fix process manager review regressions
- Fix process review follow-ups
- Fix scoped worktree process sync
- Fix process restart review issues
## [20260313.01] - 2026-03-13
### Added
- Add issue-driven worktree creation across UIs
- Add richer PR details to the changes pane


### Changed
- Share worktree name sanitization across crates
- Refine issue worktree flows across UIs
- Animate native loading indicators
- Address PR review races
- Adjust worktree refresh order
- Flush UI state and notes before quit
- Clean up orphaned daemon sessions
- Track async config saves and notes edits
- Preserve reverted UI state during async save


### Fixed
- Fix issue provider detection and Windows preview path
- Harden issue provider resolution and issue refreshes
- Fix create modal height in native UI
- Fix arbor-gui UI-thread blocking paths
- Fix persistence save ordering
- Stop idle worktree auto-refresh rerenders
- Preserve terminal closure during auto refresh
- Preserve non-local selection during inventory refresh
- Fix remaining PR persistence races
- Fix GitHub auth save ordering
## [20260312.03] - 2026-03-12
### Fixed
- [ci] Bundle the real arbor-cli binary
## [20260312.02] - 2026-03-12
### Fixed
- [release] Move bundled macOS CLI into Helpers
## [20260312.01] - 2026-03-12
### Added
- Add Arbor docs and polish GUI behavior
- Lower macOS minimum to Big Sur, add Windows CI and fix Windows bugs
- Add CodSpeed benchmarks for arbor-core
- [gui] Add manual path input for Linux without xdg-desktop-portal
- [terminal] Add experimental ghostty vt backend
- Complete Arbor tier 2 workflow features
- [gui] Add PR review comments in diff view
- [gui] Add PR Changes diff view mode in Changes pane
- [gui] Add inline comment buttons on PR diff lines
- [gui] Add refresh button for PR review comments
- [gui] Add graphql-client for typed GitHub GraphQL queries
- Add Symphony service orchestration
- [httpd] Add scheduled task system with conditional agent triggering
- [cli] Add arbor-cli crate with full daemon API coverage


### Changed
- Refactor arbor-gui module boundaries
- [terminal] Speed up snapshots and fix ghostty cursor
- Deepen tier 1 palette tasks and notifications
- Deepen tier 1 prompt runner and notifications
- Bd init: initialize beads issue tracking
- Improve worktree intelligence and add planning docs
- Add UI verification section to AGENTS.md and CLAUDE.md
- Add PR Review Comments section to CLAUDE.md and AGENTS.md
- Apply missing rustfmt change
- Address PR review feedback


### Fixed
- Fix terminal tab SVG icons
- Skip Unix-only terminal tests on Windows CI
- Fix notes wrapping and port badge behavior
- [gui] Move PR mode toggle to its own row for visibility
- [gui] Optimistically inject posted comment into diff view
- [gui] Show PR mode toggle when pr_details is unavailable
- [gui] Use SVG comment icon and improve comment background contrast
- Fix
- Fix Windows CI regressions
- Fix Windows clippy import warning
- Fix Windows clippy shell quoting lint
- Fix Windows CI for Symphony tests
- Fix Symphony review findings
- Fix Ghostty CodSpeed benchmark runtime
- Fix Ghostty CodSpeed run invocation
- Use server-side window decorations on Linux
## [20260311.03] - 2026-03-11
### Fixed
- [httpd] Fix memory leak from unfiltered broadcast log layer and dead sessions
## [20260311.02] - 2026-03-11
### Added
- Add build-release recipe to justfile for local release builds
- Add drag & drop reordering for sidebar items


### Changed
- Add git workflow conventions to CLAUDE.md
## [20260311.01] - 2026-03-11
### Added
- Implement Arbor tier 1 workflow features


### Changed
- Improve command palette navigation UI
- Use SVG assets for native toolbar and tab icons
- Refactor codebase: split large files, add feature flags, newtypes, and workspace dep hygiene


### Fixed
- Harden daemon websocket log streaming
- Fix linked worktree grouping and top bar buttons
- Fix pending GitHub check status parsing
## [20260310.04] - 2026-03-10
### Fixed
- Fix worktree hover corners, dock icon in dev, and host selector UX
## [20260310.03] - 2026-03-10
### Added
- Add agent activity status dots to web UI worktree cards


### Changed
- Replace fixed 45ms terminal polling sleep with event-driven channel notification
- Run zizmor workflow security check on PRs too
## [20260310.02] - 2026-03-10
### Fixed
- Fix release workflow clippy job missing Linux dependencies
## [20260310.01] - 2026-03-10
### Added
- Add changelog workflow and fix gpui example build
- Implement zed-style arbor shell with alacritty terminal backend
- Persist ui state and improve worktree/change views
- Add remote daemon/web UI and improve diff tab UX
- Add MIT LICENSE file
- Add mosh transport for outpost terminals
- Add Manage Hosts modal for adding/removing remote hosts from GUI
- Implement SSH agent forwarding via libssh agent channel proxying
- Add file-type icons to file tree, fix worktree repo resolution, polish changes pane
- Add top bar controls, collapsible repos, GitHub icons, and loading spinner
- Add Cmd-N New Window support with File menu item
- Add hover-visible close button to terminal and diff tabs
- Add HTTP client for avatars, in-app logs, Cmd-Q overlay, and UI polish
- Add remove button to outpost rows in sidebar
- Add changes-pane git actions and split quit behavior
- Add desktop notifications for terminal completion
- Add hook-based agent activity detection with WS streaming
- Add last activity timestamp to worktree listing
- Add delete worktree/outpost with confirmation modal
- Add Homebrew cask distribution for macOS
- Add Report Issue button to title bar
- Add GitHub issue templates for bug reports, crashes, and feature requests
- [gui] Add terminal presets bar with official icons
- Add similar tools and acknowledgements to README
- Add sound for task completion notifications
- Add CLAUDE.md with project conventions and pre-push checks
- [gui] Add Action menu launchers and toast notices
- Add zlib1g-dev:arm64 for linux arm64 cross-compile linking
- Add file viewer tabs in center pane
- Add syntax highlighting to file viewer using syntect
- Add search filter to right pane file listing
- Add image preview for image files in file viewer
- Add editable file viewer with save support and fix search focus
- Add click-to-position cursor, $EDITOR support, and fix Cmd+T focus
- Add ellipsis to truncated tab titles
- Add right padding to tabs so close button doesn't overlap title
- Add overflow_hidden to tab container so text_ellipsis works
- Add tests for truncate_with_ellipsis tab label truncation
- Add FileView arms to all CenterTab match statements
- Add macOS code signing and notarization to release workflow
- Add macOS app packaging and codesigning to CI
- Add zizmor workflow security scanning and fix codesign identity lookup
- Add arbor.toml with "just run" preset
- Add per-repository custom presets via arbor.toml
- Add right-click context menu to remove repositories from sidebar
- Add About Arbor menu item showing app version
- Add GitHub OAuth device auth UI and token persistence
- [gui] Show release version in macOS About dialog
- [gui] Show IME marked text preview at terminal cursor
- [gui] Replace hold-to-quit with a confirmation modal on Cmd-Q
- [gui] Add Dracula theme
- Add Omarchy themes and show active theme in UI
- [gui] Add theme picker modal and move theme selection to View menu
- [gui] Add Copilot preset and hide presets for uninstalled CLIs
- [web-ui] Rewrite as modular three-pane layout with xterm.js
- [httpd,web-ui] Use octocrab for PR lookups, enrich sidebar with GitHub data
- [mcp] Add arbor-mcp stdio MCP server for AI agent integration
- [httpd] Add PR lookup cache with 120s TTL
- [web-ui,gui] Mobile layout, preset buttons, worktree-scoped terminals, caching
- [gui,httpd] LAN daemon discovery, auth prompt, and remote host connection
- [web-ui,httpd] Add preset icons, worktree-scoped tabs, auth & TLS
- [gui] Add slug() method to ThemeKind for config persistence
- [gui] Add daemon CLI mode and ssh host tunneling
- Add static Arbor website and refresh product messaging
- Add start daemon modal when Remote Control is clicked while disconnected
- Add grouped discrete clone support
- Add Pi agent integration
- Productize Arbor MCP server
- [net] IPv6 dual-stack binding and improved connection logging
- [gui] Open new window when clicking LAN daemon in sidebar
- [gui] Inline remote worktrees in sidebar without switching daemon
- [gui] Add hover styling to all interactive buttons and elements
- [gui] Bundle IBM Plex Sans and Lilex UI fonts
- [config] Add embedded_shell option for embedded terminal
- Add just entry


### Changed
- Bootstrap arbor workspace with gpui app and core crate
- Scope terminals per worktree and focus terminal on selection
- Add cross-platform build matrix and tag releases
- Improve diff view UX and zonemap scrolling
- Add clickable README hero screenshot
- Refine diff wrapping using live viewport and diff metrics
- Route Cmd-T to outpost terminal when an outpost is selected
- Enable SSH agent forwarding for outpost commands and shells
- Use ssh -A for git clone to forward agent to remote hosts
- Use ssh -F /dev/null in GIT_SSH_COMMAND to skip remote SSH config
- Ensure SSH agent has keys before forwarding for git clone
- Wire up SSH shell terminals for outpost connections
- Unify create worktree/outpost into tabbed Add modal; load remote diffs via SSH
- Deduplicate shared utilities into arbor-core
- Simplify MoveActiveField toggle, deduplicate default_shell, inline wrapper
- Move Diff button to bottom of changes pane instead of tab bar
- Open diff on double-click in changes file list
- Remove Diff button, single click opens diff (Zed-style)
- Always scroll diff to clicked file, even if already visible
- Move sidebar toggle to left, minimized collapsed pane, bigger nav icons
- Commit all remaining workspace changes
- Extract repo icon and activity dots outside cell borders
- Polish sidebar sizing and update README with features list
- Show agent session prompts instead of directory names in sidebar
- Cancel in-progress CI runs on same branch and fix clippy warnings
- Use vendored-openssl for all CI/release build targets
- Open TUI editors in terminal tab, GUI editors as subprocess
- Set terminal tab title to editor name when opening files
- Truncate long tab titles with overflow hidden
- Truncate all tab labels to 16 chars with ellipsis
- Reduce tab label max chars to 12 so ellipsis fits visually
- Use character-based truncation for tab labels instead of CSS ellipsis
- Make tabs scrollable and dynamically sized with proper ellipsis
- Auto-scroll tab bar to active tab when new tabs are added
- Remove Presets tab from right pane and fix search input
- Pin actions to SHA hashes and fix template injection in workflows
- Replace all git/gh/pgrep/lsof subprocesses with pure Rust crates
- Send backtab escape sequence for Shift+Tab in terminal
- Replace worktree X button with right-click context menu
- Replace placeholder black icon with arbor tree/git-graph icon
- Update main.rs
- Use per-command PATH instead of mutating global environment
- Auto-start arbor-httpd daemon from GUI and persist sessions on quit
- Support random daemon port for multiple dev instances
- Open repo GitHub URL from sidebar icon clicks
- Add build-from-source prerequisites and setup recipes
- Apply rustfmt formatting
- Improve outpost UI, fix ghost terminals, and fix outpost provisioning
- Point to correct font on Linux
- Update website social meta tags
- Update artifact upload path in GitHub Actions workflow
- Apply UI polish and terminal behavior updates
- Refine native UI controls and modal UX
- Unify repository persistence behind store trait
- Add fullscreen screenshot lightbox navigation
- Refactor injected terminal and GitHub services
- Slim down README and add website documentation pages
- Detect daemon version mismatch on startup and auto-restart
- Remove process-based agent detection, add hook lifecycle management and daemon auth
- Prioritize active daemon terminal sync
- Refine hover interactions and scrollbar styling
- Tone down merged PR worktree rows
- Refine preset editing UX
- Simplify daemon settings modal
- Refine modal backdrop and worktree sizing
- Bundle Nerd Font, fix remote HTTP auth and bind config
- Hot-reload daemon bind mode and harden auth middleware
- Replace mdns-sd with zeroconf for native Bonjour/Avahi integration
- Wrap auth token in SecretString, fix LAN discovery UI
- Skip auto-start for remote daemons and add connection logging
- [gui] Add tracing to LAN daemon window opening flow
- Add Linux options to issue templates
- Update crates/arbor-gui/src/main.rs


### Fixed
- Fix terminal rendering parity with Zed profile
- Fix Linux CI/release deps and ARM64 cross C++ toolchain
- Fix linux-aarch64 CI build by adding Ubuntu ports mirror
- Fix linux-aarch64 release build with Ubuntu ports mirror
- Fix arm64 apt source pinning for Ubuntu 24.04 deb822 format
- Fix agent forwarding: override remote SSH config for git clone
- Fix WebSocket ping bug, remove dead code, deduplicate methods
- Fix release workflow: add checkout step to publish-release job
- Fix worktrees appearing as separate repos in sidebar
- Fix clippy and rustfmt warnings
- Fix rustfmt formatting
- Fix clippy collapsible_if warnings and formatting
- Fix OpenSSL cross-compilation in CI with vendored-openssl feature
- Clear search focus when clicking center pane
- Fix editor terminal tab title not displaying
- Fix ellipsis truncation to fit within character budget
- Fix text_ellipsis on tab labels using correct overflow_hidden pattern
- Fix clippy collapsible_if warnings and apply formatting
- Fix Windows CI: vendor zlib for libssh-rs-sys
- Fix Windows CI: gate Unix-only SSH agent forwarding with cfg(unix)
- Fix app startup without git repo and dock icon mismatch
- Fix release workflow: secrets context not allowed in step if conditions
- Fix macOS aarch64 release build: add vendored-openssl feature
- Fix formatting in augment_path_from_login_shell
- Fix delete modal checkbox icon and button hover contrast
- Fix worktree integration test failing on CI due to default branch name
- [release] Build and sign arbor-httpd in release CI
- Fix black dock icon by explicitly setting NSApplicationIconImage
- Avoid .ZedMono probe on macOS
- [gui] Resolve CI failures from rebase conflicts in About dialog
- [gui] Route regular key input through macOS IME for dead key composition
- [ci] Vendor OpenSSL for arbor-httpd release builds
- [gui] Process daemon terminal output through VTE emulator
- [httpd] Preserve ANSI boundaries when trimming terminal output tail
- [gui] Use viewport-matched PTY size to prevent reflow artifacts on tmux detach
- [gui] Make quit modal buttons same size and fix Quit click not working
- [ssh] Handle unsupported key types in SSH agent gracefully
- [web-ui] Send PTY resize after WebSocket connects
- [web-ui] Group worktrees under repos in sidebar like native app
- Fix welcome clone input and persist empty repo state
- Fix rustfmt formatting in github_service and main
- Fix terminal transport and native terminal state
- Harden daemon websocket auth and sync
- [web] Render ANSI from emulator on reconnect to prevent mangled columns
- [gui] Resolve clippy single_match and collapsible_if warnings
- Resolve clippy warnings across daemon-client, gui, and mcp crates
- [web] Match native UI worktree listing order and remove PRIMARY badge
- [gui] Propagate vendored-openssl feature to arbor-core
- [ci] Add vendored-openssl to arbor-mcp for macOS cross-compile
- [auth] Recognize IPv4-mapped loopback (::ffff:127.0.0.1) as localhost
- [gui] Downgrade per-request "connected to daemon" log to debug
- [gui] Show host:port in LAN daemon sidebar entries
- [gui] Skip local daemon/repos when opening remote daemon window
- [gui] Handle Enter/Escape keys in quit confirmation overlay
- [ci] Install libavahi-client-dev for zeroconf mDNS support
- [gui,httpd,ssh] Outpost creation fixes, quit modal, context menus, log streaming
- [ci] Fetch lfs assets and apply rustfmt
- [gui] Use platform-conditional left offset for top bar
- [terminal] Use Ctrl+Shift+C/V for copy/paste on Linux
- [gui] Use consistent icon slot size for agent preset tabs
