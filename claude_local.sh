#!/usr/bin/env bash
# Run Claude Code against a local OpenAI-compatible model server.
set -euo pipefail

BASE_URL="http://127.0.0.1:8000"
MODEL="models--mlx-community--Ornith-1.0-35B-8bit"

export ANTHROPIC_BASE_URL="$BASE_URL"
export ANTHROPIC_AUTH_TOKEN="local"      # server doesn't check this, but Claude Code requires it set
export ANTHROPIC_MODEL="$MODEL"
export ANTHROPIC_SMALL_FAST_MODEL="$MODEL"

# fail fast if the local server isn't up
if ! curl -s -o /dev/null --max-time 2 "$BASE_URL/v1/models"; then
  echo "Error: no server responding at $BASE_URL" >&2
  exit 1
fi

exec claude "$@"