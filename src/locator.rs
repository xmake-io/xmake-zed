use zed_extension_api::{
    self as zed, BuildTaskDefinition, BuildTaskDefinitionTemplatePayload, DebugRequest,
    DebugScenario, LaunchRequest, TaskTemplate, process::Command as ProcessCommand,
};

use crate::invocation::{self, DebugPlan};

const DEBUG_TARGET_ENV: &str = "XMAKE_DEBUG_TARGET";
const DEBUG_ARGS_ENV: &str = "XMAKE_DEBUG_ARGS";
const DEBUG_CWD_ENV: &str = "XMAKE_DEBUG_CWD";
const DEBUG_PROJECT_ENV: &str = "XMAKE_DEBUG_PROJECT_DIR";
const PROGRAM_MARK: &str = "XMAKE_DEBUG_PROGRAM=";
const RUNDIR_MARK: &str = "XMAKE_DEBUG_RUNDIR=";

/// `xmake lua -c` script. See `resources/find-binary.lua`.
/// The text is one argv element, so it must not start with `-`.
const FIND_BINARY_LUA: &str = include_str!("../resources/find-binary.lua");

pub fn dap_locator_create_scenario(
    locator_name: String,
    build_task: TaskTemplate,
    resolved_label: String,
    debug_adapter_name: String,
) -> Option<DebugScenario> {
    if !is_xmake_command(&build_task.command) {
        return None;
    }
    let plan = invocation::debug_plan(&build_task.args)?;
    let template = template_for_plan(build_task, &plan);

    Some(DebugScenario {
        label: resolved_label,
        adapter: debug_adapter_name.clone(),
        build: Some(BuildTaskDefinition::Template(
            BuildTaskDefinitionTemplatePayload {
                locator_name: Some(locator_name),
                template,
            },
        )),
        // No `request` field: Zed treats that as "ask the locator".
        // CodeLLDB enables language support from sourceLanguages.
        config: adapter_config(&debug_adapter_name),
        tcp_connection: None,
    })
}

