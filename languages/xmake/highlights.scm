; XMake syntax highlighting
; Inherits from Lua grammar with xmake-specific extensions

; Keywords (from Lua)
[
  "do"
  "else"
  "elseif"
  "end"
  "for"
  "function"
  "if"
  "in"
  "local"
  "repeat"
  "return"
  "then"
  "until"
  "while"
  (break_statement)
] @keyword

; Operators (from Lua)
[
  "and"
  "not"
  "or"
] @keyword.operator

[
  "+"
  "-"
  "*"
  "/"
  "%"
  "^"
  "#"
  "=="
  "~="
  "<="
  ">="
  "<"
  ">"
  "="
  "&"
  "~"
  "|"
  "<<"
  ">>"
  "//"
  ".."
] @operator

; Punctuations (from Lua)
[
  ";"
  ":"
  ","
  "."
] @punctuation.delimiter

; Brackets (from Lua)
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

; Variables (from Lua)
(identifier) @variable

; Constants (from Lua)
((identifier) @constant
 (#match? @constant "^[A-Z][A-Z_0-9]*$"))

(vararg_expression) @constant

(nil) @constant.builtin

[
  (false)
  (true)
] @boolean

; Comments (from Lua)
(comment) @comment

(number) @number

(string) @string
(escape_sequence) @string.escape

; XMake target definition
(function_call
  name: (identifier) @function.definition
  (#match? @function.definition "^(target|task|rule|package|toolchain|plugin|option)$"))

; XMake end functions
(function_call
  name: (identifier) @function.builtin
  (#match? @function.builtin "^(target_end|option_end|rule_end|task_end|package_end|toolchain_end)$"))

; XMake function calls as builtin
(function_call
  name: (identifier) @function.builtin
  (#match? @function.builtin
    ;; Target APIs
    "^(set_kind|set_strip|set_enabled|set_default|set_optional|set_visible|set_warnings|set_optimize|set_languages|set_fpmodels|set_symbols|set_basename|set_filename|set_prefixname|set_suffixname|set_extension|set_targetdir|set_objectdir|set_dependir|set_installdir|set_srcdir|set_buildir|add_files|add_headerfiles|add_installfiles|add_generatedfiles|remove_files|add_extrafiles|add_files_if|add_headerfiles_if|add_includedirs|add_sysincludedirs|add_linkdirs|add_frameworkdirs|add_rpathdirs|add_dependirs|add_scangedirs|add_defines|add_undefines|add_defines_h|add_undefines_h|add_defines_if_ok|add_undefines_if_ok|add_links|add_syslinks|add_frameworks|add_deps|add_packages|add_linkages|add_strongdeps|add_cflags|add_cxxflags|add_ldflags|add_shflags|add_arflags|add_cxflags|add_mflags|add_iflags|add_vexflags|add_asmflags|set_cflags|set_cxxflags|set_ldflags|set_options|add_options|set_showmenu|set_category|set_description|set_values|set_enable|set_required|set_native|add_rules|add_rule|set_rules|set_extensions|add_requires|add_requireconfs|add_require_imports|add_repositories|set_repositories|set_requires|set_requireconfs|set_policy|set_base|set_plat|set_arch|set_modes|set_mode|set_runtimes|set_env|set_runenv|set_safeenv|set_project|set_version|set_xmakever|set_homepage|set_author|set_license|set_pcxxheader|set_pcxxprefix|set_pcheader|set_pchprefix|set_unity|set_unity_size|set_unity_junction|set_linker|add_linkflags|add_installfiles|set_wildcard|set_encrypt|set_encrypt_file|set_installskip|add_tests|set_testconfig|add_testfiles|add_testincludes|add_benchmarks|set_benchconfig|add_benchmarkfiles|set_private|set_public|set_interface|try_add|on_check|before_check|after_check|deprecated|configvar|includes|add_includes|import|include|add_subdirs|add_subfiles|set_configdir|set_rootdir|set_programdir|set_packagedir|set_tmpdir|set_homeir|set_cachedir|set_logdir|set_md5dir|set_sdkdir|set_sysincludedirs|add_sdks|add_vectorexts|set_vectorexts|add_asmflags|add_iflags|add_premacros|add_definemacros|add_toolchains|add_toolnames|add_tools|set_toolchains)$"))

; XMake hooks (on_load, on_build, etc.)
(function_call
  name: (identifier) @function.builtin
  (#match? @function.builtin
    "^(on_load|on_config|on_prepare|on_prepare_file|on_prepare_files|on_link|on_build|on_build_file|on_build_files|on_clean|on_package|on_install|on_uninstall|on_run|before_load|before_config|before_prepare|before_prepare_file|before_prepare_files|before_link|before_build|before_build_file|before_build_files|before_clean|before_package|before_install|before_uninstall|before_run|after_load|after_config|after_prepare|after_prepare_file|after_prepare_files|after_link|after_build|after_build_file|after_build_files|after_clean|after_package|after_install|after_uninstall|after_run)$"))

; XMake global modules (os, path, etc.)
(function_call
  name: (identifier) @function.builtin
  (#match? @function.builtin
    "^(os|path|string|table|math|io|debug|coroutine|hash|utils|linuxos|macos|winos|xmake|core|api|sandbox|print|printf|cprint|pprint|raise|vformat|format|printfrag|is_host|is_plat|is_arch|is_mode|is_kind|is_config|has_config|get_config|config|option|target|rule|task|package|cli|argparse|template|process|net|http|ping|privilege|sudo|archive|platform|detect|tool|runjobs|jobgraph|jobpool|async)$"))

; XMake set_menu and related
(function_call
  name: (identifier) @function.builtin
  (#match? @function.builtin
    "^(set_menu|set_core|on_run|before_run|after_run|set_extensions|set_priority|set_properties|set_homepage|set_version|set_license|set_urls|add_sources|set_kind|set_family|set_toolset|on_link|on_install|has_features|has_ctypes|has_cxxtypes|has_cfuncs|has_cxxfuncs|has_cincludes|has_cxxincludes|has_links|check)$"))

; XMake add_rules with mode
(function_call
  name: (identifier) @function.builtin
  (#match? @function.builtin "^(mode\\.debug|mode\\.release|mode\\.profile|mode\\.minsizerel|mode\\.lto)$"))
