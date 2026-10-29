<div align="center">
  <a href="https://xmake.io">
    <img width="200" height="200" src="https://github.com/xmake-io/xmake-idea/raw/master/res/logo256.png">
  </a>

  <h1>xmake-zed</h1>

  <div>
    <a href="https://github.com/xmake-io/xmake-zed/blob/main/LICENSE.md">
      <img src="https://img.shields.io/github/license/xmake-io/xmake-zed.svg?colorB=f48041&style=flat-square" alt="license" />
    </a>
    <a href="https://www.reddit.com/r/xmake/">
      <img src="https://img.shields.io/badge/chat-on%20reddit-ff3f34.svg?style=flat-square" alt="Reddit" />
    </a>
    <a href="https://t.me/tbooxorg">
      <img src="https://img.shields.io/badge/chat-on%20telegram-blue.svg?style=flat-square" alt="Telegram" />
    </a>
    <a href="https://jq.qq.com/?_wv=1027&k=5hpwWFv">
      <img src="https://img.shields.io/badge/chat-on%20QQ-ff69b4.svg?style=flat-square" alt="QQ" />
    </a>
    <a href="https://discord.gg/xmake">
      <img src="https://img.shields.io/badge/chat-on%20discord-7289da.svg?style=flat-square" alt="Discord" />
    </a>
    <a href="https://xmake.io/about/sponsor">
      <img src="https://img.shields.io/badge/donate-us-orange.svg?style=flat-square" alt="Donate" />
    </a>
  </div>

  <p>XMake integration for Zed editor</p>
</div>

## Introduction

An XMake integration for Zed with automatic language-server installation, syntax highlighting, build tasks, and in-editor debugging.