pub fn run_dap_locator(build_task: TaskTemplate) -> Result<DebugRequest, String> {
    let xmake = if build_task.command.is_empty() {
        "xmake".to_string()
    } else {
        build_task.command.clone()
    };
    let project = build_task
        .cwd
        .clone()
        .filter(|cwd| !cwd.is_empty())
        .ok_or_else(|| "xmake debug locator needs the build task working directory".to_string())?;

    let target = env_value(&build_task.env, DEBUG_TARGET_ENV);
    let program_args = parse_program_args(env_value(&build_task.env, DEBUG_ARGS_ENV))?;
    let explicit_cwd = env_value(&build_task.env, DEBUG_CWD_ENV);

    let mut cmd = ProcessCommand::new(xmake);
    cmd = cmd.envs(debuggee_envs(&build_task.env));
    cmd = cmd.env(DEBUG_PROJECT_ENV, &project);
    if let Some(target) = target.as_deref() {
        cmd = cmd.env(DEBUG_TARGET_ENV, target);
    }
    cmd = cmd.args(["lua", "-c", FIND_BINARY_LUA.trim()]);

    let output = cmd
        .output()
        .map_err(|err| format!("failed to run xmake to locate debug target: {err}"))?;
    if output.status != Some(0) {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "xmake failed while locating the debug target (status {:?}): {stderr}",
            output.status
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let relative = marked_line(&stdout, PROGRAM_MARK).ok_or_else(|| {
        "xmake did not report a binary that exists on disk. Build a binary target first, or name one with `xmake run <target>`."
            .to_string()
    })?;
    let program = resolve_program(relative, Some(&project));
    // `xmake run -w` wins. Otherwise use target:rundir() (set_rundir, or the binary's directory).
    let debuggee_cwd = explicit_cwd
        .map(|cwd| resolve_debuggee_cwd(&cwd, &project))
        .or_else(|| {
            marked_line(&stdout, RUNDIR_MARK).map(|dir| resolve_debuggee_cwd(dir, &project))
        })
        .unwrap_or_else(|| project.clone());

    Ok(DebugRequest::Launch(LaunchRequest {
        program,
        cwd: Some(debuggee_cwd),
        args: program_args,
        envs: debuggee_envs(&build_task.env),
    }))
}

fn template_for_plan(mut template: TaskTemplate, plan: &DebugPlan) -> TaskTemplate {
    if let Some(args) = &plan.build_args {
        template.args = args.clone();
    }
    if !plan.targets.is_empty() {
        upsert_env(
            &mut template.env,
            DEBUG_TARGET_ENV,
            &plan.targets.join("\n"),
        );
    }
    if !plan.program_args.is_empty()
        && let Ok(encoded) = zed::serde_json::to_string(&plan.program_args)
    {
        upsert_env(&mut template.env, DEBUG_ARGS_ENV, &encoded);
    }
    if let Some(cwd) = &plan.debuggee_cwd {
        upsert_env(&mut template.env, DEBUG_CWD_ENV, cwd);
    }
    template
}

fn adapter_config(adapter: &str) -> String {
    // `current_platform` is a host import. Unit tests pass the OS in directly.
    let on_mac = !cfg!(test) && matches!(zed::current_platform().0, zed::Os::Mac);
    adapter_config_for(adapter, on_mac)
}

fn adapter_config_for(adapter: &str, on_mac: bool) -> String {
    if adapter != "CodeLLDB" {
        return "{}".to_string();
    }
    let mut config = zed::serde_json::json!({
        "sourceLanguages": ["c", "cpp", "rust"]
    });
    // macOS opens /tmp and /var as /private/tmp and /private/var. A binary built
    // through the other name keeps that path in debug info, so breakpoints miss.
    if on_mac {
        config["sourceMap"] = zed::serde_json::json!({
            "/tmp": "/private/tmp",
            "/var": "/private/var"
        });
    }
    config.to_string()
}

fn resolve_debuggee_cwd(cwd: &str, project: &str) -> String {
    let cwd = cwd.trim();
    if cwd.is_empty() || cwd == "." {
        project.to_string()
    } else {
        resolve_program(cwd, Some(project))
    }
}

fn marked_line<'a>(stdout: &'a str, mark: &str) -> Option<&'a str> {
    stdout.lines().map(str::trim).find_map(|line| {
        line.strip_prefix(mark)
            .map(str::trim)
            .filter(|value| !value.is_empty())
    })
}

pub(crate) fn debuggee_envs(env: &[(String, String)]) -> Vec<(String, String)> {
    env.iter()
        .filter(|(key, _)| !is_locator_env(key))
        .cloned()
        .collect()
}

fn is_locator_env(key: &str) -> bool {
    matches!(
        key,
        DEBUG_TARGET_ENV | DEBUG_ARGS_ENV | DEBUG_CWD_ENV | DEBUG_PROJECT_ENV
    )
}

pub fn is_xmake_command(command: &str) -> bool {
    let name = command
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(command)
        .trim_matches('"');
    name.eq_ignore_ascii_case("xmake")
        || name.eq_ignore_ascii_case("xmake.exe")
        || name.eq_ignore_ascii_case("xmake.cmd")
        || name.eq_ignore_ascii_case("xmake.bat")
}

fn upsert_env(env: &mut Vec<(String, String)>, key: &str, value: &str) {
    if let Some(existing) = env.iter_mut().find(|(k, _)| k == key) {
        existing.1 = value.to_string();
    } else {
        env.push((key.to_string(), value.to_string()));
    }
}

fn env_value(env: &[(String, String)], key: &str) -> Option<String> {
    env.iter()
        .find(|(k, _)| k == key)
        .map(|(_, value)| value.clone())
        .filter(|value| !value.is_empty())
}

fn parse_program_args(encoded: Option<String>) -> Result<Vec<String>, String> {
    let Some(encoded) = encoded else {
        return Ok(Vec::new());
    };
    zed::serde_json::from_str(&encoded)
        .map_err(|_| "xmake debug locator could not read the program arguments".to_string())
}

