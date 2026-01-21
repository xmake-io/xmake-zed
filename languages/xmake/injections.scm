; XMake language injections
; Injects shell highlighting into os.exec, os.run calls

; Shell command injection
((function_call
  name: (identifier) @_func
  (#any-of? @_func "exec" "run" "execv" "runv" "iorun" "iorunv")
  arguments: (arguments (string) @injection.content))
  (#set! injection.combined)
  (#set! injection.language "shell"))

; Lua code injection in import/include
((function_call
  name: (identifier) @_func
  (#any-of? @_func "import" "include")
  arguments: (arguments (string) @injection.content))
  (#set! injection.combined)
  (#set! injection.language "lua"))
