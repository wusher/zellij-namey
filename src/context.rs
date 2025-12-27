//! Context management for terminal pane state.
//!
//! This module provides the `PaneContext` struct for representing
//! the current working directory and git branch of a terminal pane.

/// Parsed context information from a terminal pane.
///
/// Contains the current working directory and optional git branch.
#[derive(Debug, Clone)]
pub struct PaneContext {
    /// The full path to the current working directory.
    pub cwd: String,
    /// The current git branch name, or `None` if not in a git repository.
    pub branch: Option<String>,
}

impl PaneContext {
    /// Extracts the folder name from the current working directory path.
    ///
    /// Returns the last component of the path (the directory name), or the
    /// full CWD if no folder name can be extracted (e.g., for root paths).
    ///
    /// # Examples
    ///
    /// For a path `/home/user/project`, returns `"project"`.
    /// For the root path `/`, returns `"/"`.
    pub fn folder_name(&self) -> &str {
        std::path::Path::new(&self.cwd)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&self.cwd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_folder_name_normal() {
        let ctx = PaneContext {
            cwd: "/home/user/project".to_string(),
            branch: Some("main".to_string()),
        };
        assert_eq!(ctx.folder_name(), "project");
    }

    #[test]
    fn test_folder_name_nested() {
        let ctx = PaneContext {
            cwd: "/home/user/deeply/nested/folder".to_string(),
            branch: Some("main".to_string()),
        };
        assert_eq!(ctx.folder_name(), "folder");
    }

    #[test]
    fn test_folder_name_root() {
        let ctx = PaneContext {
            cwd: "/".to_string(),
            branch: Some("main".to_string()),
        };
        assert_eq!(ctx.folder_name(), "/");
    }

    #[test]
    fn test_folder_name_trailing_slash() {
        let ctx = PaneContext {
            cwd: "/home/user/project/".to_string(),
            branch: None,
        };
        assert_eq!(ctx.folder_name(), "project");
    }

    #[test]
    fn test_folder_name_single_component() {
        let ctx = PaneContext {
            cwd: "project".to_string(),
            branch: None,
        };
        assert_eq!(ctx.folder_name(), "project");
    }

    #[test]
    fn test_pane_context_clone() {
        let ctx = PaneContext {
            cwd: "/home/user/project".to_string(),
            branch: Some("main".to_string()),
        };
        let cloned = ctx.clone();
        assert_eq!(cloned.cwd, ctx.cwd);
        assert_eq!(cloned.branch, ctx.branch);
    }

    #[test]
    fn test_pane_context_debug() {
        let ctx = PaneContext {
            cwd: "/home/user/project".to_string(),
            branch: Some("main".to_string()),
        };
        let debug_str = format!("{:?}", ctx);
        assert!(debug_str.contains("project"));
        assert!(debug_str.contains("main"));
    }

    #[test]
    fn test_pane_context_none_branch() {
        let ctx = PaneContext {
            cwd: "/home/user/project".to_string(),
            branch: None,
        };
        assert!(ctx.branch.is_none());
    }
}
