//! Classify an `xmake` argv as something Zed can debug.
//!
//! xmake's own grammar is the source of truth (xmake 3.x):
//! `run` takes options, then one target, then program arguments.
//! `build` is the default action. `-P` is not used here; the locator
//! changes directory in Lua so the build tree stays the project's.

/// Shell-quoted debug build. Zed concatenates task args without quoting, so
/// the Lua has to already be one single-quoted word. The quotes are not part
/// of the program xmake runs; `lua_source` strips one pair before comparing.
pub(crate) const DEBUG_BUILD_SNIPPET: &str =
    "'os.execv(\"xmake\", {\"f\", \"-m\", \"debug\", \"-y\"}); os.execv(\"xmake\", {})'";

#[cfg(test)]
pub(crate) const TERMINAL_DEBUG_SNIPPET: &str = "'os.execv(\"xmake\", {\"f\", \"-m\", \"debug\", \"-y\"}); os.execv(\"xmake\", {}); os.execv(\"xmake\", {\"run\", \"-d\"})'";

/// What the first positional word is, including xmake's short aliases.
/// `xmake --help` (3.1) is the source of the action and plugin names.
#[derive(Clone, Copy)]
enum ActionKind {
    Build,
    Run,
    Lua,
    /// Task does not leave a single binary for the Zed debugger.
    Skip,
    /// Not a task name. xmake builds this target (`xmake app`).
    Target,
}

fn action_kind(name: &str) -> ActionKind {
    match name.to_ascii_lowercase().as_str() {
        "b" | "build" => ActionKind::Build,
        "r" | "run" => ActionKind::Run,
        "l" | "lua" => ActionKind::Lua,
        "addon" | "c" | "check" | "clean" | "config" | "create" | "doxygen" | "f" | "format"
        | "g" | "global" | "i" | "install" | "m" | "macro" | "p" | "pack" | "package"
        | "plugin" | "project" | "q" | "repo" | "require" | "service" | "show" | "test" | "u"
        | "uninstall" | "update" | "watch" => ActionKind::Skip,
        _ => ActionKind::Target,
    }
}

/// Options that consume the following argument when the default action is build.
const BUILD_VALUE_OPTIONS: &[&str] = &[
    "-F",
    "--file",
    "--files",
    "-j",
    "--jobs",
    "--linkjobs",
    "-o",
    "--builddir",
    "-P",
    "--project",
];

const RUN_VALUE_OPTIONS: &[&str] = &["-g", "--group", "-j", "--jobs", "-w", "--workdir"];

