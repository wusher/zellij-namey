//! Context management for terminal pane state.
//!
//! This module handles tracking pending context requests and parsing
//! the results from the context-fetching shell script.

/// Represents a pending context request for a terminal pane.
///
/// Used to track in-flight requests to determine the current working
/// directory and git branch of a terminal process.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ContextRequest {
    /// The process ID being queried.
    pid: i32,
}

impl ContextRequest {
    /// Creates a new context request for the given process ID.
    ///
    /// # Arguments
    ///
    /// * `pid` - The process ID of the terminal to query
    #[allow(dead_code)]
    pub fn new(pid: i32) -> Self {
        Self { pid }
    }

    /// Returns the process ID associated with this request.
    #[allow(dead_code)]
    pub fn pid(&self) -> i32 {
        self.pid
    }
}

/// Parsed context information from a terminal pane.
///
/// Contains the current working directory and optional git branch
/// extracted from the helper script output.
#[derive(Debug, Clone)]
pub struct PaneContext {
    /// The full path to the current working directory.
    pub cwd: String,
    /// The current git branch name, or `None` if not in a git repository.
    pub branch: Option<String>,
}

impl PaneContext {
    /// Parses the helper script output in `"cwd|branch"` format.
    ///
    /// The expected format is a pipe-separated string where:
    /// - The first part is the absolute path to the current working directory
    /// - The second part is the git branch name (may be empty)
    ///
    /// # Arguments
    ///
    /// * `output` - The raw output string from the context script
    ///
    /// # Returns
    ///
    /// `Some(PaneContext)` if parsing succeeds, `None` if the format is invalid
    /// or the CWD is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use namey::context::PaneContext;
    ///
    /// let ctx = PaneContext::parse("/home/user/project|main").unwrap();
    /// assert_eq!(ctx.cwd, "/home/user/project");
    /// assert_eq!(ctx.branch, Some("main".to_string()));
    ///
    /// let ctx = PaneContext::parse("/home/user/project|").unwrap();
    /// assert_eq!(ctx.branch, None);
    /// ```
    #[allow(dead_code)]
    pub fn parse(output: &str) -> Option<Self> {
        let output = output.trim();
        let (cwd, branch) = output.split_once('|')?;

        if cwd.is_empty() {
            return None;
        }

        Some(Self {
            cwd: cwd.to_string(),
            branch: if branch.is_empty() {
                None
            } else {
                Some(branch.to_string())
            },
        })
    }

    /// Extracts the folder name from the current working directory path.
    ///
    /// Returns the last component of the path (the directory name), or the
    /// full CWD if no folder name can be extracted (e.g., for root paths).
    ///
    /// # Examples
    ///
    /// ```
    /// use namey::context::PaneContext;
    ///
    /// let ctx = PaneContext::parse("/home/user/project|main").unwrap();
    /// assert_eq!(ctx.folder_name(), "project");
    /// ```
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

    // ==================== ContextRequest Tests ====================

    #[test]
    fn test_context_request_new() {
        let req = ContextRequest::new(12345);
        assert_eq!(req.pid(), 12345);
    }

    #[test]
    fn test_context_request_clone() {
        let req = ContextRequest::new(12345);
        let cloned = req.clone();
        assert_eq!(cloned.pid(), 12345);
    }

    #[test]
    fn test_context_request_debug() {
        let req = ContextRequest::new(12345);
        let debug_str = format!("{:?}", req);
        assert!(debug_str.contains("12345"));
    }

    // ==================== PaneContext::parse() Tests ====================

    #[test]
    fn test_parse_with_branch() {
        let ctx = PaneContext::parse("/home/user/project|main").unwrap();
        assert_eq!(ctx.cwd, "/home/user/project");
        assert_eq!(ctx.branch, Some("main".to_string()));
    }

