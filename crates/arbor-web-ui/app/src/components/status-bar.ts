import { el } from "../utils";
import { state, subscribe } from "../state";

export function createStatusBar(): HTMLElement {
  const bar = el("footer", "status-bar");
  bar.setAttribute("data-testid", "status-bar");

  function render(): void {
    bar.replaceChildren();

    const left = el("div", "status-left");
    if (state.loading) {
      left.append(el("span", "status-loading", "Loading..."));
    } else if (state.error !== null) {
      left.append(el("span", "status-error", state.error));
    } else {
      left.append(
        el(
          "span",
          "status-info",
          `${state.repositories.length} repos · ${state.worktrees.length} worktrees · ${state.sessions.length} terminals`,
        ),
      );
    }

    const right = el("div", "status-right");
    if (state.selectedWorktreePath !== null) {
      right.append(
        el("span", "status-worktree", state.selectedWorktreePath),
      );
    }

    bar.append(left, right);
  }

  subscribe(render);
  render();
  return bar;
}