/// Options that take a value no matter which action follows.
/// `--confirm yes` must not be read as a target named `yes`.
const COMMON_VALUE_OPTIONS: &[&str] = &["--confirm", "-F", "--file", "-P", "--project"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DebugPlan {
    /// When set, the debug build step uses these args instead of the task args.
    /// `xmake run app` becomes `xmake build app` so the debugger, not xmake, launches the binary.
    pub(crate) build_args: Option<Vec<String>>,
    /// Target names, in order. Empty means "whichever default binary exists".
    pub(crate) targets: Vec<String>,
    pub(crate) program_args: Vec<String>,
    /// `xmake run -w <dir>` working directory for the debuggee.
    pub(crate) debuggee_cwd: Option<String>,
}

pub(crate) fn debug_plan(args: &[String]) -> Option<DebugPlan> {
    let mut index = 0;
    while index < args.len() {
        let arg = &args[index];
        if is_non_debug_flag(arg) || is_group_option(arg) {
            return None;
        }
        if is_value_option(arg, BUILD_VALUE_OPTIONS) || is_value_option(arg, COMMON_VALUE_OPTIONS) {
            index = consume_value(args, index);
            continue;
        }
        if arg == "--" {
            // `xmake -- app` builds target `app`. Tokens after `--` are names,
            // even when they start with `-`.
            let mut plan = plain_build();
            plan.targets.extend(args[index + 1..].iter().cloned());
            return Some(plan);
        }
        if arg.starts_with('-') {
            index += 1;
            continue;
        }
        return match action_kind(arg) {
            ActionKind::Run => parse_run(&args[index + 1..]),
            ActionKind::Build => parse_build(&args[index + 1..]),
            ActionKind::Lua => parse_lua(&args[index + 1..]),
            ActionKind::Skip => None,
            ActionKind::Target => Some(plan_for_bare_targets(args, index)),
        };
    }
    Some(plain_build())
}

fn plain_build() -> DebugPlan {
    DebugPlan {
        build_args: None,
        targets: Vec::new(),
        program_args: Vec::new(),
        debuggee_cwd: None,
    }
}

/// `xmake app tool` builds those targets. Stop at the first flag so
/// `xmake app -j 8` does not treat `8` as a target. After `--`, every
/// remaining token is a target name.
fn plan_for_bare_targets(args: &[String], index: usize) -> DebugPlan {
    let mut plan = plain_build();
    plan.targets.push(args[index].clone());
    let mut rest = index + 1;
    while rest < args.len() {
        let next = &args[rest];
        if next == "--" {
            plan.targets.extend(args[rest + 1..].iter().cloned());
            break;
        }
        if next.starts_with('-') {
            break;
        }
        plan.targets.push(next.clone());
        rest += 1;
    }
    plan
}

fn parse_build(args: &[String]) -> Option<DebugPlan> {
    let mut plan = plain_build();
    let mut index = 0;
    while index < args.len() {
        let arg = &args[index];
        if is_non_debug_flag(arg) || is_group_option(arg) {
            return None;
        }
        if is_value_option(arg, BUILD_VALUE_OPTIONS) || is_value_option(arg, COMMON_VALUE_OPTIONS) {
            index = consume_value(args, index);
            continue;
        }
        if arg == "--" {
            plan.targets.extend(args[index + 1..].iter().cloned());
            break;
        }
        if arg.starts_with('-') {
            index += 1;
            continue;
        }
        plan.targets.push(arg.clone());
        index += 1;
    }
    Some(plan)
}

fn parse_run(args: &[String]) -> Option<DebugPlan> {
    let mut target = None;
    let mut program_args = Vec::new();
    let mut debuggee_cwd = None;
    let mut index = 0;
    while index < args.len() {
        let arg = &args[index];
        if target.is_some() {
            if arg != "--" {
                program_args.push(arg.clone());
            }
            index += 1;
            continue;
        }
        if arg == "--" {
            program_args.extend(args[index + 1..].iter().cloned());
            break;
        }
        if is_non_debug_flag(arg) || is_group_option(arg) || is_terminal_debug_flag(arg) {
            return None;
        }
        if is_value_option(arg, RUN_VALUE_OPTIONS) || is_value_option(arg, COMMON_VALUE_OPTIONS) {
            // `-P` / `-F` / `--confirm` are consumed so the next word is not
            // the target. They are not copied onto the debug build: the locator
            // reads the task working directory, and `-P` would split that tree.
            if is_workdir_option(arg) {
                debuggee_cwd = option_value(args, index).map(str::to_string);
            }
            index = consume_value(args, index);
            continue;
        }
        if arg.starts_with('-') {
            index += 1;
            continue;
        }
        target = Some(arg.clone());
        index += 1;
    }

    let mut build_args = vec!["build".to_string()];
    if let Some(target) = &target {
        build_args.push(target.clone());
    }
    Some(DebugPlan {
        build_args: Some(build_args),
        targets: target.into_iter().collect(),
        program_args,
        debuggee_cwd,
    })
}

fn parse_lua(args: &[String]) -> Option<DebugPlan> {
    let mut index = 0;
    while index < args.len() {
        let arg = &args[index];
        if is_value_option(arg, &["-d", "--deserialize"]) {
            index = consume_value(args, index);
            continue;
        }
        if arg == "--" {
            break;
        }
        if arg.starts_with('-') {
            index += 1;
            continue;
        }
        // Only the extension's debug-mode build helper is a debug scenario.
        // Other `xmake lua -c` tasks (clean, compile_commands, terminal `run -d`) are not.
        // One surrounding pair of shell quotes is ignored: the task file stores
        // them so zsh does not parse the Lua, and the locator sees them too.
        if lua_source(arg) == lua_source(DEBUG_BUILD_SNIPPET) {
            return Some(plain_build());
        }
        return None;
    }
    None
}

/// Text of a `lua -c` argument after one pair of shell single quotes.
/// Quotes are stripped only when the inside has no single quote, so a Lua
/// string such as `os.execv('xmake', {})` stays intact and does not match.
fn lua_source(arg: &str) -> &str {
    let arg = arg.trim();
    let Some(inner) = arg
        .strip_prefix('\'')
        .and_then(|rest| rest.strip_suffix('\''))
    else {
        return arg;
    };
    if inner.contains('\'') { arg } else { inner }
}

fn is_non_debug_flag(arg: &str) -> bool {
    matches!(
        option_key(arg),
        "-h" | "--help" | "--version" | "--menu" | "--dry-run"
    )
}

fn is_group_option(arg: &str) -> bool {
    if matches!(option_key(arg), "-g" | "--group") {
        return true;
    }
    // xmake accepts `-gtest` and `-g=test` as a group, not only `-g test`.
    let bytes = arg.as_bytes();
    bytes.len() > 2 && bytes[0] == b'-' && bytes[1] == b'g' && bytes[2] != b'-'
}

fn is_terminal_debug_flag(arg: &str) -> bool {
    matches!(option_key(arg), "-d" | "--debug")
}

fn is_workdir_option(arg: &str) -> bool {
    matches!(option_key(arg), "-w" | "--workdir")
}

fn is_value_option(arg: &str, names: &[&str]) -> bool {
    let key = option_key(arg);
    names.iter().any(|name| *name == key)
}

fn option_key(arg: &str) -> &str {
    if let Some((key, _)) = arg.split_once('=')
        && key.starts_with("--")
    {
        return key;
    }
    arg
}

fn option_value<'a>(args: &'a [String], index: usize) -> Option<&'a str> {
    let arg = args.get(index)?;
    if let Some((key, value)) = arg.split_once('=')
        && key.starts_with("--")
    {
        return Some(value);
    }
    args.get(index + 1).map(String::as_str)
}

