import { el, changeKindInfo, shortPath } from "../utils";
import { state, subscribe } from "../state";

export function createChangesPanel(): HTMLElement {
  const panel = el("div", "changes-panel");
  panel.setAttribute("data-testid", "changes-panel");

  function render(): void {
    panel.replaceChildren();

    const header = el("div", "changes-header");
    const title = el("h3", "changes-title", "Changes");
    const count = el("span", "changes-count", String(state.changedFiles.length));
    header.append(title, count);
    panel.append(header);

    if (state.selectedWorktreePath === null) {
      panel.append(el("div", "changes-empty", "Select a worktree"));
      return;
    }

    if (state.changedFiles.length === 0) {
      panel.append(el("div", "changes-empty", "No changes"));
      return;
    }

    const list = el("ul", "changes-list");
    for (const file of state.changedFiles) {
      const info = changeKindInfo(file.kind);
      const item = el("li", "changes-item");

      const statusBadge = el("span", "changes-status");
      statusBadge.textContent = info.code;
      statusBadge.style.color = info.color;

      const pathEl = el("span", "changes-path", file.path);
      pathEl.title = file.path;

      const stats = el("span", "changes-stats");
      if (file.additions > 0) {
        stats.append(el("span", "changes-additions", `+${file.additions}`));
      }
      if (file.deletions > 0) {
        stats.append(el("span", "changes-deletions", `-${file.deletions}`));
      }

      item.append(statusBadge, pathEl, stats);
      list.append(item);
    }
    panel.append(list);
  }

  subscribe(render);
  render();
  return panel;
}
