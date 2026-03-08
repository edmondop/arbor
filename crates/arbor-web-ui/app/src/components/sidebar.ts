import { el, formatAge, shortPath } from "../utils";
import type { Repository, Worktree } from "../types";
import {
  state,
  subscribe,
  notify,
  selectWorktree,
} from "../state";

/** Track which repo groups are collapsed (by repo root). */
const collapsedRepos = new Set<string>();

export function createSidebar(): HTMLElement {
  const sidebar = el("aside", "sidebar");
  sidebar.setAttribute("data-testid", "sidebar");

  function render(): void {
    sidebar.replaceChildren();

    const header = el("div", "sidebar-header");
    header.append(el("h2", "sidebar-title", "Arbor"));
    sidebar.append(header);

    const scroll = el("div", "sidebar-scroll");

    if (state.repositories.length === 0) {
      scroll.append(el("div", "sidebar-empty", "No repositories"));
      sidebar.append(scroll);
      return;
    }

    for (const repo of state.repositories) {
      const repoWorktrees = state.worktrees.filter(
        (w) => w.repo_root === repo.root,
      );
      scroll.append(renderRepoGroup(repo, repoWorktrees));
    }

    sidebar.append(scroll);
  }

  subscribe(render);
  render();
  return sidebar;
}

function renderRepoGroup(repo: Repository, worktrees: Worktree[]): HTMLElement {
  const isCollapsed = collapsedRepos.has(repo.root);
  const group = el("div", "repo-group");

  // Repository header row
  const header = el("div", "repo-header");
  header.addEventListener("click", (e) => {
    // Don't toggle if clicking the chevron (it has its own handler)
    if ((e.target as HTMLElement).closest(".repo-chevron")) return;
  });

  const chevron = el("span", "repo-chevron", isCollapsed ? "\u25B8" : "\u25BE");
  chevron.addEventListener("click", (e) => {
    e.stopPropagation();
    if (collapsedRepos.has(repo.root)) {
      collapsedRepos.delete(repo.root);
    } else {
      collapsedRepos.add(repo.root);
    }
    notify();
  });

  const icon = el("span", "repo-icon", repo.label.charAt(0).toUpperCase());

  const name = el("span", "repo-name", repo.label);

  const count = el("span", "repo-wt-count", String(worktrees.length));

  header.append(chevron, icon, name, count);
  group.append(header);

  // Worktree cards (when not collapsed)
  if (!isCollapsed) {
    const wtList = el("div", "wt-list");
    for (const wt of worktrees) {
      wtList.append(renderWorktreeCard(wt));
    }
    group.append(wtList);
  }

  return group;
}

function renderWorktreeCard(wt: Worktree): HTMLElement {
  const isActive = state.selectedWorktreePath === wt.path;
  const card = el("div", "wt-card");
  if (isActive) card.classList.add("active");

  card.addEventListener("click", () => selectWorktree(wt.path));

  // Git branch icon
  const branchIcon = el("span", "wt-branch-icon", "\u{e725}");

  // Text column: two lines
  const info = el("div", "wt-info");

  // Line 1: branch name + diff summary (if we had it) + age
  const line1 = el("div", "wt-line1");
  const branchName = el("span", "wt-branch", wt.branch);
  line1.append(branchName);

  if (wt.last_activity_unix_ms !== null) {
    line1.append(el("span", "wt-age", formatAge(wt.last_activity_unix_ms)));
  }

  // Line 2: path + primary badge
  const line2 = el("div", "wt-line2");
  line2.append(el("span", "wt-path", shortPath(wt.path)));
  if (wt.is_primary_checkout) {
    line2.append(el("span", "wt-badge", "primary"));
  }

  info.append(line1, line2);
  card.append(branchIcon, info);

  return card;
}
