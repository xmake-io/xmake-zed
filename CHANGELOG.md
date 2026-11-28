# Changelog

## 0.4.0

- Highlight xmake files from the tree-sitter-lua query instead of a hand-kept list of API names. Calls are matched by shape, so `add_urls`, `add_versions`, `on_test`, `package:config(...)` and `os.cp(...)` are no longer mistaken for variables.
- Use [tree-sitter-xmake](https://github.com/Arteiimis/tree-sitter-xmake): `package`/`target`/`option`/... now produce an `xmake_block` node, so a block indents, outlines and selects as one unit.
- Shell highlighting for `os.exec` / `os.run` / `os.iorun`, and xmake highlighting for the port scripts written with `io.writefile("xmake.lua", [[ ... ]])`.
- Control-flow keywords take `@keyword`; Zed themes do not define `@conditional` / `@repeat`, so `if`/`then`/`end` used to render as plain text.
- Outline names functions (including `M.f = function() end`) and reads declaration names without their quotes.

## 0.3.3

- Stop writing Zed's global `tasks.json` and the project's `.zed/settings.json`. Tasks stay on `xmake.lua`. Copy the task file, or set `file_types`, yourself if you want them everywhere.

## 0.3.2

- Only `xmake.lua` is associated by file name. Other `.lua` files in that project are associated after `.zed/settings.json` is written.
- Leave an existing `.zed/settings.json` unchanged when it is not plain JSON, including files that contain `//` inside strings.

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
