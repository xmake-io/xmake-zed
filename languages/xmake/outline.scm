; Symbols for xmake.lua and the Lua next to it.

; Lua functions: function name(), function a.b(), function a:b()
(function_declaration
  name: (identifier) @name) @item

(function_declaration
  name: (dot_index_expression
    field: (identifier) @name)) @item

(function_declaration
  name: (method_index_expression
    method: (identifier) @name)) @item

; local f = function() end and M.f = function() end
(assignment_statement
  (variable_list .
    name: [
      (identifier) @name
      (dot_index_expression
        field: (identifier) @name)
    ])
  (expression_list .
    value: (function_definition))) @item

; XMake declarations: package("x"), target("x"), option("x"), ...
(function_call
  name: (identifier) @_kind
  arguments: (arguments
    (string
      (string_content) @name))
  (#any-of? @_kind
    "package" "target" "option" "rule" "task" "toolchain" "plugin" "addon"
    "template" "group")) @item

; includes("dir/subdir") pulls in another script; keep it navigable.
(function_call
  name: (identifier) @_kind
  arguments: (arguments
    (string
      (string_content) @name))
  (#eq? @_kind "includes")) @item
