; Indentation for xmake.lua and the Lua it embeds.
;
; xmake's own blocks (package/target/option/...) are plain Lua calls, so they
; have no node to indent: config.toml covers those with line patterns.

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
