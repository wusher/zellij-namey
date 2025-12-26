# Technical Design Document: zellij-namey

## Overview

A Zellij plugin that automatically renames tabs based on the current working directory and git branch of the focused pane.

## Behavior

### Tab Naming Format

```
<folder>[:<branch>]
```

Examples:
- `myproject:main`
- `my_pr…ject:m-ain` (truncated)
- `src` (no git repo)

### Truncation Rules (Defaults)

| Element | Threshold | Format |
|---------|-----------|--------|
| Folder  | >10 chars | first 5 + `…` + last 4 |
| Branch  | >5 chars  | first char + `-` + last 4 |

### Trigger

Updates on:
- Pane focus change
- Tab focus change

**Scope**: Rename active tab only (other tabs renamed when focused).

### Debouncing

To prevent rename storms during rapid focus changes:
- Wait 150ms after focus event before triggering rename
- Cancel pending rename if new focus event arrives

## Configuration

Via Zellij plugin config (KDL):

```kdl
plugin location="file:~/.config/zellij/plugins/zellij-namey.wasm" {
    folder_max_len 10
    folder_prefix_len 5
    folder_suffix_len 4
    branch_max_len 5
    branch_prefix_len 1
    branch_suffix_len 4
    separator ":"
    show_branch true
}
```

## Technical Approach

### Plugin Events to Subscribe

- `PaneUpdate` (get focused pane info)
- `TabUpdate` (current tab context)

### Data Retrieval

**Problem**: Zellij plugin API doesn't expose CWD directly.

**Solution**: Use `run_command()` to execute a cross-platform helper script.

**PID Source**: Use `terminal_pid` from `PaneInfo` in `PaneManifest` (available via `PaneUpdate` event).

### Helper Script (Cross-Platform)

```bash
#!/bin/bash
# get_context.sh - Cross-platform CWD and git branch retrieval
PID="$1"

# Get CWD (platform-specific)
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    cwd=$(lsof -p "$PID" 2>/dev/null | awk '/cwd/{print $NF}')
else
    # Linux
    cwd=$(readlink -f "/proc/$PID/cwd" 2>/dev/null)
fi

# Get git branch (if in a repo)
if [[ -n "$cwd" ]]; then
    branch=$(git -C "$cwd" rev-parse --abbrev-ref HEAD 2>/dev/null)
fi

echo "${cwd}|${branch}"
```

### Error Handling

- CWD lookup fails → Skip rename, keep existing tab name
- Git detection fails → Show folder only (graceful degradation)
- Process no longer exists → Skip rename
- Command timeout (>500ms) → Skip rename

## File Structure

```
zellij-namey/
├── Cargo.toml
├── src/
│   ├── main.rs        # Plugin entry, event handling
│   ├── context.rs     # CWD + git detection logic
│   └── formatter.rs   # Name formatting + truncation
├── scripts/
│   └── get_context.sh # Cross-platform helper script
└── README.md
```

## Design Decisions

| Question | Decision |
|----------|----------|
| How to get pane PID? | Use `terminal_pid` from `PaneInfo` in `PaneManifest` |
| Truncation character? | Use `…` (single Unicode ellipsis U+2026) |
| Rename scope? | Active tab only |
| Platform support? | macOS + Linux (cross-platform script) |
| Git detection failure? | Graceful degradation (show folder only) |
