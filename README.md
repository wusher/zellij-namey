# zellij-namey

A Zellij plugin that automatically renames tabs based on the current working directory and git branch.

## Features

- Auto-renames tabs to `folder:branch` format
- Cross-platform (macOS + Linux)
- Configurable truncation for long names
- Graceful fallback when git is unavailable

## Installation

```bash
# Build the plugin
make setup && make build

# Install to Zellij plugins directory
make install
```

## Configuration

Add to your Zellij config (`~/.config/zellij/config.kdl`):

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

Load on startup by adding to a layout, or load manually with `Ctrl+o` then `w`.

### Options

| Option | Default | Description |
|--------|---------|-------------|
| `folder_max_len` | 10 | Max folder name length before truncation |
| `folder_prefix_len` | 5 | Chars to keep at start when truncating |
| `folder_suffix_len` | 4 | Chars to keep at end when truncating |
| `branch_max_len` | 5 | Max branch name length before truncation |
| `branch_prefix_len` | 1 | Chars to keep at start when truncating |
| `branch_suffix_len` | 4 | Chars to keep at end when truncating |
| `separator` | `:` | Separator between folder and branch |
| `show_branch` | true | Show git branch in tab name |

### Examples

| Folder | Branch | Tab Name |
|--------|--------|----------|
| `myproject` | `main` | `myproject:main` |
| `my_long_project` | `main` | `my_lo…ject:main` |
| `src` | `feature-login` | `src:featu` |
| `project` | _(none)_ | `project` |

## Development

Requires [Rust](https://rustup.rs/) (via mise or rustup).

```bash
make help     # Show available commands
make setup    # Install cargo tools
make lint     # Check formatting + clippy
make test     # Run tests (100% coverage required)
make build    # Build WASM plugin
```

## License

MIT
