crate::ssa!(

simple_definition: "int x = 2;" => "[x] int x0 = 2"

simple_declaration: "int y;" => "[y] int x0 = \u{2205} "

scoped_redeclaration: "int y; { int y = 2; }" =>
"[y] int x0 = \u{2205} 
[y] int x1 = 2"

unscoped_redefinition: "{ int y = 2; int y = 3; }" =>
":1:18: error: Redefinition of variable y
    1 | { int y = 2; int y = 3; }
                         ^
"

definition_after_declaration: "int y; int y = 2;" => "[y] int x0 = 2"

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

multiple_declarations: "int x; int x = NULL; int x; int x;" => "[x] int x0 = NULL"

function_declaration:
"const char* func(static volatile int** first_argument, struct custom * arg2)"
=>
"[func] f0(static volatile int * *, struct custom *) -> const char * ;"

function_definition: "int**f(int v) {}" => "[f] f0(int) -> int * * \u{2205} "

function_def_after_decl: "int f(int v); int f(int v) {} int f(int v);" => "[f] f0(int) -> int \u{2205} "

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

function_return: "int glob() { return 1; }" =>
"[glob] f0() -> int
  BB0:
    return x1
[] const int x1 = 1"

hello_world: r#"void printf(const char* s); int main() { printf("Hello, world!"); return 1; }"# =>
r#"[printf] f0(const char *) -> void ;
[main] f1() -> int
  BB0:
    call f0(x2)
    return x3
[] const char * const x2 = "Hello, world!"
[] const int x3 = 1"#


check_id_not_skipped: "int x; int x = 2; int x; int y;" =>
"[x] int x0 = 2
[y] int x1 = ∅ "

call_undeclared: "int f() { g(1) }" =>
":1:11: error: Call of undeclared function g
    1 | int f() { g(1) }
                  ^
"

);
