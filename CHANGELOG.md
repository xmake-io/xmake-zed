# Changelog

## 0.3.1

- In a project that contains `xmake.lua`, every `.lua` file uses this extension's Lua language. The extension records that in the project's `.zed/settings.json`.

## 0.3.0

Beta. The copy in the Zed extension catalog is still 0.1.0. This tag is the installable preview.

- Task scripts are one shell word, so zsh does not parse Lua `end` or `{}`.
- Tasks are merged into Zed's global `tasks.json`. The extension does not write `.zed/tasks.json` or `.zed/debug.json`.
- The debugger list is available while `xmake.lua` is the focused file.
- Document the settings that keep `xmake.lua` on xmake_ls when the Lua extension is installed.

## 0.2.0

Not published to the Zed catalog. Included in the 0.3.0 preview.

- Build the extension as a `wasm32-wasip2` component.
- Debug locator for CodeLLDB and GDB, including the macOS `/tmp` source map.
- Declare `process:exec` for `xmake`.
- Ship the xmake 3.1 create templates as global tasks.
