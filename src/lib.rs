use std::fs;
use zed_extension_api::{
    self as zed, DebugRequest, DebugScenario, LanguageServerId, Result, TaskTemplate, Worktree,
    process::Command as ProcessCommand,
    serde_json::{self, Value},
    settings::LspSettings,
};

mod invocation;
mod locator;

struct XMakeExtension {
    cached_binary_path: Option<String>,
}

const GLOBAL_TASKS_TEMPLATE: &str = include_str!("../resources/install-global-tasks.lua");
const SHIPPED_TASKS: &str = include_str!("../languages/xmake/tasks.json");

fn global_tasks_script() -> String {
    GLOBAL_TASKS_TEMPLATE.replace("@TASKS@", SHIPPED_TASKS.trim())
}

/// Copy the shipped tasks into Zed's global tasks.json so they show up from
/// any file, for anyone who installed the extension. xmake tasks are replaced;
/// every other task in that file is left in place.
fn install_global_tasks() {
    let script = global_tasks_script();
    if script.trim_start().starts_with('-') {
        return;
    }
    let _ = ProcessCommand::new("xmake")
        .arg("lua")
        .arg("-c")
        .arg(script.trim())
        .output();
}

impl XMakeExtension {
    fn linux_variant_setting(settings: Option<&Value>) -> Option<String> {
        let settings = settings?;
        let raw = settings
            .get("linuxVariant")
            .or_else(|| settings.get("linux_variant"))?
            .as_str()?
            .trim();
        if raw.is_empty() || raw.eq_ignore_ascii_case("auto") {
            None
        } else {
            Some(raw.to_string())
        }
    }

    fn get_linux_variant(&self, worktree: &Worktree) -> Result<String> {
        let settings = LspSettings::for_worktree("xmake-ls", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings);
        if let Some(variant) = Self::linux_variant_setting(settings.as_ref()) {
            return Ok(variant);
        }

        let (_, arch) = zed::current_platform();
        match arch {
            zed::Architecture::Aarch64 => Ok("aarch64-glibc.2.17".to_string()),
            zed::Architecture::X8664 => Ok("x64-glibc.2.17".to_string()),
            zed::Architecture::X86 => {
                Err("32-bit x86 Linux is not supported by xmake_ls".to_string())
            }
        }
    }

    fn get_settings(&self, worktree: &Worktree) -> Result<Option<Value>> {
        let settings = LspSettings::for_worktree("xmake-ls", worktree).ok();
        Ok(settings.and_then(|s| s.settings))
    }

    fn system_xmake_ls(worktree: &Worktree) -> Option<String> {
        let (os, _) = zed::current_platform();
        let names: &[&str] = if os == zed::Os::Windows {
            &["xmake_ls.exe", "xmake_ls"]
        } else {
            &["xmake_ls"]
        };
        names.iter().find_map(|name| worktree.which(name))
    }

    fn lsp_command(command: String, worktree: &Worktree) -> zed::Command {
        zed::Command {
            command,
            args: vec![],
            env: worktree.shell_env(),
        }
    }
}

