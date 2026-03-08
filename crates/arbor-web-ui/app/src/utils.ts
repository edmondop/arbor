export function el<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  className?: string,
  text?: string,
): HTMLElementTagNameMap[K] {
  const element = document.createElement(tag);
  if (className !== undefined) element.className = className;
  if (text !== undefined) element.textContent = text;
  return element;
}

export function formatAge(timestampMs: number | null): string {
  if (timestampMs === null) return "unknown";
  const ageMs = Date.now() - timestampMs;
  if (ageMs < 15_000) return "just now";
  const seconds = Math.floor(ageMs / 1000);
  if (seconds < 60) return `${seconds}s ago`;
  const minutes = Math.floor(seconds / 60);
  if (minutes < 60) return `${minutes}m ago`;
  const hours = Math.floor(minutes / 60);
  if (hours < 24) return `${hours}h ago`;
  const days = Math.floor(hours / 24);
  return `${days}d ago`;
}

export function shortPath(fullPath: string): string {
  const parts = fullPath.replace(/\\/g, "/").split("/").filter(Boolean);
  return parts.length <= 2 ? fullPath : parts.slice(-2).join("/");
}

export function titleFromPath(path: string): string {
  const parts = path.replace(/\\/g, "/").split("/").filter(Boolean);
  const last = parts[parts.length - 1];
  return last !== undefined ? last : "term";
}

export type ChangeKindInfo = {
  code: string;
  color: string;
};

const CHANGE_KIND_MAP: Record<string, ChangeKindInfo> = {
  added:          { code: "A", color: "#a6e3a1" },
  modified:       { code: "M", color: "#f9e2af" },
  removed:        { code: "D", color: "#f38ba8" },
  renamed:        { code: "R", color: "#89dceb" },
  copied:         { code: "C", color: "#74c7ec" },
  "type-change":  { code: "T", color: "#cba6f7" },
  conflict:       { code: "!", color: "#f38ba8" },
  "intent-to-add": { code: "?", color: "#94e2d5" },
};

export function changeKindInfo(kind: string): ChangeKindInfo {
  return CHANGE_KIND_MAP[kind] ?? { code: "?", color: "#9399b2" };
}
