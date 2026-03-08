import "@xterm/xterm/css/xterm.css";
import "./styles/variables.css";
import "./styles/layout.css";
import "./styles/sidebar.css";
import "./styles/terminal.css";
import "./styles/changes.css";
import "./styles/status-bar.css";

import { createSidebar } from "./components/sidebar";
import { createTerminalPanel } from "./components/terminal-panel";
import { createChangesPanel } from "./components/changes-panel";
import { createStatusBar } from "./components/status-bar";
import { refresh } from "./state";

const REFRESH_INTERVAL_MS = 5000;

function bootstrap(): void {
  const appNode = document.getElementById("app");
  if (!(appNode instanceof HTMLDivElement)) {
    throw new Error("missing #app root");
  }

  const shell = document.createElement("div");
  shell.className = "app-shell";

  const mainLayout = document.createElement("div");
  mainLayout.className = "main-layout";

  const sidebar = createSidebar();
  const leftHandle = createResizeHandle(sidebar, "left");
  const terminalPanel = createTerminalPanel();
  const rightHandle = createResizeHandle(null, "right"); // assigned after changesPanel
  const changesPanel = createChangesPanel();
  rightHandle.dataset["target"] = "right";

  mainLayout.append(sidebar, leftHandle, terminalPanel, rightHandle, changesPanel);

  const statusBar = createStatusBar();

  shell.append(mainLayout, statusBar);
  appNode.append(shell);

  // Setup resize handles
  setupResize(leftHandle, sidebar, "left");
  setupResize(rightHandle, changesPanel, "right");

  // Initial data fetch
  void refresh();
  setInterval(() => { void refresh(); }, REFRESH_INTERVAL_MS);
}

function createResizeHandle(_target: HTMLElement | null, _side: string): HTMLDivElement {
  const handle = document.createElement("div");
  handle.className = "resize-handle";
  return handle;
}

function setupResize(handle: HTMLElement, target: HTMLElement, side: "left" | "right"): void {
  let startX = 0;
  let startWidth = 0;

  function onMouseDown(event: MouseEvent): void {
    event.preventDefault();
    startX = event.clientX;
    startWidth = target.getBoundingClientRect().width;
    handle.classList.add("dragging");
    document.addEventListener("mousemove", onMouseMove);
    document.addEventListener("mouseup", onMouseUp);
    document.body.style.cursor = "col-resize";
    document.body.style.userSelect = "none";
  }

  function onMouseMove(event: MouseEvent): void {
    const delta = event.clientX - startX;
    const newWidth = side === "left" ? startWidth + delta : startWidth - delta;
    const clamped = Math.max(200, Math.min(400, newWidth));
    target.style.width = `${clamped}px`;
  }

  function onMouseUp(): void {
    handle.classList.remove("dragging");
    document.removeEventListener("mousemove", onMouseMove);
    document.removeEventListener("mouseup", onMouseUp);
    document.body.style.cursor = "";
    document.body.style.userSelect = "";
  }

  handle.addEventListener("mousedown", onMouseDown);
}

bootstrap();