impl zed::Extension for XMakeExtension {
    fn new() -> Self {
        install_global_tasks();
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let settings = LspSettings::for_worktree("xmake-ls", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary)
            .and_then(|binary_settings| binary_settings.path);

        if let Some(path) = settings {
            return Ok(Self::lsp_command(path, worktree));
        }

        if let Some(path) = Self::system_xmake_ls(worktree) {
            self.cached_binary_path = Some(path.clone());
            return Ok(Self::lsp_command(path, worktree));
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(Self::lsp_command(path.clone(), worktree));
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "CppCXY/xmake_ls",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();

        let (asset_name, file_type, binary_name) = match platform {
            zed::Os::Mac => {
                let arch_str = match arch {
                    zed::Architecture::Aarch64 => "arm64",
                    zed::Architecture::X8664 => "x64",
                    zed::Architecture::X86 => {
                        return Err("32-bit macOS is not supported by xmake_ls".to_string());
                    }
                };
                (
                    format!("xmake_ls-darwin-{}.tar.gz", arch_str),
                    zed::DownloadedFileType::GzipTar,
                    "xmake_ls".to_string(),
                )
            }
            zed::Os::Linux => {
                let variant = self.get_linux_variant(worktree)?;
                (
                    format!("xmake_ls-linux-{}.tar.gz", variant),
                    zed::DownloadedFileType::GzipTar,
                    "xmake_ls".to_string(),
                )
            }
            zed::Os::Windows => {
                let arch_str = match arch {
                    zed::Architecture::Aarch64 => "arm64",
                    zed::Architecture::X8664 => "x64",
                    zed::Architecture::X86 => "ia32",
                };
                (
                    format!("xmake_ls-win32-{}.zip", arch_str),
                    zed::DownloadedFileType::Zip,
                    "xmake_ls.exe".to_string(),
                )
            }
        };

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| {
                format!(
                    "no asset found matching {:?}. Available: {:?}",
                    asset_name,
                    release.assets.iter().map(|a| &a.name).collect::<Vec<_>>()
                )
            })?;

        let version_dir = format!("xmake_ls-{}", release.version);
        let binary_path = format!("{version_dir}/{binary_name}");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(&asset.download_url, &version_dir, file_type)
                .map_err(|e| format!("failed to download file: {e}"))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }

            if platform != zed::Os::Windows {
                zed::make_file_executable(&binary_path)?;
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(Self::lsp_command(binary_path, worktree))
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        let settings = self.get_settings(worktree)?;

        let init_options = serde_json::json!({
            "settings": settings.clone().unwrap_or_else(|| serde_json::json!({}))
        });

        Ok(Some(init_options))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        let settings = self.get_settings(worktree)?;
        Ok(settings)
    }

    fn dap_locator_create_scenario(
        &mut self,
        locator_name: String,
        build_task: TaskTemplate,
        resolved_label: String,
        debug_adapter_name: String,
    ) -> Option<DebugScenario> {
        locator::dap_locator_create_scenario(
            locator_name,
            build_task,
            resolved_label,
            debug_adapter_name,
        )
    }

    fn run_dap_locator(
        &mut self,
        _locator_name: String,
        build_task: TaskTemplate,
    ) -> Result<DebugRequest> {
        locator::run_dap_locator(build_task)
    }
}

