;; Syntax highlighting for xmake.lua.
;;
;; The first half is the official tree-sitter-lua query, copied verbatim from
;; tree-sitter-lua (the grammar this repository forks, pinned in extension.toml),
;; so Lua constructs keep exactly the colours they have in .lua buffers.
;;
;; The XMake section comes after it: Zed applies later patterns last, so those
;; rules override the base ones where xmake needs a different reading.

;; ---------------------------------------------------------------------------
;; tree-sitter-lua base query (verbatim)
;; ---------------------------------------------------------------------------

;; Keywords

"return" @keyword.return

[
 "goto"
 "in"
 "local"
] @keyword

(label_statement) @label

(break_statement) @keyword

(do_statement
[
  "do"
  "end"
] @keyword)

(while_statement
[
  "while"
  "do"
  "end"
] @repeat)

(repeat_statement
[
  "repeat"
  "until"
] @repeat)

(if_statement
[
  "if"
  "elseif"
  "else"
  "then"
  "end"
] @conditional)

(elseif_statement
[
  "elseif"
  "then"
  "end"
] @conditional)

(else_statement
[
  "else"
  "end"
] @conditional)

(for_statement
[
  "for"
  "do"
  "end"
] @repeat)

(function_declaration
[
  "function"
  "end"
] @keyword.function)

(function_definition
[
  "function"
  "end"
] @keyword.function)

;; Operators

(binary_expression operator: _ @operator)

(unary_expression operator: _ @operator)

[
 "and"
 "not"
 "or"
] @keyword.operator

;; Punctuations

[
  ";"
  ":"
  ","
  "."
] @punctuation.delimiter

;; Brackets

[
 "("
 ")"
 "["
 "]"
 "{"
 "}"
] @punctuation.bracket

;; Variables

(identifier) @variable

((identifier) @variable.builtin
 (#eq? @variable.builtin "self"))

(variable_list
  (attribute
    "<" @punctuation.bracket
    (identifier) @attribute
    ">" @punctuation.bracket))

;; Constants

((identifier) @constant
 (#match? @constant "^[A-Z][A-Z_0-9]*$"))

(vararg_expression) @constant

(nil) @constant.builtin

[
  (false)
  (true)
] @boolean

;; Tables

(field name: (identifier) @field)

(dot_index_expression field: (identifier) @field)

(table_constructor
[
  "{"
  "}"
] @constructor)

;; Functions

(parameters (identifier) @parameter)

(function_declaration
  name: [
    (identifier) @function
    (dot_index_expression
      field: (identifier) @function)
  ])

(function_declaration
  name: (method_index_expression
    method: (identifier) @method))

(assignment_statement
  (variable_list .
    name: [
      (identifier) @function
      (dot_index_expression
        field: (identifier) @function)
    ])
  (expression_list .
    value: (function_definition)))

(table_constructor
  (field
    name: (identifier) @function
    value: (function_definition)))

(function_call
  name: [
    (identifier) @function.call
    (dot_index_expression
      field: (identifier) @function.call)
    (method_index_expression
      method: (identifier) @method.call)
  ])

(function_call
  (identifier) @function.builtin
  (#any-of? @function.builtin
    ;; built-in functions in Lua 5.1
    "assert" "collectgarbage" "dofile" "error" "getfenv" "getmetatable" "ipairs"
    "load" "loadfile" "loadstring" "module" "next" "pairs" "pcall" "print"
    "rawequal" "rawget" "rawset" "require" "select" "setfenv" "setmetatable"
    "tonumber" "tostring" "type" "unpack" "xpcall"))

;; Others

(comment) @comment

(hash_bang_line) @preproc

(number) @number

(string) @string

(escape_sequence) @string.escape

;; ---------------------------------------------------------------------------
;; XMake
;; ---------------------------------------------------------------------------

;; Block declarations: package/target/option/... open a block, so the grammar
;; gives them their own node; the opener and its *_end() closer share the
;; builtin colour so a block reads as one unit.
(xmake_block
  name: (xmake_block_name) @function.builtin)

(xmake_block_end) @function.builtin

;; Platform/mode predicates are xmake built-ins rather than ordinary calls.
(function_call
  name: (identifier) @function.builtin
  (#any-of? @function.builtin
    "is_plat" "is_arch" "is_mode" "is_kind" "is_host" "is_subhost"
    "is_config" "has_config" "get_config"))

;; Method calls (package:config, target:add, ...) use the call colour; the base
;; query only defines @method.call for them.
(function_call
  name: (method_index_expression
    method: (identifier) @function.call))

;; Dotted calls (os.cp, path.join, table.insert, ...) as well.
(function_call
  name: (dot_index_expression
    field: (identifier) @function.call))

;; xmake's global modules: colour the namespace so "os" does not look like a
;; free variable next to the function it carries.
(function_call
  name: (dot_index_expression
    (identifier) @variable.builtin)
  (#any-of? @variable.builtin
    "os" "io" "path" "table" "string" "math" "coroutine" "utf8" "debug"
    "xmake" "semver" "hash" "utils" "process" "timer" "base64" "csv" "json"
    "net" "http" "winos" "linuxos" "macos" "privilege" "jobgraph" "jobpool"
    "async" "sandbox" "config"))

;; Table keys ({headeronly = true}, {optional = true}, ...) are properties.
(table_constructor
  (field
    name: (identifier) @property))

;; Keep the base rule that reads "name = function() end" as a function.
(table_constructor
  (field
    name: (identifier) @function
    value: (function_definition)))

;; Function parameters, including the hook argument: on_install(function (package)).
(parameters (identifier) @variable.parameter)

;; Control flow. The base query labels these tokens @conditional, @repeat and
;; @keyword.return; Zed themes (One Dark Pro Next among them) define none of
;; those, so the keywords rendered as plain text. Function bodies keep
;; @keyword.function; everything else takes the keyword colour.
[
  "if" "then" "elseif" "else"
  "for" "while" "do" "repeat" "until"
  "return"
] @keyword

(if_statement "end" @keyword)
(while_statement "end" @keyword)
(for_statement "end" @keyword)
(do_statement "end" @keyword)

;; Dot field access (configs.kind) is @field in the base query, which no Zed
;; theme defines; read it as a property and re-assert call names after it.
(dot_index_expression
  field: (identifier) @property)

(function_call
  name: (dot_index_expression
    field: (identifier) @function.call))

;; Method declarations (function a:b()) are @method in the base query.
(function_declaration
  name: (method_index_expression
    method: (identifier) @function))

;; `package("x") ... end` is not Lua syntax, so tree-sitter keeps the closing
;; `end` as an error node. Colour it like the `end` of a real block instead of
;; leaving it looking like an identifier.
((ERROR (identifier) @keyword.function)
 (#eq? @keyword.function "end"))