pub fn resolve_program(path: &str, cwd: Option<&str>) -> String {
    let path = path.trim();
    if path.is_empty() {
        return path.to_string();
    }
    let absolute =
        path.starts_with('/') || path.starts_with('\\') || path.chars().nth(1) == Some(':');
    let path = path.replace('\\', "/");
    if absolute {
        return path;
    }
    match cwd {
        Some(cwd) if !cwd.is_empty() => {
            let cwd = cwd.trim_end_matches(['/', '\\']).replace('\\', "/");
            format!("{cwd}/{path}")
        }
        _ => path,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task(args: Vec<String>) -> TaskTemplate {
        TaskTemplate {
            label: "xmake".into(),
            command: "xmake".into(),
            args,
            env: Vec::new(),
            cwd: Some("/proj".into()),
        }
    }

    fn scenario(args: Vec<String>) -> DebugScenario {
        dap_locator_create_scenario(
            "xmake".into(),
            task(args),
            "label".into(),
            "CodeLLDB".into(),
        )
        .expect("debuggable")
    }

    fn build_args(scenario: &DebugScenario) -> Vec<String> {
        let BuildTaskDefinition::Template(payload) = scenario.build.clone().expect("build") else {
            panic!("expected template build");
        };
        payload.template.args
    }

    #[test]
    fn detects_xmake_binaries() {
        assert!(is_xmake_command("xmake"));
        assert!(is_xmake_command("/usr/bin/xmake"));
        assert!(is_xmake_command(r"C:\tools\xmake.exe"));
        assert!(is_xmake_command(r"C:\tools\xmake.cmd"));
        assert!(is_xmake_command("XMAKE.BAT"));
        assert!(!is_xmake_command("cmake"));
        assert!(!is_xmake_command("xmake-check"));
    }

    #[test]
    fn converts_run_into_build_without_leaking_into_the_binary_name() {
        let scenario = scenario(vec!["run".into(), "demo".into(), "--verbose".into()]);
        assert_eq!(build_args(&scenario), vec!["build", "demo"]);
        assert!(scenario.config.contains("\"rust\""));
        let BuildTaskDefinition::Template(payload) = scenario.build.unwrap() else {
            panic!("template");
        };
        assert!(
            payload
                .template
                .env
                .iter()
                .any(|(key, value)| { key == DEBUG_TARGET_ENV && value == "demo" })
        );
        assert!(
            payload
                .template
                .env
                .iter()
                .any(|(key, value)| { key == DEBUG_ARGS_ENV && value.contains("--verbose") })
        );
    }

    #[test]
    fn macos_codelldb_remaps_private_tmp() {
        let config = adapter_config_for("CodeLLDB", true);
        assert!(config.contains("\"/tmp\":\"/private/tmp\""));
        assert!(config.contains("\"/var\":\"/private/var\""));
        assert!(!adapter_config_for("CodeLLDB", false).contains("sourceMap"));
        assert_eq!(adapter_config_for("GDB", true), "{}");
    }

    #[test]
    fn debug_helper_keeps_its_build_and_does_not_target_lua() {
        let scenario = scenario(vec![
            "lua".into(),
            "-c".into(),
            crate::invocation::DEBUG_BUILD_SNIPPET.into(),
        ]);
        let args = build_args(&scenario);
        assert_eq!(args[0], "lua");
        let BuildTaskDefinition::Template(payload) = scenario.build.unwrap() else {
            panic!("template");
        };
        assert!(
            payload
                .template
                .env
                .iter()
                .all(|(key, _)| key != DEBUG_TARGET_ENV)
        );
    }

    #[test]
    fn skips_tasks_that_do_not_build_a_binary() {
        for args in [
            vec!["config".into(), "-m".into(), "debug".into()],
            vec!["-h".into()],
            vec!["--version".into()],
            vec![
                "lua".into(),
                "-c".into(),
                crate::invocation::TERMINAL_DEBUG_SNIPPET.into(),
            ],
            vec![
                "lua".into(),
                "-c".into(),
                "os.execv('xmake', {'clean', '-a'}); os.execv('xmake', {})".into(),
            ],
            vec![
                "lua".into(),
                "-c".into(),
                "os.execv('xmake', {'project', '-k', 'compile_commands', '.zed'})".into(),
            ],
        ] {
            assert!(
                dap_locator_create_scenario(
                    "xmake".into(),
                    task(args),
                    "label".into(),
                    "CodeLLDB".into(),
                )
                .is_none()
            );
        }
    }

    #[test]
    fn locate_script_retargets_the_xmake_project() {
        let chdir = FIND_BINARY_LUA
            .find("project.chdir(projectdir)")
            .expect("project.chdir");
        let cd = FIND_BINARY_LUA.find("os.cd(projectdir)").expect("os.cd");
        let load = FIND_BINARY_LUA.find("config.load()").expect("config.load");
        assert!(chdir < cd && cd < load);
        assert!(!FIND_BINARY_LUA.trim_start().starts_with('-'));
        assert!(FIND_BINARY_LUA.contains(PROGRAM_MARK));
        assert!(FIND_BINARY_LUA.contains("os.isfile(file)"));
        assert!(FIND_BINARY_LUA.contains("not fallback and existing_file(t)"));
        assert!(FIND_BINARY_LUA.contains("raise("));
        assert!(FIND_BINARY_LUA.contains(RUNDIR_MARK));
        assert!(!FIND_BINARY_LUA.contains("'-P'") && !FIND_BINARY_LUA.contains("\"-P\""));
    }

    #[test]
    fn reads_sentinels_and_ignores_xmake_noise() {
        let stdout =
            "checking for gcc ... ok\nXMAKE_DEBUG_PROGRAM=build/app\nXMAKE_DEBUG_RUNDIR=/proj\n";
        assert_eq!(marked_line(stdout, PROGRAM_MARK), Some("build/app"));
        assert_eq!(marked_line(stdout, RUNDIR_MARK), Some("/proj"));
        assert_eq!(marked_line("checking for gcc ... ok\n", PROGRAM_MARK), None);
    }

    #[test]
    fn strips_locator_variables_from_the_debuggee() {
        let env = vec![
            ("PATH".into(), "/usr/bin".into()),
            (DEBUG_TARGET_ENV.into(), "app".into()),
            (DEBUG_ARGS_ENV.into(), "[\"--verbose\"]".into()),
            (DEBUG_PROJECT_ENV.into(), "/proj".into()),
        ];
        assert_eq!(
            debuggee_envs(&env),
            vec![("PATH".into(), "/usr/bin".into())]
        );
    }

    #[test]
    fn joins_relative_target_with_cwd() {
        assert_eq!(
            resolve_program("build/macosx/arm64/debug/app", Some("/proj")),
            "/proj/build/macosx/arm64/debug/app"
        );
        assert_eq!(resolve_program("/abs/app", Some("/proj")), "/abs/app");
        assert_eq!(
            resolve_program(r"build\macosx\app", Some(r"C:\proj")),
            "C:/proj/build/macosx/app"
        );
        assert_eq!(resolve_debuggee_cwd(".", "/proj"), "/proj");
        assert_eq!(resolve_debuggee_cwd("/tmp", "/proj"), "/tmp");
        assert_eq!(
            marked_line("XMAKE_DEBUG_PROGRAM=/tmp/my app/bin\n", PROGRAM_MARK),
            Some("/tmp/my app/bin")
        );
    }

    #[test]
    fn corrupt_program_args_are_an_error() {
        let mut broken = task(vec![]);
        broken.env.push((DEBUG_ARGS_ENV.into(), "not-json".into()));
        let err = run_dap_locator(broken).unwrap_err();
        assert!(err.contains("program arguments"));
    }

    #[test]
    fn several_build_targets_are_passed_in_order() {
        let scenario = scenario(vec!["build".into(), "lib".into(), "app".into()]);
        let BuildTaskDefinition::Template(payload) = scenario.build.unwrap() else {
            panic!("template");
        };
        assert!(
            payload
                .template
                .env
                .iter()
                .any(|(key, value)| key == DEBUG_TARGET_ENV && value == "lib\napp")
        );
    }
}