zed::register_extension!(XMakeExtension);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_linux_variant_falls_back_to_arch() {
        let auto = serde_json::json!({ "linuxVariant": "auto" });
        assert_eq!(XMakeExtension::linux_variant_setting(Some(&auto)), None);

        let empty = serde_json::json!({ "linuxVariant": "" });
        assert_eq!(XMakeExtension::linux_variant_setting(Some(&empty)), None);

        let snake = serde_json::json!({ "linux_variant": "musl" });
        assert_eq!(
            XMakeExtension::linux_variant_setting(Some(&snake)).as_deref(),
            Some("musl")
        );

        let camel = serde_json::json!({ "linuxVariant": "x64" });
        assert_eq!(
            XMakeExtension::linux_variant_setting(Some(&camel)).as_deref(),
            Some("x64")
        );
    }

    #[test]
    fn workspace_install_embeds_the_templates() {
        let tasks: Vec<serde_json::Value> =
            serde_json::from_str(include_str!("../languages/xmake/tasks.json")).unwrap();
        let install = tasks
            .iter()
            .find(|task| task["label"] == "xmake: install workspace tasks")
            .unwrap();
        let script = install["args"][2].as_str().unwrap();
        let workspace_tasks = include_str!("../resources/workspace-tasks.json").trim();
        let workspace_debug = include_str!("../resources/workspace-debug.json").trim();
        assert!(script.contains(workspace_tasks));
        assert!(script.contains(workspace_debug));
        // Zed expands `$ZED_WORKTREE_ROOT` in task args before xmake runs.
        // The installer must assemble that name itself.
        assert!(!script.contains("$ZED_WORKTREE_ROOT"));
        assert!(script.contains("'$' .. 'ZED_WORKTREE_ROOT'"));

        let debug = tasks
            .iter()
            .find(|task| task["label"] == "xmake: debug")
            .unwrap();
        assert_eq!(debug["args"][2], crate::invocation::DEBUG_BUILD_SNIPPET);

        let workspace: Vec<serde_json::Value> = serde_json::from_str(workspace_tasks).unwrap();
        let installed_debug = workspace
            .iter()
            .find(|task| task["label"] == "xmake: debug")
            .unwrap();
        assert_eq!(
            installed_debug["args"][2],
            crate::invocation::DEBUG_BUILD_SNIPPET
        );
        let installed_terminal = workspace
            .iter()
            .find(|task| task["label"] == "xmake: debug in terminal")
            .unwrap();
        assert_eq!(
            installed_terminal["args"][2],
            crate::invocation::TERMINAL_DEBUG_SNIPPET
        );

        let example = include_str!("../debug.json.example").trim();
        let expected = workspace_debug.replace("@CWD@", "$ZED_WORKTREE_ROOT");
        assert_eq!(example, expected.trim());

        assert_eq!(
            include_str!("../resources/zed-tasks.json"),
            include_str!("../languages/xmake/tasks.json")
        );
        let global: Vec<serde_json::Value> =
            serde_json::from_str(include_str!("../resources/zed-tasks.json")).unwrap();
        let global_labels: Vec<_> = global
            .iter()
            .map(|task| task["label"].as_str().unwrap())
            .filter(|label| *label != "xmake: install workspace tasks")
            .collect();
        let workspace_labels: Vec<_> = workspace
            .iter()
            .map(|task| task["label"].as_str().unwrap())
            .collect();
        assert_eq!(global_labels, workspace_labels);
    }

    /// `xmake lua -c <script>` is one argv. xmake parses an argument that
    /// starts with `-` as an option, before it ever runs the script.
    /// Zed also expands `$ZED_*` in task args before xmake starts.
    #[test]
    fn lua_c_scripts_are_safe_to_pass_as_argv() {
        let files = [
            include_str!("../languages/xmake/tasks.json"),
            include_str!("../resources/workspace-tasks.json"),
            include_str!("../resources/workspace-debug.json"),
            include_str!("../debug.json.example"),
        ];
        for file in files {
            let value: serde_json::Value = serde_json::from_str(file).unwrap();
            check_lua_c_scripts(&value);
        }
        let locator = include_str!("../resources/find-binary.lua").trim();
        assert!(!locator.starts_with('-'));
        assert!(!locator.contains("$ZED_"));

        let global_script = global_tasks_script();
        assert!(global_script.trim_start().starts_with("local shipped"));
        assert!(!global_script.trim_start().starts_with('-'));
        assert!(global_script.contains("xmake: create Rust static library"));
        assert!(global_script.contains("xmake: create Objective-C iOS app"));
        // The tasks are wrapped in a [==[ ]==] string. The payload must not close it.
        assert!(!SHIPPED_TASKS.contains("]==]"));
    }

    fn check_lua_c_scripts(value: &serde_json::Value) {
        match value {
            serde_json::Value::Array(items) => {
                for index in 0..items.len().saturating_sub(2) {
                    let command = items[index].as_str();
                    let flag = items[index + 1].as_str();
                    let Some(script) = items[index + 2].as_str() else {
                        continue;
                    };
                    if matches!(command, Some("lua") | Some("l"))
                        && matches!(flag, Some("-c") | Some("--command"))
                    {
                        let body = script.trim();
                        assert!(!body.is_empty(), "empty xmake lua -c script");
                        assert!(
                            !body.starts_with('-'),
                            "xmake treats this -c script as an option: {body}"
                        );
                        assert!(
                            !body.contains("$ZED_"),
                            "Zed would expand $ZED_ in this -c script before xmake runs"
                        );
                    }
                }
                for item in items {
                    check_lua_c_scripts(item);
                }
            }
            serde_json::Value::Object(map) => {
                for item in map.values() {
                    check_lua_c_scripts(item);
                }
            }
            _ => {}
        }
    }
}
