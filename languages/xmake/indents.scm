; Indentation rules for xmake.lua files
; Inherits from Lua grammar

; Control flow (from Lua)
(if_statement "end" @end) @indent
(do_statement "end" @end) @indent
(while_statement "end" @end) @indent
(for_statement "end" @end) @indent
(repeat_statement "until" @end) @indent
(function_declaration "end" @end) @indent
(function_definition "end" @end) @indent

; Brackets (from Lua)
(_ "[" "]" @end) @indent
(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent
