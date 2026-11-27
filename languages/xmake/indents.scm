; Indentation for xmake.lua and the Lua it embeds.

; xmake blocks: package/target/option/... keep their body indented and their
; *_end() closer outdented.
(xmake_block
  arguments: (arguments) @start
  (xmake_block_end)? @end) @indent

; Lua blocks indent their body and outdent the token that closes them.
(if_statement "end" @end) @indent
(do_statement "end" @end) @indent
(while_statement "end" @end) @indent
(for_statement "end" @end) @indent
(repeat_statement "until" @end) @indent
(function_declaration "end" @end) @indent
(function_definition "end" @end) @indent

; elseif/else line up with the if they belong to.
(if_statement (elseif_statement) @outdent)
(if_statement (else_statement) @outdent)

; Brackets indent what they contain.
(_ "{" "}" @end) @indent
(_ "[" "]" @end) @indent
(_ "(" ")" @end) @indent
