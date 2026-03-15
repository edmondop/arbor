import { el, formatAge, shortPath } from "../utils";
import type { Issue, Repository, Worktree } from "../types";
import {
  state,
  subscribe,
  notify,
  selectWorktree,
  agentStateForWorktree,
  selectedIssueRepoRoot,
  refreshIssues,
} from "../state";
import { openCreateWorktreeModal } from "./create-worktree-modal";

const COLLAPSED_REPOS_STORAGE_KEY = "arbor.sidebar.collapsedRepos";
const SIDEBAR_TABS_STORAGE_KEY = "arbor.sidebar.repoTabs";

type RepoSidebarTab = "worktrees" | "issues";

/** Track which repo groups are collapsed (by repo root). */
const collapsedRepos = loadCollapsedRepos();

/** Track which sidebar tab is active per repo (worktrees is default). */
const repoSidebarTabs = loadRepoSidebarTabs();

export function createSidebar(): HTMLElement {
  const sidebar = el("aside", "sidebar");
  sidebar.setAttribute("data-testid", "sidebar");

  function render(): void {
    sidebar.replaceChildren();

    const header = el("div", "sidebar-header");
    header.append(el("h2", "sidebar-title", "Arbor"));
    sidebar.append(header);

    const scroll = el("div", "sidebar-scroll");

    if (state.loading && state.repositories.length === 0) {
      scroll.append(el("div", "sidebar-loading", "Loading\u2026"));
      sidebar.append(scroll);
      return;
    }

    if (!state.loading && state.repositories.length === 0) {
      scroll.append(el("div", "sidebar-empty", "No repositories"));
      sidebar.append(scroll);
      return;
    }

    pruneCollapsedRepos(state.repositories);

    const issueRepoRoot = selectedIssueRepoRoot();

    for (const repo of state.repositories) {
      const repoWorktrees = state.worktrees.filter((w) => w.repo_root === repo.root);
      const hasIssues = issueRepoRoot === repo.root;
      scroll.append(renderRepoGroup(repo, repoWorktrees, hasIssues));
    }

    sidebar.append(scroll);
  }

  subscribe(render);
  render();
  return sidebar;
}

function renderRepoGroup(repo: Repository, worktrees: Worktree[], hasIssues: boolean): HTMLElement {
  const isCollapsed = collapsedRepos.has(repo.root);
  const activeTab = repoSidebarTabs.get(repo.root) ?? "worktrees";
  const group = el("div", "repo-group");

  const header = el("div", "repo-header");

  const chevron = el("span", "repo-chevron", isCollapsed ? "\u25B8" : "\u25BE");
  chevron.addEventListener("click", (e) => {
    e.stopPropagation();
    if (collapsedRepos.has(repo.root)) {
      collapsedRepos.delete(repo.root);
    } else {
      collapsedRepos.add(repo.root);
    }
    persistCollapsedRepos();
    notify();
  });

  const icon = renderRepoIcon(repo);
  const name = el("span", "repo-name", repo.label);
  const count = el("span", "repo-wt-count", String(worktrees.length));

  header.append(chevron, icon, name, count);
  group.append(header);

  if (!isCollapsed) {
    // Subtabs: Worktrees | Issues
    const nonPrimaryCount = worktrees.filter((w) => !w.is_primary_checkout).length;
    const issueCount = hasIssues ? issuesBadgeLabel() : null;
    group.append(renderRepoSubtabs(repo.root, activeTab, nonPrimaryCount, issueCount));

    if (activeTab === "issues" && hasIssues) {
      group.append(renderIssuesContent(repo.root));
    } else {
      const wtList = el("div", "wt-list");
      const sorted = [...worktrees].sort((a, b) => {
        if (a.is_primary_checkout !== b.is_primary_checkout) {
          return a.is_primary_checkout ? -1 : 1;
        }
        return 0;
      });
      for (const wt of sorted) {
        wtList.append(renderWorktreeCard(wt));
      }
      group.append(wtList);
    }
  }

  return group;
}

function issuesBadgeLabel(): string | null {
  if (state.issuesLoading && state.issues.length === 0) {
    return "\u2026";
  }
  if (state.issuesLoadedRepoRoot !== null || state.issuesNotice !== null || state.issuesError !== null) {
    return String(state.issues.length);
  }
  return null;
}

