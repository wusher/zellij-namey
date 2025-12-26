#!/bin/bash
# get_context.sh - Cross-platform CWD and git branch retrieval
# Usage: ./get_context.sh <PID>
# Output: <cwd>|<branch>

PID="$1"

if [[ -z "$PID" ]]; then
    echo "|"
    exit 1
fi

# Get CWD (platform-specific)
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS: use lsof to find current working directory
    cwd=$(lsof -p "$PID" 2>/dev/null | awk '/cwd/{print $NF}')
else
    # Linux: read from /proc filesystem
    cwd=$(readlink -f "/proc/$PID/cwd" 2>/dev/null)
fi

# Get git branch (if in a git repo)
branch=""
if [[ -n "$cwd" && -d "$cwd" ]]; then
    branch=$(git -C "$cwd" rev-parse --abbrev-ref HEAD 2>/dev/null)
fi

echo "${cwd}|${branch}"
