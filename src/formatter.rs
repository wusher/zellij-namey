//! Tab name formatting utilities.
//!
//! This module provides configuration and functions for formatting tab names
//! from folder paths and git branch names, with support for intelligent truncation.

use std::collections::BTreeMap;

/// Configuration for tab name formatting.
///
/// Controls how folder names and git branches are displayed in tab names,
/// including maximum lengths, truncation behavior, and separators.
///
/// # Truncation Strategy
///
/// When a name exceeds its maximum length, it is truncated using an ellipsis
/// in the middle, preserving both the prefix and suffix. For example, with
/// `prefix_len = 5` and `suffix_len = 4`, the folder `"my_long_project_name"`
/// becomes `"my_lo…name"`.
///
/// # Default Values
///
/// | Field | Default |
/// |-------|---------|
/// | `folder_max_len` | 10 |
/// | `folder_prefix_len` | 5 |
/// | `folder_suffix_len` | 4 |
/// | `branch_max_len` | 5 |
/// | `branch_prefix_len` | 1 |
/// | `branch_suffix_len` | 4 |
/// | `separator` | `":"` |
/// | `show_branch` | `true` |
#[derive(Debug, Clone)]
pub struct FormatterConfig {
    /// Maximum length for the folder name display.
    pub folder_max_len: usize,
    /// Number of characters to preserve at the start when truncating folders.
    pub folder_prefix_len: usize,
    /// Number of characters to preserve at the end when truncating folders.
    pub folder_suffix_len: usize,
    /// Maximum length for the branch name display.
    pub branch_max_len: usize,
    /// Number of characters to preserve at the start when truncating branches.
    pub branch_prefix_len: usize,
    /// Number of characters to preserve at the end when truncating branches.
    pub branch_suffix_len: usize,
    /// String placed between folder and branch names (e.g., `":"`).
    pub separator: String,
    /// Whether to include the git branch in the tab name.
    pub show_branch: bool,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            folder_max_len: 10,
            folder_prefix_len: 5,
            folder_suffix_len: 4,
            branch_max_len: 5,
            branch_prefix_len: 1,
            branch_suffix_len: 4,
            separator: ":".to_string(),
            show_branch: true,
        }
    }
}

impl FormatterConfig {
    /// Creates a configuration from Zellij plugin settings.
    ///
    /// Parses the plugin configuration map and applies any specified overrides
    /// to the default values. Invalid or unparseable values are silently ignored,
    /// preserving the defaults.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration map from Zellij's plugin loading
    ///
    /// # Supported Keys
    ///
    /// - `folder_max_len` - Maximum folder name length (usize)
    /// - `folder_prefix_len` - Folder truncation prefix length (usize)
    /// - `folder_suffix_len` - Folder truncation suffix length (usize)
    /// - `branch_max_len` - Maximum branch name length (usize)
    /// - `branch_prefix_len` - Branch truncation prefix length (usize)
    /// - `branch_suffix_len` - Branch truncation suffix length (usize)
    /// - `separator` - String between folder and branch
    /// - `show_branch` - `"false"` to hide branch, any other value shows it
    pub fn from_config(config: &BTreeMap<String, String>) -> Self {
        let mut result = Self::default();

        if let Some(v) = config.get("folder_max_len").and_then(|s| s.parse().ok()) {
            result.folder_max_len = v;
        }
        if let Some(v) = config.get("folder_prefix_len").and_then(|s| s.parse().ok()) {
            result.folder_prefix_len = v;
        }
        if let Some(v) = config.get("folder_suffix_len").and_then(|s| s.parse().ok()) {
            result.folder_suffix_len = v;
        }
        if let Some(v) = config.get("branch_max_len").and_then(|s| s.parse().ok()) {
            result.branch_max_len = v;
        }
        if let Some(v) = config.get("branch_prefix_len").and_then(|s| s.parse().ok()) {
            result.branch_prefix_len = v;
        }
        if let Some(v) = config.get("branch_suffix_len").and_then(|s| s.parse().ok()) {
            result.branch_suffix_len = v;
        }
        if let Some(v) = config.get("separator") {
            result.separator = v.clone();
        }
        if let Some(v) = config.get("show_branch") {
            result.show_branch = v != "false";
        }

        result
    }
}

