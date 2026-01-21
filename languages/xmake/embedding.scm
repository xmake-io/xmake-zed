; XMake code folding
; Defines what can be collapsed/folded

(
    (comment)* @context
    .
    (function_declaration
        "function" @name
        name: (_) @name
        (comment)* @collapse
        body: (block) @collapse
    ) @item
)

(
    (comment)* @context
    .
    (function_definition
        "function" @name
        name: (_) @name
        (comment)* @collapse
        body: (block) @collapse
    ) @item
)

; Collapse target/task/rule bodies
(
    (comment)* @context
    .
    (function_call
        name: (identifier) @name
        (#any-of? @name "target" "task" "rule" "package" "toolchain" "plugin" "option")
        arguments: (arguments
            (string) @name
            (block) @collapse
        )
    ) @item
)

; Collapse if statements
(if_statement
  (comment)* @context
  "then" @collapse
  (block) @collapse
  "end" @end) @item

; Collapse loops
(do_statement
  (comment)* @context
  "do" @collapse
  (block) @collapse
  "end" @end) @item

(while_statement
  (comment)* @context
  "do" @collapse
  (block) @collapse
  "end" @end) @item

(for_statement
  (comment)* @context
  "do" @collapse
  (block) @collapse
  "end" @end) @item

(repeat_statement
  (comment)* @context
  "repeat" @collapse
  (block) @collapse
  "until" @end) @item

; Collapse table constructors
(table_constructor
  (comment)* @context
  "{" @collapse
  "}" @end) @item
