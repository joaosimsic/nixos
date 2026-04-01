#!/bin/bash
# Ralph Wiggum - Long-running Claude agent loop
# Usage: ./scripts/ralph/ralph.sh [max_iterations]

set -euo pipefail

usage() {
  echo "Usage: $0 [max_iterations]"
  echo ""
  echo "Runs Ralph with Claude for a number of iterations (default: 10)."
}

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  usage
  exit 0
fi

MAX_ITERATIONS=10
if [[ $# -gt 0 ]]; then
  if [[ "$1" =~ ^[0-9]+$ ]]; then
    MAX_ITERATIONS="$1"
  else
    echo "Error: max_iterations must be a non-negative integer."
    usage
    exit 1
  fi
fi

if ! command -v claude >/dev/null 2>&1; then
  echo "Error: 'claude' command not found in PATH."
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

PRD_FILE="$ROOT_DIR/prd.json"
PROGRESS_FILE="$ROOT_DIR/progress.txt"
ARCHIVE_DIR="$ROOT_DIR/archive"
LAST_BRANCH_FILE="$ROOT_DIR/.last-branch"
CLAUDE_PROMPT_FILE="$ROOT_DIR/CLAUDE.md"

# Archive previous run if branch changed (based on prd.json metadata)
if [[ -f "$PRD_FILE" && -f "$LAST_BRANCH_FILE" ]]; then
  if ! command -v jq >/dev/null 2>&1; then
    echo "Error: 'jq' is required when '$PRD_FILE' exists."
    exit 1
  fi

  CURRENT_BRANCH="$(jq -r '.branchName // empty' "$PRD_FILE" 2>/dev/null || echo "")"
  LAST_BRANCH="$(<"$LAST_BRANCH_FILE" 2>/dev/null || echo "")"

  if [[ -n "$CURRENT_BRANCH" && -n "$LAST_BRANCH" && "$CURRENT_BRANCH" != "$LAST_BRANCH" ]]; then
    DATE="$(date +%Y-%m-%d)"
    FOLDER_NAME="$(echo "$LAST_BRANCH" | sed 's|^ralph/||')"
    ARCHIVE_FOLDER="$ARCHIVE_DIR/$DATE-$FOLDER_NAME"

    echo "Archiving previous run: $LAST_BRANCH"
    mkdir -p "$ARCHIVE_FOLDER"
    [[ -f "$PRD_FILE" ]] && cp "$PRD_FILE" "$ARCHIVE_FOLDER/"
    [[ -f "$PROGRESS_FILE" ]] && cp "$PROGRESS_FILE" "$ARCHIVE_FOLDER/"
    echo "Archived to: $ARCHIVE_FOLDER"

    echo "# Ralph Progress Log" > "$PROGRESS_FILE"
    echo "Started: $(date)" >> "$PROGRESS_FILE"
    echo "---" >> "$PROGRESS_FILE"
  fi
fi

# Track current branch from prd.json when available
if [[ -f "$PRD_FILE" ]]; then
  if ! command -v jq >/dev/null 2>&1; then
    echo "Error: 'jq' is required when '$PRD_FILE' exists."
    exit 1
  fi

  CURRENT_BRANCH="$(jq -r '.branchName // empty' "$PRD_FILE" 2>/dev/null || echo "")"
  if [[ -n "$CURRENT_BRANCH" ]]; then
    echo "$CURRENT_BRANCH" > "$LAST_BRANCH_FILE"
  fi
fi

if [[ ! -f "$PROGRESS_FILE" ]]; then
  echo "# Ralph Progress Log" > "$PROGRESS_FILE"
  echo "Started: $(date)" >> "$PROGRESS_FILE"
  echo "---" >> "$PROGRESS_FILE"
fi

if [[ ! -f "$CLAUDE_PROMPT_FILE" ]]; then
  echo "Error: '$CLAUDE_PROMPT_FILE' not found."
  exit 1
fi

echo "Starting Ralph - Tool: claude - Max iterations: $MAX_ITERATIONS"

for i in $(seq 1 "$MAX_ITERATIONS"); do
  echo ""
  echo "==============================================================="
  echo "  Ralph Iteration $i of $MAX_ITERATIONS (claude)"
  echo "==============================================================="

  OUTPUT="$(claude --dangerously-skip-permissions --print < "$CLAUDE_PROMPT_FILE" 2>&1 | tee /dev/stderr)" || true

  if [[ "$OUTPUT" == *"<promise>COMPLETE</promise>"* ]]; then
    echo ""
    echo "Ralph completed all tasks!"
    echo "Completed at iteration $i of $MAX_ITERATIONS"
    exit 0
  fi

  echo "Iteration $i complete. Continuing..."
  sleep 2
done

echo ""
echo "Ralph reached max iterations ($MAX_ITERATIONS) without completing all tasks."
echo "Check $PROGRESS_FILE for status."
exit 1
