; XMake text objects
; Enables selecting code blocks like functions, targets, tasks, etc.

; Select entire function definition
(function_definition) @function.outer

; Select entire function declaration
(function_declaration) @function.outer

; Select if statement block
(if_statement) @conditional.outer

; Select loop statement blocks
(do_statement) @repeat.outer
(while_statement) @repeat.outer
(for_statement) @repeat.outer
(repeat_statement) @repeat.outer

; Select table constructors
(table_constructor) @constructor.outer

; Select function call arguments
(arguments) @parameter.outer

; Select table fields
(table_constructor
  (field) @property.outer)