fn consume_value(args: &[String], index: usize) -> usize {
    let attached = args
        .get(index)
        .and_then(|arg| arg.split_once('='))
        .is_some_and(|(key, _)| key.starts_with("--"));
    if attached || index + 1 >= args.len() {
        index + 1
    } else {
        index + 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan(args: &[&str]) -> Option<DebugPlan> {
        debug_plan(
            &args
                .iter()
                .map(|arg| (*arg).to_string())
                .collect::<Vec<_>>(),
        )
    }

    #[test]
    fn plain_and_verbose_builds_are_debuggable() {
        assert_eq!(plan(&[]), Some(plain_build()));
        assert_eq!(plan(&["-v"]), Some(plain_build()));
        assert_eq!(
            plan(&["build", "app"]),
            Some(DebugPlan {
                build_args: None,
                targets: vec!["app".into()],
                program_args: Vec::new(),
                debuggee_cwd: None,
            })
        );
    }

    #[test]
    fn run_becomes_build_and_keeps_program_args() {
        assert_eq!(
            plan(&["run", "-w", "/tmp", "app", "--verbose"]),
            Some(DebugPlan {
                build_args: Some(vec!["build".into(), "app".into()]),
                targets: vec!["app".into()],
                program_args: vec!["--verbose".into()],
                debuggee_cwd: Some("/tmp".into()),
            })
        );
        assert_eq!(
            plan(&["run"]).unwrap().build_args.as_deref(),
            Some(["build".to_string()].as_slice())
        );
    }

    #[test]
    fn help_version_group_and_terminal_debug_are_skipped() {
        assert!(plan(&["-h"]).is_none());
        assert!(plan(&["--version"]).is_none());
        assert!(plan(&["f", "--menu"]).is_none());
        assert!(plan(&["clean"]).is_none());
        assert!(plan(&["c"]).is_none());
        assert!(plan(&["i"]).is_none());
        assert!(plan(&["p"]).is_none());
        assert!(plan(&["q"]).is_none());
        assert!(plan(&["u"]).is_none());
        assert!(plan(&["m", "foo"]).is_none());
        assert!(plan(&["addon"]).is_none());
        assert!(plan(&["pack"]).is_none());
        assert!(plan(&["doxygen"]).is_none());
        assert!(plan(&["--dry-run"]).is_none());
        assert!(plan(&["-g", "test"]).is_none());
        assert!(plan(&["-gtest"]).is_none());
        assert!(plan(&["-g=test"]).is_none());
        assert!(plan(&["--group=bench/*"]).is_none());
        assert!(plan(&["run", "-gtest"]).is_none());
        assert!(plan(&["run", "-d"]).is_none());
        assert!(plan(&["run", "-d", "app"]).is_none());
    }

    #[test]
    fn value_options_are_not_target_names() {
        assert_eq!(plan(&["--confirm", "yes"]), Some(plain_build()));
        assert_eq!(
            plan(&["run", "--confirm", "yes", "app"]).unwrap().targets,
            vec!["app".to_string()]
        );
        assert_eq!(
            plan(&["run", "-P", "/other", "app", "--flag"]),
            Some(DebugPlan {
                build_args: Some(vec!["build".into(), "app".into()]),
                targets: vec!["app".into()],
                program_args: vec!["--flag".into()],
                debuggee_cwd: None,
            })
        );
        assert_eq!(
            plan(&["build", "--confirm=yes", "app"]).unwrap().targets,
            vec!["app".to_string()]
        );
        // Unknown words are targets. `export` is not an xmake task.
        assert_eq!(
            plan(&["export"]).unwrap().targets,
            vec!["export".to_string()]
        );
        assert_eq!(
            plan(&["--", "app"]).unwrap().targets,
            vec!["app".to_string()]
        );
        assert_eq!(
            plan(&["build", "--", "-app"]).unwrap().targets,
            vec!["-app".to_string()]
        );
        assert_eq!(
            plan(&["build", "lib", "app"]).unwrap().targets,
            vec!["lib".to_string(), "app".to_string()]
        );
        assert_eq!(
            plan(&["-j2", "app"]).unwrap().targets,
            vec!["app".to_string()]
        );
        assert_eq!(
            plan(&["app", "-j", "8"]).unwrap().targets,
            vec!["app".to_string()]
        );
    }

    #[test]
    fn only_the_debug_build_lua_helper_is_debuggable() {
        assert!(plan(&["lua", "-c", DEBUG_BUILD_SNIPPET]).is_some());
        assert!(plan(&["lua", "-c", lua_source(DEBUG_BUILD_SNIPPET)]).is_some());
        assert!(plan(&["lua", "-c", TERMINAL_DEBUG_SNIPPET]).is_none());
        assert!(
            plan(&[
                "lua",
                "-c",
                "os.execv('xmake', {'clean', '-a'}); os.execv('xmake', {})"
            ])
            .is_none()
        );
        assert!(
            plan(&[
                "lua",
                "-c",
                "'os.execv(\"xmake\", {\"clean\", \"-a\"}); os.execv(\"xmake\", {})'"
            ])
            .is_none()
        );
        assert!(plan(&[
            "lua",
            "-c",
            "os.mkdir('.zed'); os.execv('xmake', {'project', '-k', 'compile_commands', '.zed'})",
        ])
        .is_none());
    }
}