function renderRepoSubtabs(
  repoRoot: string,
  activeTab: RepoSidebarTab,
  worktreeCount: number,
  issueBadge: string | null,
): HTMLElement {
  const tabs = el("div", "repo-subtabs");

  const wtBtn = el("button", "repo-subtab-btn");
  wtBtn.type = "button";
  if (activeTab === "worktrees") wtBtn.classList.add("active");
  const wtContent = el("span", "repo-subtab-content");
  wtContent.append(el("span", "", "Worktrees"));
  wtContent.append(el("span", "repo-subtab-badge", String(worktreeCount)));
  wtBtn.append(wtContent);
  wtBtn.addEventListener("click", (e) => {
    e.stopPropagation();
    setRepoSidebarTab(repoRoot, "worktrees");
  });

  const issueBtn = el("button", "repo-subtab-btn");
  issueBtn.type = "button";
  if (activeTab === "issues") issueBtn.classList.add("active");
  const issueContent = el("span", "repo-subtab-content");
  issueContent.append(el("span", "", "Issues"));
  if (issueBadge !== null) {
    issueContent.append(el("span", "repo-subtab-badge", issueBadge));
  }
  issueBtn.append(issueContent);
  issueBtn.addEventListener("click", (e) => {
    e.stopPropagation();
    setRepoSidebarTab(repoRoot, "issues");
    // Ensure issues are loaded when switching to issues tab
    refreshIssues(repoRoot, false);
  });

  tabs.append(wtBtn, issueBtn);
  return tabs;
}

function setRepoSidebarTab(repoRoot: string, tab: RepoSidebarTab): void {
  if (tab === "worktrees") {
    repoSidebarTabs.delete(repoRoot);
  } else {
    repoSidebarTabs.set(repoRoot, tab);
  }
  persistRepoSidebarTabs();
  notify();
}

function renderRepoIcon(repo: Repository): HTMLElement {
  if (repo.avatar_url !== null) {
    const img = document.createElement("img");
    img.className = "repo-avatar";
    img.src = repo.avatar_url;
    img.alt = repo.label;
    img.width = 20;
    img.height = 20;
    img.addEventListener("error", () => {
      const fallback = el("span", "repo-icon", repo.label.charAt(0).toUpperCase());
      img.replaceWith(fallback);
    });
    return img;
  }

  if (repo.github_repo_slug !== null) {
    const icon = el("span", "repo-icon repo-icon-github");
    // Static SVG icon, not user content
    icon.innerHTML = GITHUB_SVG; // eslint-disable-line no-unsanitized/property
    return icon;
  }

  return el("span", "repo-icon", repo.label.charAt(0).toUpperCase());
}

function renderWorktreeCard(wt: Worktree): HTMLElement {
  const isActive = state.selectedWorktreePath === wt.path;
  const card = el("div", "wt-card");
  if (isActive) card.classList.add("active");

  card.addEventListener("click", () => selectWorktree(wt.path));

  const main = el("div", "wt-card-main");

  const agentState = agentStateForWorktree(wt.path);
  let leadingIcon: HTMLElement;
  if (agentState !== null) {
    const dotClass = agentState === "working" ? "dot-working" : "dot-waiting";
    leadingIcon = el("span", `wt-agent-dot ${dotClass}`);
  } else {
    leadingIcon = el("span", "wt-branch-icon");
    // Static SVG icon, not user content
    leadingIcon.innerHTML = GIT_BRANCH_SVG; // eslint-disable-line no-unsanitized/property
  }

  const info = el("div", "wt-info");

  const line1 = el("div", "wt-line1");
  line1.append(el("span", "wt-branch", wt.branch));

  const hasAdditions = wt.diff_additions !== null && wt.diff_additions > 0;
  const hasDeletions = wt.diff_deletions !== null && wt.diff_deletions > 0;
  if (hasAdditions || hasDeletions) {
    const stats = el("span", "wt-diff-stats");
    if (hasAdditions) {
      stats.append(el("span", "wt-diff-add", `+${wt.diff_additions}`));
    }
    if (hasDeletions) {
      stats.append(el("span", "wt-diff-del", `-${wt.diff_deletions}`));
    }
    line1.append(stats);
  }

  if (wt.pr_number !== null) {
    const prBadge = el("span", "wt-pr");
    if (wt.pr_url !== null) {
      const link = document.createElement("a");
      link.href = wt.pr_url;
      link.target = "_blank";
      link.rel = "noopener";
      link.textContent = `#${wt.pr_number}`;
      link.addEventListener("click", (e) => e.stopPropagation());
      prBadge.append(link);
    } else {
      prBadge.textContent = `#${wt.pr_number}`;
    }
    line1.append(prBadge);
  }

  if (wt.last_activity_unix_ms !== null) {
    line1.append(el("span", "wt-age", formatAge(wt.last_activity_unix_ms)));
  }

  const line2 = el("div", "wt-line2");
  line2.append(el("span", "wt-path", shortPath(wt.path)));

  info.append(line1, line2);
  main.append(leadingIcon, info);
  card.append(main);

  return card;
}

