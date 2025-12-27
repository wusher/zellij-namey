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
- `my_lo…ject:main` (truncated folder)
- `src:featu` (truncated branch)
- `src` (no git repo)

### Truncation Rules (Defaults)

| Element | Threshold | Format |
|---------|-----------|--------|
| Folder  | >10 chars | first 5 + `…` + last 4 |
| Branch  | >5 chars  | first 1 + `…` + last 4 (or first 5 if ellipsis doesn't fit) |

### Trigger

Updates on:
- Pane focus change (via `PaneUpdate` event)
- Tab focus change (via `TabUpdate` event)

**Scope**: Renames active tab only. Other tabs are renamed when focused.

## Configuration

Via Zellij plugin config (KDL):

```kdl
plugins {
    namey location="file:~/.config/zellij/plugins/zellij_namey.wasm" {
        folder_max_len 10
        folder_prefix_len 5
        folder_suffix_len 4
        branch_max_len 5
        branch_prefix_len 1
        branch_suffix_len 4
        separator ":"
        show_branch true
    }
}
```

## Technical Approach

### Plugin Events

Subscribes to:
- `TabUpdate` - Tracks current tab index and name
- `PaneUpdate` - Detects focused pane and extracts CWD from title
- `RunCommandResult` - Receives git branch query results
- `PermissionRequestResult` - Handles permission grants

### CWD Detection

The plugin extracts the current working directory from the pane title. Zellij panes typically include the CWD in their title in formats like:
- `zsh: /home/user/project`
- `/home/user/project`
- `vim:/home/user/project`

The plugin parses these patterns to extract the path.

### Git Branch Detection

Once a CWD is detected, the plugin runs:
```bash
git -C "$path" rev-parse --abbrev-ref HEAD 2>/dev/null
```

This is executed via Zellij's `run_command()` API with a context marker to identify our commands.

### Error Handling

- CWD extraction fails → Uses pane title as folder name
- Git command fails → Shows folder only (graceful degradation)
- Empty results → Skips rename, keeps existing tab name

## File Structure

```
zellij-namey/
├── Cargo.toml
├── src/
│   ├── main.rs        # Plugin entry, event handling, git commands
│   ├── context.rs     # PaneContext for CWD/branch data
│   └── formatter.rs   # Name formatting + truncation
└── README.md
```

## Design Decisions

| Question | Decision |
|----------|----------|
| How to get CWD? | Parse pane title (most reliable across platforms) |
| How to get git branch? | `run_command()` with git CLI |
| Truncation character? | Use `…` (single Unicode ellipsis U+2026) |
| Rename scope? | Active tab only |
| Platform support? | macOS + Linux (git CLI is cross-platform) |
| Git detection failure? | Graceful degradation (show folder only) |
