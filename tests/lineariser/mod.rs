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

literal_definition: "int x = 2;" => &symb(&["int x0 = 2"])

literal_declaration: "int y;" => &symb(&["int x0 = \u{2205} "])


literal_valid_redeclaration: "int y; { int y = 2; }" => &symb(&["int x0 = \u{2205} ", "int x1 = 2"])

literal_invalid_redeclaration: "{ int y = 2; int y = 3; }" =>
":1:18: error: Redefinition of variable y
    1 | { int y = 2; int y = 3; }
                         ^
"

literal_definition: "int y; int y = 2;" => &symb(&["int x0 = 2"])

literal_definition_wrong_type: "int y; char y = 2;" =>
":1:13: error: Defining declared variable y with a different type
    1 | int y; char y = 2;
                    ^
"

function: "const char* func() {}" => &symb(&["f0() -> const char * ;"])

);