// ── Issues section ─────────────────────────────────────────────────

function renderIssuesContent(repoRoot: string): HTMLElement {
  const wrapper = el("div", "sidebar-issues-content");

  // Source info + actions bar
  const sourceBar = el("div", "sidebar-issues-source-bar");
  if (state.issueSource !== null) {
    sourceBar.append(
      el("span", "sidebar-issues-source", `${state.issueSource.label} \u00b7 ${state.issueSource.repository}`),
    );
  }

  const actions = el("div", "sidebar-issues-actions");
  if (state.issueSource?.url != null) {
    const link = document.createElement("a");
    link.className = "sidebar-issues-link";
    link.href = state.issueSource.url;
    link.target = "_blank";
    link.rel = "noopener";
    link.textContent = "Open";
    link.addEventListener("click", (e) => e.stopPropagation());
    actions.append(link);
  }
  const refreshButton = el("button", "sidebar-issues-refresh", "\u21BB");
  refreshButton.title = "Refresh issues";
  refreshButton.type = "button";
  refreshButton.disabled = state.issuesLoading;
  refreshButton.addEventListener("click", () => {
    refreshIssues(repoRoot, true);
  });
  actions.append(refreshButton);
  sourceBar.append(actions);
  wrapper.append(sourceBar);

  if (state.issuesLoading && state.issues.length === 0) {
    wrapper.append(el("div", "sidebar-issues-empty", "Loading issues\u2026"));
    return wrapper;
  }

  if (state.issuesError !== null) {
    wrapper.append(el("div", "sidebar-issues-empty sidebar-issues-error", state.issuesError));
    return wrapper;
  }

  if (state.issuesNotice !== null) {
    wrapper.append(el("div", "sidebar-issues-empty", state.issuesNotice));
    return wrapper;
  }

  if (state.issues.length === 0) {
    wrapper.append(el("div", "sidebar-issues-empty", "No open issues"));
    return wrapper;
  }

  const list = el("div", "sidebar-issues-list");
  for (const issue of state.issues) {
    list.append(renderIssueCard(issue));
  }
  wrapper.append(list);
  return wrapper;
}

function renderIssueCard(issue: Issue): HTMLElement {
  const linkedReview = issue.linked_review;
  const linkedBranch = issue.linked_branch;

  const card = el("div", "sidebar-issue-card");
  card.setAttribute("role", "button");
  card.tabIndex = 0;
  card.addEventListener("click", () => openCreateWorktreeModal(issue));
  card.addEventListener("keydown", (event) => {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      openCreateWorktreeModal(issue);
    }
  });

  const topRow = el("div", "sidebar-issue-top");
  topRow.append(
    el("span", "sidebar-issue-id", issue.display_id),
    el("span", "sidebar-issue-title", issue.title),
  );

  if (issue.url !== null) {
    const link = document.createElement("a");
    link.className = "sidebar-issue-link";
    link.href = issue.url;
    link.target = "_blank";
    link.rel = "noopener";
    link.textContent = "\u2197";
    link.title = "Open in browser";
    link.addEventListener("click", (event) => {
      event.stopPropagation();
    });
    topRow.append(link);
  }

  card.append(topRow);

  if (linkedReview !== null || linkedBranch !== null) {
    const linkedRow = el("div", "sidebar-issue-linked");

    if (linkedReview !== null) {
      if (linkedReview.url !== null) {
        const reviewLink = document.createElement("a");
        reviewLink.className = "sidebar-issue-chip sidebar-issue-chip-review";
        reviewLink.href = linkedReview.url;
        reviewLink.target = "_blank";
        reviewLink.rel = "noopener";
        reviewLink.textContent = linkedReview.label;
        reviewLink.addEventListener("click", (event) => {
          event.stopPropagation();
        });
        linkedRow.append(reviewLink);
      } else {
        linkedRow.append(
          el("span", "sidebar-issue-chip sidebar-issue-chip-review", linkedReview.label),
        );
      }
    }

    if (linkedBranch !== null) {
      linkedRow.append(el("span", "sidebar-issue-chip sidebar-issue-chip-branch", linkedBranch));
    }

    card.append(linkedRow);
  }

  const bottomRow = el("div", "sidebar-issue-bottom");
  const ctaLabel = linkedReview !== null
    ? linkedReview.kind === "merge_request"
      ? "MR exists"
      : "PR exists"
    : linkedBranch !== null
      ? "Branch exists"
      : "Create worktree";
  bottomRow.append(
    el("span", "sidebar-issue-state", issue.state),
    el(
      "span",
      "sidebar-issue-age",
      issue.updated_at === null ? "recently" : formatIssueAge(issue.updated_at),
    ),
    el("span", "sidebar-issue-cta", ctaLabel),
  );

  card.append(bottomRow);
  return card;
}

