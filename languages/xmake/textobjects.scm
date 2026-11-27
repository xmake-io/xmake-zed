; Text objects use Zed's capture names (around / inside).

(function_definition
  body: (_) @function.inside) @function.around

(function_declaration
  body: (_) @function.inside) @function.around

; A package/target/option/... block is a section of the file, which is what
; Zed's class objects are for.
(xmake_block) @class.around

(comment)+ @comment.around
