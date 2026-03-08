import { el, formatAge, shortPath } from "../utils";
import {
  state,
  subscribe,
  selectRepository,
  selectWorktree,
  filteredWorktrees,
} from "../state";

export function createSidebar(): HTMLElement {
  const sidebar = el("aside", "sidebar");
  sidebar.setAttribute("data-testid", "sidebar");

  function render(): void {
    sidebar.replaceChildren();

    const header = el("div", "sidebar-header");
    const title = el("h2", "sidebar-title", "Arbor");
    header.append(title);
    sidebar.append(header);

    renderRepos(sidebar);
    renderWorktrees(sidebar);
  }

  subscribe(render);
  render();
  return sidebar;
}

function renderRepos(container: HTMLElement): void {
  const section = el("div", "sidebar-section");
  const heading = el("div", "sidebar-section-heading", "Repositories");
  section.append(heading);

  if (state.repositories.length === 0) {
    section.append(el("div", "sidebar-empty", "No repositories"));
    container.append(section);
    return;
  }

  const list = el("ul", "sidebar-list");
  for (const repo of state.repositories) {
    const item = el("li", "sidebar-item");
    if (state.selectedRepoRoot === repo.root) {
      item.classList.add("active");
    }

    const btn = el("button", "sidebar-item-btn");
    btn.addEventListener("click", () => selectRepository(repo.root));

    const icon = el("span", "sidebar-icon", repoIcon(repo.label));
    const info = el("div", "sidebar-item-info");
    info.append(
      el("span", "sidebar-item-name", repo.label),
      el("span", "sidebar-item-meta", shortPath(repo.root)),
    );
    btn.append(icon, info);
    item.append(btn);
    list.append(item);
  }
  section.append(list);
  container.append(section);
}

function renderWorktrees(container: HTMLElement): void {
  const section = el("div", "sidebar-section");
  const heading = el("div", "sidebar-section-heading", "Worktrees");
  section.append(heading);

  const worktrees = filteredWorktrees();
  if (worktrees.length === 0) {
    section.append(
      el(
        "div",
        "sidebar-empty",
        state.selectedRepoRoot !== null
          ? "No worktrees for this repo"
          : "Select a repository",
      ),
    );
    container.append(section);
    return;
  }

  const list = el("ul", "sidebar-list");
  for (const wt of worktrees) {
    const item = el("li", "sidebar-item");
    if (state.selectedWorktreePath === wt.path) {
      item.classList.add("active");
    }

    const btn = el("button", "sidebar-item-btn");
    btn.addEventListener("click", () => selectWorktree(wt.path));

    const branchChar = wt.branch.charAt(0).toUpperCase();
    const icon = el("span", "sidebar-icon branch-icon", branchChar);
    const info = el("div", "sidebar-item-info");
    const nameText = shortPath(wt.path);
    const metaParts = [wt.branch];
    if (wt.is_primary_checkout) metaParts.push("primary");
    if (wt.last_activity_unix_ms !== null) {
      metaParts.push(formatAge(wt.last_activity_unix_ms));
    }

    info.append(
      el("span", "sidebar-item-name", nameText),
      el("span", "sidebar-item-meta", metaParts.join(" · ")),
    );
    btn.append(icon, info);
    item.append(btn);
    list.append(item);
  }
  section.append(list);
  container.append(section);
}

function repoIcon(label: string): string {
  return label.charAt(0).toUpperCase();
}