Install [xmake](https://github.com/xmake-io/xmake) first and open a project that contains `xmake.lua`.

See the [xmake repository](https://github.com/xmake-io/xmake) and the [website](https://xmake.io) to learn the build system itself.

## Features

* Language Server (xmake_ls)
* Syntax Highlighting (300+ functions)
* Code Intelligence
* Symbol Navigation
* Build, run, and debug tasks
* DAP locator for CodeLLDB / GDB
* `compile_commands.json` generation into `.zed/`
* Project Templates

* **Full LSP Integration**: Complete language server support via [xmake_ls](https://github.com/CppCXY/xmake_ls)
* **Auto-completion**: Intelligent code completion for XMake APIs
* **Diagnostics**: Real-time error detection and warnings
* **Hover Information**: Documentation on hover
* **Code Actions**: Quick fixes and refactoring support
* **Go to Definition**: Navigate to symbol definitions
* **Find References**: Locate all usages of symbols
* **Document Symbols**: Outline of targets, options, rules, tasks
* **Workspace Symbols**: Search across all xmake.lua files
* **Formatting**: Document and range formatting support

## Syntax Highlighting (300+ functions)

* **Target APIs**: target, set_kind, set_version, set_description, set_default
* **File APIs**: add_files, add_headerfiles, add_installfiles, remove_files
* **Include APIs**: add_includedirs, add_sysincludedirs, add_defines, add_undefines
* **Linking APIs**: add_links, add_linkdirs, add_syslinks, add_frameworks, add_deps
* **Compiler APIs**: add_cflags, add_cxxflags, add_ldflags, set_warnings, set_optimize
* **Rule APIs**: add_rules, rule, on_load, on_build, before_build, after_build
* **Option APIs**: option, add_options, set_default, set_showmenu
* **Package APIs**: add_requires, add_packages, add_repositories, add_requireconfs
* **Task APIs**: task, on_run, set_menu
* **Utility APIs**: is_plat, is_arch, is_mode, is_host, has_config, get_config
* **Global Modules**: os, path, string, table, math, io, debug, coroutine, hash, utils

## Platform Support

| Platform | Architecture | Auto-Install |
|----------|-------------|--------------|
| macOS | ARM64 (Apple Silicon) | ✅ |
| macOS | x64 (Intel) | ✅ |
| Linux | x64 | ✅ |
| Linux | ARM64 | ✅ |
| Linux | RISC-V | set `linuxVariant` to `riscv64` |
| Windows | x64 | ✅ |
| Windows | ARM64 | ✅ |
| Windows | x86 (32-bit) | ✅ |

## Installation

1. Open the extension view with `Cmd+Shift+X` (macOS) or `Ctrl+Shift+X` (Linux/Windows).
2. Search for XMake and install it.

Opening `xmake.lua` downloads xmake_ls. Syntax highlighting uses the Lua grammar shipped with this extension. A separate Lua extension is not required.

## Tasks

Zed shows tasks in `task: spawn`, not in the command palette. On startup this extension merges its tasks into Zed's global `tasks.json` (`~/.config/zed/tasks.json` on macOS and Linux, `%APPDATA%\Zed\tasks.json` on Windows). They then appear in every project, from a C or C++ file, without opening `xmake.lua`. Tasks that are not named `xmake: …` are left alone. This runs after Zed is allowed to execute `xmake`; restart Zed once that permission is granted.

`xmake: install workspace tasks` writes the same list into the current project's `.zed/tasks.json`, and writes `.zed/debug.json` only when that file is missing. Delete `.zed/debug.json` and run the install task again to refresh an older debug scenario.

Optional keybinding in `~/.config/zed/keymap.json`:

```json
[
  {
    "context": "Workspace",
    "bindings": {
      "ctrl-shift-b": ["task::Spawn", { "task_name": "xmake: build" }]
    }
  }
]
```

Useful tasks:

| Task | What it runs |
|------|----------------|
| `xmake: build` | `xmake` |
| `xmake: run` | `xmake run` |
| `xmake: debug` | configure debug mode, then build |
| `xmake: debug in terminal` | debug build, then `xmake run -d` (lldb/gdb in the terminal) |
| `xmake: generate compile_commands` | write `.zed/compile_commands.json` for clangd |
| `xmake: install workspace tasks` | copy tasks into `.zed/tasks.json` |

Create tasks are listed under [Creating a project](#creating-a-project).

## Debugging

Zed's built-in C/C++ adapters (CodeLLDB, GDB) are used. This extension registers an **xmake debug locator**. It turns an `xmake` build or `xmake run` task into a debug session: the build runs first, then Zed launches the binary reported by `target:targetfile()`.

Zed blocks that second step until this extension is allowed to run `xmake`. The manifest already lists `process:exec`. Grant it in Zed settings, then restart Zed. Without the grant the build still succeeds and the debugger reports that no locator completed.

```json
{
  "extension": {
    "granted_extension_capabilities": [
      { "kind": "process:exec", "command": "xmake", "args": ["**"] }
    ]
  }
}
```

On Windows, grant `xmake.exe` the same way. `xmake.cmd` and `xmake.bat` are listed in the manifest for the same reason.

On macOS, CodeLLDB is told that `/tmp` and `/var` are `/private/tmp` and `/private/var`. Otherwise a binary built through one name and a breakpoint set through the other never stop.

`xmake: debug` is the task that matches "compile in debug mode and debug inside Zed". It runs `xmake f -m debug -y`, builds, and hands the binary to CodeLLDB or GDB. `xmake run <target> -- args` is the same flow for one target: the build step becomes `xmake build <target>`, and the arguments after the target are passed to the program. The program starts in the target's `set_rundir`, or in the binary's directory when that is unset. `xmake run -w <dir>` overrides it. A named target whose binary is not on disk is not replaced with a different target.

1. Set breakpoints in the program source.
2. Run `debugger: start` (`debugger::Start`). The `xmake.lua` buffer does not need to be open.
3. Pick **xmake: debug current target**. That entry comes from `.zed/debug.json`. The install task writes it, or you can copy [debug.json.example](debug.json.example). The scenario has no `program` field on purpose. Zed asks the locator for the binary after the debug build finishes.

**xmake: debug**, **xmake: build**, and **xmake: run** appear in that same list after workspace tasks are installed. Until then, those names exist only while `xmake.lua` is the active file.

`xmake: debug in terminal` uses xmake's own `xmake run -d` (system lldb/gdb/windbg) instead of the Zed debug adapter. It does not appear in the Zed debugger list.

## IntelliSense (`compile_commands.json`)

**xmake: generate compile_commands** writes `compile_commands.json` to `.zed/` (gitignore that folder, same idea as VSCode's `.vscode/`) and creates a `.clangd` file if you do not already have one:

```yaml
CompileFlags:
  CompilationDatabase: .zed
```

If you already manage clangd yourself, set:

```json
{
  "lsp": {
    "clangd": {
      "binary": {
        "arguments": ["--compile-commands-dir=.zed"]
      }
    }
  }
}
```

## Creating a project

Every template from `xmake create --list` (xmake 3.1) is a global task named `xmake: create …`. You do not need `xmake.lua` open. `xmake: create project help` prints the same list from xmake.

The first create writes the project into the folder you have open, so `xmake.lua` is at that folder's root and build, run, and debug find it. The target is named `app`, or `lib` for a library. The folder name is not used. A folder called `try` would otherwise become `namespace try` in the C# template, and that does not compile.

A later create in that same folder does not overwrite the first project. It adds `project2`, then `project3`, and inserts `includes("project2")` at the top of the root `xmake.lua`. `xmake` at the root builds every included project.

| Language | Console | Static | Shared | Module |
|----------|---------|--------|--------|--------|
| C | yes | yes | yes | binary and shared |
| C++ | yes | yes | yes | binary and shared |
| C# | yes | yes | yes | |
| CUDA | yes | yes | yes | |
| D | yes | yes | yes | |
| Fortran | yes | yes | yes | |
| Go | yes | yes | | |
| Kotlin | yes | yes | yes | |
| Nim | yes | yes | yes | |
| Objective-C | yes | | | |
| Objective-C++ | yes | | | |
| Pascal | yes | | yes | |
| Rust | yes | yes | yes | |
| Swift | yes | | | |
| Vala | yes | yes | yes | |
| Zig | yes | yes | yes | |

C and C++ also have an `xmake.cli` task. C++ adds Qt (console, static, shared, Quick, Quick static, widget, widget static) and wxWidgets. Objective-C adds Xcode bundle, framework, iOS app, iOS app with framework, macOS app, and macOS app with framework. Objective-C++ adds the Xcode bundle and framework.

xmake 3.1 has no TBOX create template, and Pascal has no static library template. Those tasks are not invented here.

## Configuration

### Using System-Installed xmake_ls

If you already have `xmake_ls` installed via your package manager or built from source, the extension will automatically detect and use it from your PATH.

```bash
# Example: Install from source
cargo install --git https://github.com/CppCXY/xmake_ls xmake_ls
```

### Custom Binary Path

Override the binary location in your Zed settings (`~/.config/zed/settings.json`):

```json
{
  "lsp": {
    "xmake-ls": {
      "binary": {
        "path": "/custom/path/to/xmake_ls"
      }
    }
  }
}
```

### Linux Variant Selection

Linux users can choose their preferred binary variant. Add to your Zed settings:

```json
{
  "lsp": {
    "xmake-ls": {
      "settings": {
        "linuxVariant": "musl"
      }
    }
  }
}
```

#### Available Linux Variants

| Variant | Description | Best For |
|---------|-------------|----------|
| `x64-glibc.2.17` | Default for x86_64 | Ubuntu 16.04+, Debian 8+, RHEL/CentOS 7+ |
| `x64` | Newer glibc | Recent distros (Ubuntu 20.04+, Fedora 30+) |
| `aarch64-glibc.2.17` | Default for ARM64 | ARM-based servers, Raspberry Pi 3+ |
| `musl` | Alpine Linux libc | Alpine Linux, embedded systems, containers |
| `riscv64` | RISC-V architecture | RISC-V hardware |

**Variant Selection Guide:**
* **Don't know?** Leave `linuxVariant` as `auto` (x86_64 → `x64-glibc.2.17`, ARM64 → `aarch64-glibc.2.17`)
* **Alpine Linux, containers:** Use `musl`
* **RISC-V:** Set `linuxVariant` to `riscv64` (`auto` does not pick RISC-V)
* **Older distros:** Use `*-glibc.2.17` variants
* **Latest distros:** Use `x64` or `aarch64`

## Binary Detection Priority

The extension checks for xmake_ls in the following order:

1. **Custom path** from Zed settings (highest priority)
2. **System installation** on the worktree PATH (`xmake_ls` / `xmake_ls.exe`)
3. **Cached download** from previous extension installation
4. **Auto-download** from GitHub releases (with platform/variant detection)

## Troubleshooting

### View Extension Logs

Run Zed from the terminal to see detailed logs:

```bash
zed --foreground
```

### Force Re-download

Remove the cached binary to trigger a fresh download:

```bash
# macOS
rm -rf ~/Library/Application\ Support/Zed/extensions/work/xmake

# Linux
rm -rf ~/.local/share/zed/extensions/work/xmake

# Windows
Remove-Item -Recurse -Force "$env:LOCALAPPDATA\Zed\extensions\work\xmake"
```

### Build succeeds, debugger says no locator completed

The build task runs in the terminal. The locator runs inside the extension and is refused until `process:exec` for `xmake` is granted. Add the settings block in [Debugging](#debugging) and restart Zed.

### Language Server Not Starting

1. Verify xmake_ls is executable:
   ```bash
   xmake_ls --version
   ```

2. Check Zed's language server status:
   * Open Command Palette (`Cmd+Shift+P` / `Ctrl+Shift+P`)
   * Type "Zed: Open Log"
   * Look for xmake_ls errors

3. Try manual installation:
   ```bash
   # Download for your platform from:
   # https://github.com/CppCXY/xmake_ls/releases/latest
   ```

### Linux: Wrong Binary Variant

If you're getting glibc version errors, explicitly set your variant:

```json
{
  "lsp": {
    "xmake-ls": {
      "settings": {
        "linuxVariant": "musl"
      }
    }
  }
}
```

## How to contribute?

Please see [CONTRIBUTING](CONTRIBUTING.md) for details.

## Resources

* [XMake Documentation](https://xmake.io)
* [xmake_ls Language Server](https://github.com/CppCXY/xmake_ls)
* [Zed Extension API](https://zed.dev/docs/extensions)
* [xmake create](https://xmake.io/guide/basic-commands/create-project.html)

## License

Apache License 2.0 - See [LICENSE](LICENSE.md) for details.

## Acknowledgments

* [XMake](https://xmake.io) build system by Ruki Wang
* [xmake_ls](https://github.com/CppCXY/xmake_ls) language server by CppCXY
* [Zed](https://zed.dev) editor team
