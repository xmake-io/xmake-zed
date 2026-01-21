; Symbol outline for xmake.lua files
; Uses Lua TreeSitter grammar node names

(function_declaration "function" @context) @item

(function_definition "function" @context) @item

; XMake target definitions
(function_call
  name: (identifier) @_kind
  arguments: (arguments (string) @name . _)
  (#eq? @_kind "target")) @item

; Options
(function_call
  name: (identifier) @_kind
  arguments: (arguments (string) @name . _)
  (#eq? @_kind "option")) @item

; Rules
(function_call
  name: (identifier) @_kind
  arguments: (arguments (string) @name . _)
  (#eq? @_kind "rule")) @item

; Packages
(function_call
  name: (identifier) @_kind
  arguments: (arguments (string) @name . _)
  (#eq? @_kind "package")) @item

; Tasks
(function_call
  name: (identifier) @_kind
  arguments: (arguments (string) @name . _)
  (#eq? @_kind "task")) @item

; Plugins
(function_call
  name: (identifier) @_kind
  arguments: (arguments (string) @name . _)
  (#eq? @_kind "plugin")) @item

; Toolchains
(function_call
  name: (identifier) @_kind
  arguments: (arguments (string) @name . _)
  (#eq? @_kind "toolchain")) @item

; Namespaces
(function_call
  name: (identifier) @_kind
  arguments: (arguments (string) @name . _)
  (#eq? @_kind "namespace")) @item

; Includes
(function_call
  name: (identifier) @_kind
  arguments: (arguments (string) @name . _)
  (#eq? @_kind "includes")) @item

; Imports
(function_call
  name: (identifier) @_kind
  arguments: (arguments (string) @name . _)
  (#eq? @_kind "import")) @item
