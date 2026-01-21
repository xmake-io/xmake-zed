<div align="center">
  <a href="https://xmake.io">
    <img width="200" heigth="200" src="https://github.com/xmake-io/xmake-idea/raw/master/res/logo256.png">
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

A XMake integration for Zed editor with automatic LSP installation, comprehensive syntax highlighting, and build tasks.

It is deeply integrated with [xmake](https://github.com/xmake-io/xmake) and Zed editor to provide a convenient and fast cross-platform c/c++ development and building.

You need install [xmake](https://github.com/xmake-io/xmake) first and a project with `xmake.lua`.

Please see [xmake-github](https://github.com/xmake-io/xmake) and [website](https://xmake.io) if you want to known more about xmake.

## Features

* Language Server (xmake_ls)
* Syntax Highlighting (300+ functions)
* Code Intelligence
* Symbol Navigation
* Project Templates

## Language Server (xmake_ls)

<div align="center">
<img src="https://raw.githubusercontent.com/xmake-io/xmake-zed/main/res/xmake_ls_demo.png" width="80%" />
</div>

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
| Linux | RISC-V | ✅ (musl) |
| Windows | x64 | ✅ |
| Windows | ARM64 | ✅ |
| Windows | x86 (32-bit) | ✅ |

## Installation

1. **Install Lua Extension** (required for syntax highlighting)
   - Press `Cmd+Shift+X` → Search "Lua" → Install
   - xmake.lua files use Lua syntax, so this grammar is needed

2. **Install XMake Extension**
   - Press `Cmd+Shift+X` (macOS) or `Ctrl+Shift+X` (Linux/Windows)
   - Search for "XMake"
   - Click Install

The XMake extension will automatically download and install the xmake_ls language server when you open an `xmake.lua` file.

## Quick Start: Creating Projects

Create new xmake projects using the built-in tasks:

1. Open Command Palette (`Cmd+Shift+P` / `Ctrl+Shift+P`)
2. Type "Tasks: Run Task"
3. Select "xmake: Create project help" to see available options

Example:
```bash
# C++ console project
xmake create -l c++ -t console myproject

# Rust static library
xmake create -l rust -t static mylib

# Zig shared library
xmake create -l zig -t shared mylib

# Qt widget application
xmake create -l c++ -t qt.widgetapp myapp

# iOS app
xmake create -l objc -t xcode.iosapp myapp
```

## Supported Languages (15)

| Language | Console | Static | Shared | Module |
|----------|---------|--------|--------|--------|
| C | ✅ | ✅ | ✅ | ✅ |
| C++ | ✅ | ✅ | ✅ | ✅ |
| Rust | ✅ | ✅ | ❌ | ❌ |
| Zig | ✅ | ✅ | ✅ | ❌ |
| Go | ✅ | ✅ | ❌ | ❌ |
| D Language | ✅ | ✅ | ✅ | ❌ |
| CUDA | ✅ | ✅ | ✅ | ❌ |
| Swift | ✅ | ❌ | ❌ | ❌ |
| Fortran | ✅ | ✅ | ✅ | ❌ |
| Objective-C | ✅ | ❌ | ❌ | ❌ |
| Objective-C++ | ✅ | ❌ | ❌ | ❌ |
| Nim | ✅ | ✅ | ✅ | ❌ |
| Kotlin | ✅ | ✅ | ✅ | ❌ |
| Vala | ✅ | ✅ | ✅ | ❌ |
| Pascal | ✅ | ✅ | ✅ | ❌ |

## Project Templates (25+)

* **Console**: Basic executable projects
* **Static Library**: .a / .lib files
* **Shared Library**: .so / .dylib / .dll files
* **Module**: C/C++ module binaries
* **Qt**: Console, Quick App, Widget App (static/shared variants)
* **TBOX**: Console, Static, Shared library projects
* **wxWidgets**: Desktop GUI applications
* **Xcode**: Bundle, Framework, macOS App, iOS App
* **XMake CLI**: Command-line tool projects

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
        "linux_variant": "musl"
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
* **Don't know?** Use default (auto-selected based on architecture)
* **Alpine Linux, containers:** Use `musl`
* **Older distros:** Use `*-glibc.2.17` variants
* **Latest distros:** Use `x64` or `aarch64`

## Binary Detection Priority

The extension checks for xmake_ls in the following order:

1. **Custom path** from Zed settings (highest priority)
2. **System installation** via PATH (`which xmake_ls`)
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
# macOS/Linux
rm -rf ~/.local/share/zed/extensions/xmake/

# Windows
Remove-Item -Recurse -Force "$env:LOCALAPPDATA\Zed\extensions\xmake"
```

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
        "linux_variant": "musl"
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
* [XMake Create Command](https://xmake.io/guide/basic-commands/create-project.html)

## License

Apache License 2.0 - See [LICENSE](LICENSE.md) for details.

## Acknowledgments

* [XMake](https://xmake.io) build system by Ruki Wang
* [xmake_ls](https://github.com/CppCXY/xmake_ls) language server by CppCXY
* [Zed](https://zed.dev) editor team
