mod context;
mod formatter;

use std::collections::BTreeMap;
use zellij_tile::prelude::*;

use context::PaneContext;
use formatter::{format_tab_name, FormatterConfig};

const GIT_BRANCH_SCRIPT: &str = r#"git -C "$1" rev-parse --abbrev-ref HEAD 2>/dev/null"#;

fn is_our_command(context: &BTreeMap<String, String>) -> bool {
    context.get("source").map(|s| s.as_str()) == Some("namey")
}

fn extract_cwd_from_title(title: &str) -> Option<String> {
    let title = title.trim();
    if title.is_empty() {
        return None;
    }

    // Try ": /path" format
    if let Some(idx) = title.rfind(": ") {
        let after_colon = title[idx + 2..].trim();
        if after_colon.starts_with('/') || after_colon.starts_with('~') {
            return Some(after_colon.to_string());
        }
    }

    // Check if whole title is a path
    if title.starts_with('/') || title.starts_with('~') {
        return Some(title.to_string());
    }

    // Try ":/" format (no space)
    if let Some(idx) = title.rfind(':') {
        let after_colon = title[idx + 1..].trim();
        if after_colon.starts_with('/') || after_colon.starts_with('~') {
            return Some(after_colon.to_string());
        }
    }

    None
}

fn parse_git_branch(stdout: &[u8]) -> Option<String> {
    let output = String::from_utf8_lossy(stdout);
    let branch = output.trim();
    if branch.is_empty() {
        None
    } else {
        Some(branch.to_string())
    }
}

fn build_command_context(path: &str) -> BTreeMap<String, String> {
    BTreeMap::from([
        ("source".to_string(), "namey".to_string()),
        ("path".to_string(), path.to_string()),
    ])
}

#[derive(Default)]
struct State {
    config: FormatterConfig,
    current_cwd: Option<String>,
    current_tab_index: usize,
    current_tab_name: String,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        self.config = FormatterConfig::from_config(&configuration);

        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
            PermissionType::RunCommands,
        ]);

        subscribe(&[
            EventType::TabUpdate,
            EventType::PaneUpdate,
            EventType::RunCommandResult,
            EventType::PermissionRequestResult,
        ]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::PermissionRequestResult(_status) => {}
            Event::TabUpdate(tab_info) => {
                if let Some(active_tab) = tab_info.iter().find(|t| t.active) {
                    self.current_tab_index = active_tab.position;
                    self.current_tab_name = active_tab.name.clone();
                }
            }
            Event::PaneUpdate(pane_manifest) => {
                self.handle_pane_update(pane_manifest);
            }
            Event::RunCommandResult(exit_code, stdout, _stderr, context) => {
                self.handle_command_result(exit_code, stdout, context);
            }
            _ => {}
        }
        false
    }

    fn render(&mut self, _rows: usize, _cols: usize) {}
}

impl State {
    fn handle_pane_update(&mut self, pane_manifest: PaneManifest) {
        let focused_pane = pane_manifest
            .panes
            .values()
            .flatten()
            .find(|p| p.is_focused && !p.is_plugin);

        if let Some(pane) = focused_pane {
            if let Some(cwd) = extract_cwd_from_title(&pane.title) {
                if self.current_cwd.as_ref() != Some(&cwd) {
                    self.current_cwd = Some(cwd.clone());
                    self.request_git_branch(&cwd);
                }
            } else {
                // Use title directly as folder name
                let folder = pane.title.trim();
                if !folder.is_empty() {
                    let new_name = format_tab_name(folder, None, &self.config);
                    if new_name != self.current_tab_name {
                        rename_tab(self.current_tab_index as u32, &new_name);
                    }
                }
            }
        }
    }

    fn request_git_branch(&mut self, path: &str) {
        let context = build_command_context(path);
        run_command(&["bash", "-c", GIT_BRANCH_SCRIPT, "_", path], context);
    }

    fn handle_command_result(
        &mut self,
        exit_code: Option<i32>,
        stdout: Vec<u8>,
        context: BTreeMap<String, String>,
    ) {
        if !is_our_command(&context) {
            return;
        }

        let path = match context.get("path") {
            Some(p) => p.clone(),
            None => return,
        };

        let branch = if exit_code == Some(0) {
            parse_git_branch(&stdout)
        } else {
            None
        };

        let ctx = PaneContext { cwd: path, branch };
        let new_name = format_tab_name(ctx.folder_name(), ctx.branch.as_deref(), &self.config);

        if new_name != self.current_tab_name {
            rename_tab(self.current_tab_index as u32, &new_name);
        }
    }
}