function formatIssueAge(updatedAt: string): string {
  const timestamp = Date.parse(updatedAt);
  if (Number.isNaN(timestamp)) {
    return updatedAt;
  }
  return formatAge(timestamp);
}

// ── SVG icons ──────────────────────────────────────────────────────

const GITHUB_SVG = `<svg viewBox="0 0 16 16" width="16" height="16" fill="currentColor"><path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"/></svg>`;

const GIT_BRANCH_SVG = `<svg viewBox="0 0 16 16" width="14" height="14" fill="currentColor"><path d="M9.5 3.25a2.25 2.25 0 1 1 3 2.122V6A2.5 2.5 0 0 1 10 8.5H6a1 1 0 0 0-1 1v1.128a2.251 2.251 0 1 1-1.5 0V5.372a2.25 2.25 0 1 1 1.5 0v1.836A2.493 2.493 0 0 1 6 7h4a1 1 0 0 0 1-1v-.628A2.25 2.25 0 0 1 9.5 3.25zm-6 0a.75.75 0 1 0 1.5 0 .75.75 0 0 0-1.5 0zm8.25-.75a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5zM4.25 12a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5z"/></svg>`;

// ── Collapsed repos persistence ─────────────────────────────────────

function loadCollapsedRepos(): Set<string> {
  if (typeof window === "undefined") {
    return new Set<string>();
  }

  try {
    const raw = window.localStorage.getItem(COLLAPSED_REPOS_STORAGE_KEY);
    if (raw === null) {
      return new Set<string>();
    }

    const parsed: unknown = JSON.parse(raw);
    if (!Array.isArray(parsed)) {
      return new Set<string>();
    }

    const repoRoots = parsed.filter((value): value is string => typeof value === "string");
    return new Set<string>(repoRoots);
  } catch {
    return new Set<string>();
  }
}

function persistCollapsedRepos(): void {
  if (typeof window === "undefined") {
    return;
  }

  try {
    window.localStorage.setItem(
      COLLAPSED_REPOS_STORAGE_KEY,
      JSON.stringify([...collapsedRepos].sort()),
    );
  } catch {
    // Ignore storage failures, the sidebar can still function without persistence.
  }
}

function loadRepoSidebarTabs(): Map<string, RepoSidebarTab> {
  if (typeof window === "undefined") {
    return new Map();
  }

  try {
    const raw = window.localStorage.getItem(SIDEBAR_TABS_STORAGE_KEY);
    if (raw === null) {
      return new Map();
    }

    const parsed: unknown = JSON.parse(raw);
    if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {
      return new Map();
    }

    const result = new Map<string, RepoSidebarTab>();
    for (const [key, value] of Object.entries(parsed as Record<string, unknown>)) {
      if (value === "issues") {
        result.set(key, "issues");
      }
    }
    return result;
  } catch {
    return new Map();
  }
}

function persistRepoSidebarTabs(): void {
  if (typeof window === "undefined") {
    return;
  }

  try {
    const obj: Record<string, string> = {};
    for (const [key, value] of repoSidebarTabs) {
      obj[key] = value;
    }
    window.localStorage.setItem(SIDEBAR_TABS_STORAGE_KEY, JSON.stringify(obj));
  } catch {
    // Ignore storage failures
  }
}

function pruneCollapsedRepos(repositories: Repository[]): void {
  const repositoryRoots = new Set(repositories.map((repository) => repository.root));
  let changed = false;

  for (const repoRoot of collapsedRepos) {
    if (repositoryRoots.has(repoRoot)) {
      continue;
    }

    collapsedRepos.delete(repoRoot);
    changed = true;
  }

  if (changed) {
    persistCollapsedRepos();
  }
}