    #[test]
    fn test_parse_without_branch() {
        let ctx = PaneContext::parse("/home/user/project|").unwrap();
        assert_eq!(ctx.cwd, "/home/user/project");
        assert_eq!(ctx.branch, None);
    }

    #[test]
    fn test_parse_empty_cwd() {
        assert!(PaneContext::parse("|main").is_none());
    }

    #[test]
    fn test_parse_empty_both() {
        assert!(PaneContext::parse("|").is_none());
    }

    #[test]
    fn test_parse_no_pipe() {
        // No pipe separator means split_once returns None
        assert!(PaneContext::parse("/home/user/project").is_none());
    }

    #[test]
    fn test_parse_multiple_pipes() {
        // Only first pipe is used as separator
        let ctx = PaneContext::parse("/home/user/project|feature|extra").unwrap();
        assert_eq!(ctx.cwd, "/home/user/project");
        assert_eq!(ctx.branch, Some("feature|extra".to_string()));
    }

    #[test]
    fn test_parse_with_leading_whitespace() {
        let ctx = PaneContext::parse("  /home/user/project|main").unwrap();
        assert_eq!(ctx.cwd, "/home/user/project");
    }

    #[test]
    fn test_parse_with_trailing_whitespace() {
        let ctx = PaneContext::parse("/home/user/project|main  ").unwrap();
        assert_eq!(ctx.branch, Some("main".to_string()));
    }

    #[test]
    fn test_parse_with_newline() {
        let ctx = PaneContext::parse("/home/user/project|feature\n").unwrap();
        assert_eq!(ctx.branch, Some("feature".to_string()));
    }

    #[test]
    fn test_parse_with_carriage_return() {
        let ctx = PaneContext::parse("/home/user/project|feature\r\n").unwrap();
        assert_eq!(ctx.branch, Some("feature".to_string()));
    }

    #[test]
    fn test_parse_macos_path() {
        let ctx = PaneContext::parse("/Users/wusher/code/project|develop").unwrap();
        assert_eq!(ctx.cwd, "/Users/wusher/code/project");
        assert_eq!(ctx.branch, Some("develop".to_string()));
    }

    #[test]
    fn test_parse_root_path() {
        let ctx = PaneContext::parse("/|main").unwrap();
        assert_eq!(ctx.cwd, "/");
    }

    #[test]
    fn test_parse_branch_with_slashes() {
        let ctx = PaneContext::parse("/home/user/project|feature/ABC-123").unwrap();
        assert_eq!(ctx.branch, Some("feature/ABC-123".to_string()));
    }

    // ==================== PaneContext::folder_name() Tests ====================

    #[test]
    fn test_folder_name_normal() {
        let ctx = PaneContext::parse("/home/user/project|main").unwrap();
        assert_eq!(ctx.folder_name(), "project");
    }

    #[test]
    fn test_folder_name_nested() {
        let ctx = PaneContext::parse("/home/user/deeply/nested/folder|main").unwrap();
        assert_eq!(ctx.folder_name(), "folder");
    }

    #[test]
    fn test_folder_name_root() {
        let ctx = PaneContext::parse("/|main").unwrap();
        // Root path has no file_name, falls back to full cwd
        assert_eq!(ctx.folder_name(), "/");
    }

    #[test]
    fn test_folder_name_trailing_slash() {
        // Rust's Path::file_name handles trailing slashes correctly
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

    // ==================== PaneContext Clone/Debug Tests ====================

    #[test]
    fn test_pane_context_clone() {
        let ctx = PaneContext::parse("/home/user/project|main").unwrap();
        let cloned = ctx.clone();
        assert_eq!(cloned.cwd, ctx.cwd);
        assert_eq!(cloned.branch, ctx.branch);
    }

    #[test]
    fn test_pane_context_debug() {
        let ctx = PaneContext::parse("/home/user/project|main").unwrap();
        let debug_str = format!("{:?}", ctx);
        assert!(debug_str.contains("project"));
        assert!(debug_str.contains("main"));
    }
}
