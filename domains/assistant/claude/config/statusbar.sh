#!/usr/bin/env bash

CLAUDE_STATE_DIR="$HOME/.config/claude/ide"

get_latest_session() {
  if [ -d "$CLAUDE_STATE_DIR" ]; then
    find "$CLAUDE_STATE_DIR" -name "*.json" -type f -printf '%T@ %p\n' 2>/dev/null | sort -rn | head -1 | cut -d' ' -f2
  fi
}

SESSION_FILE=$(get_latest_session)

if [ -z "$SESSION_FILE" ] || [ ! -f "$SESSION_FILE" ]; then
  echo " --"
  exit 0
fi

if command -v jq &>/dev/null; then
  TOKENS_USED=$(jq -r '.tokensUsed // .contextTokens // empty' "$SESSION_FILE" 2>/dev/null)
  TOKENS_MAX=$(jq -r '.maxTokens // .contextLimit // empty' "$SESSION_FILE" 2>/dev/null)
  
  if [ -n "$TOKENS_USED" ] && [ -n "$TOKENS_MAX" ] && [ "$TOKENS_MAX" != "0" ]; then
    PERCENT=$((TOKENS_USED * 100 / TOKENS_MAX))
    echo " ${PERCENT}%"
  else
    if pgrep -x "claude" >/dev/null; then
      echo " active"
    else
      echo " --"
    fi
  fi
else
  if pgrep -x "claude" >/dev/null; then
    echo " active"
  else
    echo " --"
  fi
fi
