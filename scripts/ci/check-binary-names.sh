#!/usr/bin/env bash
# Detect case-insensitive filename collisions.
#
# Usage:
#   check-binary-names.sh <directory>    — check files in a directory
#   check-binary-names.sh --cargo        — check [[bin]] names across workspace Cargo.toml files
#
# Exits 0 if no collisions, 1 if duplicates are found.
set -euo pipefail

check_directory() {
  local dir="$1"
  if [[ ! -d "$dir" ]]; then
    echo "error: directory not found: $dir" >&2
    exit 1
  fi

  local dupes
  dupes="$(
    for path in "$dir"/*; do
      [[ -f "$path" ]] || continue
      basename "$path" | tr '[:upper:]' '[:lower:]'
    done | sort | uniq -d
  )"

  if [[ -n "$dupes" ]]; then
    echo "error: case-insensitive collision(s) in $dir:" >&2
    echo "$dupes" >&2
    exit 1
  fi
  echo "ok: no case-insensitive collisions in $dir"
}

check_cargo() {
  local root_dir
  root_dir="$(cd "$(dirname "$0")/../.." && pwd)"

  local dupes
  dupes="$(
    find "$root_dir/crates" -name Cargo.toml -exec \
      awk '/^\[\[bin\]\]/{b=1;next} /^\[/{b=0} b && /^name[ \t]*=/{gsub(/.*= *"/,""); gsub(/".*/,""); print}' {} + \
    | tr '[:upper:]' '[:lower:]' | sort | uniq -d
  )"

  if [[ -n "$dupes" ]]; then
    echo "error: case-insensitive binary name collision(s):" >&2
    echo "$dupes" >&2
    exit 1
  fi
  echo "ok: no case-insensitive binary name collisions in workspace"
}

if [[ $# -lt 1 ]]; then
  echo "usage: $0 <directory> | --cargo" >&2
  exit 1
fi

case "$1" in
  --cargo) check_cargo ;;
  *)       check_directory "$1" ;;
esac