/// Truncates a string using a prefix + ellipsis + suffix strategy.
///
/// If the string fits within `max_len`, it is returned unchanged. Otherwise,
/// the string is truncated by keeping `prefix_len` characters from the start
/// and `suffix_len` characters from the end, joined by an ellipsis (`…`).
///
/// # Arguments
///
/// * `s` - The string to truncate
/// * `max_len` - Maximum allowed length
/// * `prefix_len` - Characters to keep from the beginning
/// * `suffix_len` - Characters to keep from the end
///
/// # Returns
///
/// The original string if it fits, or a truncated version with an ellipsis.
fn truncate(s: &str, max_len: usize, prefix_len: usize, suffix_len: usize) -> String {
    let char_count = s.chars().count();

    if char_count <= max_len {
        return s.to_string();
    }

    // If no prefix/suffix specified, just take first max_len chars
    if prefix_len == 0 && suffix_len == 0 {
        return s.chars().take(max_len).collect();
    }

    // Ensure we have room for ellipsis
    let ellipsis = '…';
    let needed = prefix_len + 1 + suffix_len; // prefix + ellipsis + suffix

    if needed > max_len || prefix_len + suffix_len >= char_count {
        // Just take what we can
        return s.chars().take(max_len).collect();
    }

    let prefix: String = s.chars().take(prefix_len).collect();
    let suffix: String = s.chars().skip(char_count - suffix_len).collect();

    format!("{}{}{}", prefix, ellipsis, suffix)
}

