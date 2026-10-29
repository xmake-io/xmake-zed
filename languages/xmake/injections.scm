; Shell commands. os.exec / os.run / os.iorun take a shell string.
; os.execv and the other *v forms take an argv array, not a shell command.
; import() / include() arguments are module paths, not Lua source.
((function_call
  name: (dot_index_expression
    table: (identifier) @_module
    field: (identifier) @_func)
  (#eq? @_module "os")
  (#any-of? @_func "exec" "run" "iorun")
  arguments: (arguments (string) @injection.content))
  (#set! injection.language "bash"))
