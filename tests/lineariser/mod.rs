crate::ssa!(

definition: "int x = 2;" => "int x0 = 2"

declaration: "int y;" => "int x0 = \u{2205} "

scoped_redeclaration: "int y; { int y = 2; }" => "int x0 = \u{2205} \nint x1 = 2"

unscoped_redefinition: "{ int y = 2; int y = 3; }" =>
":1:18: error: Redefinition of variable y
    1 | { int y = 2; int y = 3; }
                         ^
"

definition_after_declaration: "int y; int y = 2;" => "int x0 = 2"

definition_wrong_type: "int y; char y = 2;" =>
":1:13: error: Redeclaration of y with a different type
    1 | int y; char y = 2;
                    ^
"

declaration_wrong_type: "int y = 2; char y;" =>
":1:17: error: Redeclaration of y with a different type
    1 | int y = 2; char y;
                        ^
"

multiple_declarations: "int x; int x = NULL; int x; int x;" => "int x0 = NULL"

function_declaration:
    "const char* func(static volatile int** first_argument, struct custom * arg2)"
=> "f0(static volatile int * *, struct custom *) -> const char * ;"

function_definition: "int**f(int v) {}" => "f0(int) -> int * * \u{2205} "

function_def_after_decl: "int f(int v); int f(int v) {} int f(int v);" => "f0(int) -> int \u{2205} "

function_redefinition: "int f(int v); int f(int v) {} int f(int v) {}" =>
":1:35: error: Redefinition of function f
    1 | int f(int v); int f(int v) {} int f(int v) {}
                                          ^
"

function_def_wrong_type: "int f(int v); char f(int v) {}" =>
":1:20: error: Redeclaration of function f with a different signature
    1 | int f(int v); char f(int v) {}
                           ^
"

function_decl_wrong_type: "int f(int v) {} char f(int v)" =>
":1:22: error: Redeclaration of function f with a different signature
    1 | int f(int v) {} char f(int v)
                             ^
"

function_shadow_variable: "int f; int f(int v);" =>
":1:12: error: Function declaration shadows variable f
    1 | int f; int f(int v);
                   ^
"

variable_shadow_function: "int f(bool v); int f;" =>
":1:20: error: Variable declaration shadows function f
    1 | int f(bool v); int f;
                           ^
"

function_return: "int f() { return 1; }" =>
"f0() -> int
  BB0:
    return 1"

hello_world: r#"int f() { printf("Hello, world!"); return 1; }"# =>
r#"f0() -> int
  BB0:
    call printf("Hello, world!")
    return 1"#

);
