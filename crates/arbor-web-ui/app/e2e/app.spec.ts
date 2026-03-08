import { test, expect } from "@playwright/test";

test.describe("Arbor Web UI", () => {
  test.beforeEach(async ({ page }) => {
    // Mock API responses so tests work without a running backend
    await page.route("**/api/v1/repositories", (route) =>
      route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify([
          { root: "/home/user/projects/arbor", label: "arbor" },
          { root: "/home/user/projects/moltis", label: "moltis" },
        ]),
      }),
    );

    await page.route("**/api/v1/worktrees**", (route) =>
      route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify([
          {
            repo_root: "/home/user/projects/arbor",
            path: "/home/user/projects/arbor",
            branch: "main",
            is_primary_checkout: true,
            last_activity_unix_ms: Date.now() - 30_000,
          },
          {
            repo_root: "/home/user/projects/arbor",
            path: "/home/user/projects/arbor-worktrees/feature-auth",
            branch: "feature/auth",
            is_primary_checkout: false,
            last_activity_unix_ms: Date.now() - 120_000,
          },
          {
            repo_root: "/home/user/projects/moltis",
            path: "/home/user/projects/moltis",
            branch: "main",
            is_primary_checkout: true,
            last_activity_unix_ms: null,
          },
        ]),
      }),
    );

    await page.route("**/api/v1/terminals", (route) => {
      if (route.request().method() === "GET") {
        return route.fulfill({
          status: 200,
          contentType: "application/json",
          body: JSON.stringify([
            {
              session_id: "daemon-1",
              workspace_id: "/home/user/projects/arbor",
              cwd: "/home/user/projects/arbor",
              shell: "/bin/zsh",
              cols: 120,
              rows: 35,
              title: "arbor",
              last_command: "just test",
              output_tail: "All tests passed!",
              exit_code: null,
              state: "running",
              updated_at_unix_ms: Date.now() - 5_000,
            },
            {
              session_id: "daemon-2",
              workspace_id: "/home/user/projects/arbor-worktrees/feature-auth",
              cwd: "/home/user/projects/arbor-worktrees/feature-auth",
              shell: "/bin/zsh",
              cols: 120,
              rows: 35,
              title: "feature-auth",
              last_command: "cargo build",
              output_tail: null,
              exit_code: 0,
              state: "completed",
              updated_at_unix_ms: Date.now() - 60_000,
            },
          ]),
        });
      }
      return route.fulfill({ status: 200, contentType: "application/json", body: "{}" });
    });

    await page.route("**/api/v1/worktrees/changes**", (route) =>
      route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify([
          { path: "src/main.rs", kind: "modified", additions: 15, deletions: 3 },
          { path: "src/api.rs", kind: "added", additions: 42, deletions: 0 },
          { path: "tests/old_test.rs", kind: "removed", additions: 0, deletions: 28 },
          { path: "README.md", kind: "modified", additions: 2, deletions: 1 },
        ]),
      }),
    );

    await page.goto("/");
  });

  test("renders three-pane layout", async ({ page }) => {
    const sidebar = page.getByTestId("sidebar");
    const terminalPanel = page.getByTestId("terminal-panel");
    const changesPanel = page.getByTestId("changes-panel");
    const statusBar = page.getByTestId("status-bar");

    await expect(sidebar).toBeVisible();
    await expect(terminalPanel).toBeVisible();
    await expect(changesPanel).toBeVisible();
    await expect(statusBar).toBeVisible();

    await page.screenshot({ path: "e2e/screenshots/layout.png", fullPage: true });
  });

  test("sidebar shows repositories", async ({ page }) => {
    const sidebar = page.getByTestId("sidebar");
    await expect(sidebar.locator(".sidebar-item-name").getByText("arbor", { exact: true })).toBeVisible();
    await expect(sidebar.locator(".sidebar-item-name").getByText("moltis", { exact: true })).toBeVisible();
  });

  test("sidebar shows worktrees", async ({ page }) => {
    const sidebar = page.getByTestId("sidebar");
    await expect(sidebar.locator(".sidebar-item-meta").getByText(/^main/).first()).toBeVisible();
    await expect(sidebar.locator(".sidebar-item-meta").getByText(/feature\/auth/)).toBeVisible();
  });

  test("clicking repo filters worktrees", async ({ page }) => {
    const sidebar = page.getByTestId("sidebar");

    // Click the moltis repo
    await sidebar.getByRole("button", { name: /moltis/ }).first().click();

    // Should only show moltis worktrees
    await expect(sidebar.getByText("feature/auth")).not.toBeVisible();

    await page.screenshot({
      path: "e2e/screenshots/repo-filter.png",
      fullPage: true,
    });
  });

  test("terminal panel shows session tabs", async ({ page }) => {
    const terminalPanel = page.getByTestId("terminal-panel");
    await expect(terminalPanel.getByText("arbor")).toBeVisible();
    await expect(terminalPanel.getByText("feature-auth")).toBeVisible();
  });

  test("changes panel shows files when worktree selected", async ({ page }) => {
    const sidebar = page.getByTestId("sidebar");

    // Select the arbor repo first
    await sidebar.getByRole("button", { name: /arbor/ }).first().click();
    // Select the main worktree
    await sidebar.getByRole("button", { name: /main/ }).first().click();

    const changesPanel = page.getByTestId("changes-panel");
    await expect(changesPanel.getByText("src/main.rs")).toBeVisible();
    await expect(changesPanel.getByText("src/api.rs")).toBeVisible();
    await expect(changesPanel.getByText("+15")).toBeVisible();
    await expect(changesPanel.getByText("-3")).toBeVisible();

    await page.screenshot({
      path: "e2e/screenshots/changes.png",
      fullPage: true,
    });
  });

  test("status bar shows summary", async ({ page }) => {
    const statusBar = page.getByTestId("status-bar");
    await expect(statusBar.getByText(/2 repos/)).toBeVisible();
    await expect(statusBar.getByText(/3 worktrees/)).toBeVisible();
    await expect(statusBar.getByText(/2 terminals/)).toBeVisible();
  });

  test("full layout screenshot", async ({ page }) => {
    // Select a repo and worktree for full context
    const sidebar = page.getByTestId("sidebar");
    await sidebar.getByRole("button", { name: /arbor/ }).first().click();
    await sidebar.getByRole("button", { name: /main/ }).first().click();

    // Wait for changes to load
    const changesPanel = page.getByTestId("changes-panel");
    await expect(changesPanel.getByText("src/main.rs")).toBeVisible();

    await page.screenshot({
      path: "e2e/screenshots/full-layout.png",
      fullPage: true,
    });
  });

  test("resize handles exist", async ({ page }) => {
    const handles = page.locator(".resize-handle");
    await expect(handles).toHaveCount(2);
  });
});