/// Formats a tab name from a folder name and optional git branch.
///
/// Applies truncation rules from the configuration to both the folder and
/// branch names, then combines them with the configured separator.
///
/// # Arguments
///
/// * `folder` - The folder name (typically the last component of the CWD)
/// * `branch` - The current git branch, or `None` if not in a git repository
/// * `config` - Formatting configuration
///
/// # Returns
///
/// The formatted tab name. If `show_branch` is `false` or `branch` is `None`,
/// only the folder name is returned.
///
/// # Examples
///
/// ```
/// use namey::formatter::{format_tab_name, FormatterConfig};
///
/// let config = FormatterConfig::default();
/// assert_eq!(format_tab_name("myproject", Some("main"), &config), "myproject:main");
/// assert_eq!(format_tab_name("myproject", None, &config), "myproject");
/// ```
pub fn format_tab_name(folder: &str, branch: Option<&str>, config: &FormatterConfig) -> String {
    let folder_display = truncate(
        folder,
        config.folder_max_len,
        config.folder_prefix_len,
        config.folder_suffix_len,
    );

    match (branch, config.show_branch) {
        (Some(branch), true) => {
            let branch_display = truncate(
                branch,
                config.branch_max_len,
                config.branch_prefix_len,
                config.branch_suffix_len,
            );
            format!("{}{}{}", folder_display, config.separator, branch_display)
        }
        _ => folder_display,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== FormatterConfig Tests ====================

    #[test]
    fn test_default_config() {
        let config = FormatterConfig::default();
        assert_eq!(config.folder_max_len, 10);
        assert_eq!(config.folder_prefix_len, 5);
        assert_eq!(config.folder_suffix_len, 4);
        assert_eq!(config.branch_max_len, 5);
        assert_eq!(config.branch_prefix_len, 1);
        assert_eq!(config.branch_suffix_len, 4);
        assert_eq!(config.separator, ":");
        assert!(config.show_branch);
    }

    #[test]
    fn test_from_config_empty() {
        let map = BTreeMap::new();
        let config = FormatterConfig::from_config(&map);
        assert_eq!(config.folder_max_len, 10); // defaults preserved
    }

    #[test]
    fn test_from_config_all_values() {
        let map = BTreeMap::from([
            ("folder_max_len".to_string(), "20".to_string()),
            ("folder_prefix_len".to_string(), "8".to_string()),
            ("folder_suffix_len".to_string(), "6".to_string()),
            ("branch_max_len".to_string(), "10".to_string()),
            ("branch_prefix_len".to_string(), "3".to_string()),
            ("branch_suffix_len".to_string(), "5".to_string()),
            ("separator".to_string(), " | ".to_string()),
            ("show_branch".to_string(), "false".to_string()),
        ]);
        let config = FormatterConfig::from_config(&map);
        assert_eq!(config.folder_max_len, 20);
        assert_eq!(config.folder_prefix_len, 8);
        assert_eq!(config.folder_suffix_len, 6);
        assert_eq!(config.branch_max_len, 10);
        assert_eq!(config.branch_prefix_len, 3);
        assert_eq!(config.branch_suffix_len, 5);
        assert_eq!(config.separator, " | ");
        assert!(!config.show_branch);
    }

    #[test]
    fn test_from_config_invalid_numbers_ignored() {
        let map = BTreeMap::from([
            ("folder_max_len".to_string(), "not_a_number".to_string()),
            ("branch_max_len".to_string(), "-5".to_string()), // negative, invalid for usize
        ]);
        let config = FormatterConfig::from_config(&map);
        assert_eq!(config.folder_max_len, 10); // default preserved
        assert_eq!(config.branch_max_len, 5); // default preserved
    }

    #[test]
    fn test_from_config_show_branch_truthy() {
        // Any value except "false" should be true
        let map = BTreeMap::from([("show_branch".to_string(), "true".to_string())]);
        assert!(FormatterConfig::from_config(&map).show_branch);

        let map = BTreeMap::from([("show_branch".to_string(), "yes".to_string())]);
        assert!(FormatterConfig::from_config(&map).show_branch);

        let map = BTreeMap::from([("show_branch".to_string(), "1".to_string())]);
        assert!(FormatterConfig::from_config(&map).show_branch);

        let map = BTreeMap::from([("show_branch".to_string(), "false".to_string())]);
        assert!(!FormatterConfig::from_config(&map).show_branch);
    }

    // ==================== truncate() Tests ====================

    #[test]
    fn test_truncate_empty_string() {
        assert_eq!(truncate("", 10, 5, 4), "");
    }

    #[test]
    fn test_truncate_short() {
        assert_eq!(truncate("hello", 10, 5, 4), "hello");
    }

    #[test]
    fn test_truncate_exact_length() {
        assert_eq!(truncate("helloworld", 10, 5, 4), "helloworld");
    }

    #[test]
    fn test_truncate_one_over() {
        assert_eq!(truncate("helloworld!", 10, 5, 4), "hello…rld!");
    }

    #[test]
    fn test_truncate_long_folder() {
        assert_eq!(truncate("my_project_name", 10, 5, 4), "my_pr…name");
    }

    #[test]
    fn test_truncate_prefix_suffix_exceeds_length() {
        // prefix(5) + suffix(4) = 9 >= char_count(8), so just take first max_len chars
        assert_eq!(truncate("abcdefgh", 10, 5, 4), "abcdefgh");
    }

    #[test]
    fn test_truncate_needed_exceeds_max() {
        // needed = 1 + 1 + 4 = 6 > max_len(5), so just take first 5 chars
        assert_eq!(truncate("feature-branch", 5, 1, 4), "featu");
    }

    #[test]
    fn test_truncate_unicode() {
        // Unicode characters should be handled correctly
        assert_eq!(truncate("héllo", 10, 5, 4), "héllo");
        assert_eq!(truncate("日本語テスト文字列", 6, 2, 2), "日本…字列");
    }

    #[test]
    fn test_truncate_zero_max_len() {
        assert_eq!(truncate("hello", 0, 0, 0), "");
    }

    #[test]
    fn test_truncate_single_char_result() {
        assert_eq!(truncate("hello", 1, 0, 0), "h");
    }

    // ==================== format_tab_name() Tests ====================

    #[test]
    fn test_format_tab_name_with_branch() {
        let config = FormatterConfig::default();
        assert_eq!(
            format_tab_name("myproject", Some("main"), &config),
            "myproject:main"
        );
    }

    #[test]
    fn test_format_tab_name_no_branch() {
        let config = FormatterConfig::default();
        assert_eq!(format_tab_name("myproject", None, &config), "myproject");
    }

    #[test]
    fn test_format_tab_name_truncated_folder() {
        let config = FormatterConfig::default();
        assert_eq!(
            format_tab_name("my_long_project_name", Some("main"), &config),
            "my_lo…name:main"
        );
    }

    #[test]
    fn test_format_tab_name_truncated_branch() {
        let config = FormatterConfig::default();
        // branch "feature" is 7 chars, max is 5, prefix 1 + ellipsis + suffix 4 = 6 > 5
        // so it just takes first 5 chars
        assert_eq!(
            format_tab_name("src", Some("feature"), &config),
            "src:featu"
        );
    }

    #[test]
    fn test_format_tab_name_both_truncated() {
        let config = FormatterConfig::default();
        let result = format_tab_name("my_long_project_name", Some("feature-branch-name"), &config);
        assert_eq!(result, "my_lo…name:featu");
    }

    #[test]
    fn test_format_tab_name_branch_disabled() {
        let mut config = FormatterConfig::default();
        config.show_branch = false;
        assert_eq!(
            format_tab_name("myproject", Some("main"), &config),
            "myproject"
        );
    }

    #[test]
    fn test_format_tab_name_custom_separator() {
        let mut config = FormatterConfig::default();
        config.separator = " @ ".to_string();
        assert_eq!(
            format_tab_name("myproject", Some("main"), &config),
            "myproject @ main"
        );
    }

    #[test]
    fn test_format_tab_name_empty_folder() {
        let config = FormatterConfig::default();
        assert_eq!(format_tab_name("", Some("main"), &config), ":main");
    }

    #[test]
    fn test_format_tab_name_empty_branch() {
        let config = FormatterConfig::default();
        // Empty string branch is still Some, so it shows separator
        assert_eq!(format_tab_name("src", Some(""), &config), "src:");
    }
}
