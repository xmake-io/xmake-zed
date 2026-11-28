; Shell commands: os.exec / os.run / os.iorun hand their string to the shell.
; os.execv / os.runv / os.iorunv take an argv table instead, so they are not
; injected.
((function_call
  name: (dot_index_expression
    table: (identifier) @_module
    field: (identifier) @_func)
  arguments: (arguments
    (string
      (string_content) @injection.content)))
  (#eq? @_module "os")
  (#any-of? @_func "exec" "run" "iorun")
  (#set! injection.language "bash"))

; Port scripts written from a recipe: io.writefile("xmake.lua", [[ ... ]]).
; The embedded chunk is an xmake script, so highlight it as one instead of
; leaving a wall of plain string.
((function_call
  name: (dot_index_expression
    table: (identifier) @_module
    field: (identifier) @_func)
  arguments: (arguments
    (string
      (string_content) @_filename)
    (string
      (string_content) @injection.content)))
  (#eq? @_module "io")
  (#eq? @_func "writefile")
  (#eq? @_filename "xmake.lua")
  (#set! injection.language "xmake"))
