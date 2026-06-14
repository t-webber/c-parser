fn symb(symbols: &[&'static str]) -> String {
    let mut ret = String::from("Symbols:");
    for sym in symbols {
        ret.push('\n');
        ret.push_str("  ");
        ret.push_str(sym);
    }
    ret
}

crate::ssa!(

definition: "int x = 2;" => &symb(&["int x0 = 2"])

declaration: "int y;" => &symb(&["int x0 = \u{2205} "])

scoped_redeclaration: "int y; { int y = 2; }" => &symb(&["int x0 = \u{2205} ", "int x1 = 2"])

unscoped_redefinition: "{ int y = 2; int y = 3; }" =>
":1:18: error: Redefinition of variable y
    1 | { int y = 2; int y = 3; }
                         ^
"

definition_after_declaration: "int y; int y = 2;" => &symb(&["int x0 = 2"])

definition_wrong_type: "int y; char y = 2;" =>
":1:13: error: Defining declared variable y with a different type
    1 | int y; char y = 2;
                    ^
"

function_declaration:
    "const char* func(static volatile int** first_argument, struct custom * arg2)"
=> &symb(&["f0(static volatile int * *, struct custom *) -> const char * ;"])

function_definition: "int**f(int v) {}" =>
&symb(&["f0(int) -> int * * .."])

);
