use std::fs;
use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result, Worktree};

struct XMakeExtension {
    cached_binary_path: Option<String>,
}

impl XMakeExtension {
    fn get_linux_variant(&self, worktree: &Worktree) -> Result<String> {
        let settings = LspSettings::for_worktree("xmake-ls", worktree).ok();

        if let Some(settings) = settings {
            if let Some(settings_value) = settings.settings {
                if let Some(variant) = settings_value.get("linuxVariant") {
                    if let Some(variant_str) = variant.as_str() {
                        return Ok(variant_str.to_string());
                    }
                }
            }
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

    fn get_settings(&self, worktree: &Worktree) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree("xmake-ls", worktree).ok();
        Ok(settings.and_then(|s| s.settings))
    }
}

impl zed::Extension for XMakeExtension {
    fn new() -> Self {
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
            return Ok(zed::Command {
                command: path,
                args: vec![],
                env: Default::default(),
            });
        }

        if let Ok(path) = which::which("xmake_ls") {
            let path_str = path.to_string_lossy().to_string();
            self.cached_binary_path = Some(path_str.clone());
            return Ok(zed::Command {
                command: path_str,
                args: vec![],
                env: Default::default(),
            });
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(zed::Command {
                    command: path.clone(),
                    args: vec![],
                    env: Default::default(),
                });
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
        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = self.get_settings(worktree)?;

        let init_options = zed::serde_json::json!({
            "settings": settings.clone().unwrap_or_else(|| zed::serde_json::json!({}))
        });

        Ok(Some(init_options))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = self.get_settings(worktree)?;
        Ok(settings)
    }
}

zed::register_extension!(XMakeExtension);
