use std::fs;

use c_parser::*;

const SEP: &str = "\n--------------------\n";

fn test_string(content: &str, output: &str) {
    let files = &[(String::new(), content)];
    let mut location = Location::from(String::new());
    eprintln!("{SEP}Content = {content}{SEP}");
    let tokens = lex_file(content, &mut location).unwrap_or_display(files, "lexer");
    eprintln!("{SEP}Tokens = {}{SEP}", display_tokens(&tokens));
    let node = parse_tokens(tokens, "".to_owned()).unwrap_or_display(files, "parser");
    assert!(
        output == format!("{node}"),
        "{SEP}Mismatch! Expected:\n{output}\n!= Computed\n{node}{SEP}"
    );
}

fn test_string_error(content: &str, output: &str) {
    let files = &[(String::new(), content)];
    let mut location = Location::from(String::new());
    let res = lex_file(content, &mut location);
    let displayed = if res.errors_empty() {
        let tokens = res.unwrap_or_display(files, "lexer");
        parse_tokens(tokens, "".to_owned()).get_displayed_errors(files, "parser")
    } else {
        res.get_displayed_errors(files, "lexer")
    };
    fs::write("displayed.txt", &displayed).unwrap();
    fs::write("expected.txt", output).unwrap();
    assert!(
        output == displayed,
        "Mismatch! Expected:\n!{output}!\n!= Computed\n!{displayed}!"
    );
}

macro_rules! make_string_tests {
    ($($name:ident: $input:expr => $output:expr)*) => {
        mod parser_string {
            $(
                #[test]
                fn $name() {
                    super::test_string($input, $output)
                }
            )*
        }

    };
}

make_string_tests!(

digraphs:
    "
    int arr<:3:> = <%1, 2, 3%>; // Equivalent to int arr[3];
    arr<:1:> = 42;            // Equivalent to arr[1] = 42;
    "
    =>
    "[(((int arr)[3]) = {1, 2, 3}), ((arr[1]) = 42), \u{2205} ..]"

multiline_string:
    "\"multi\"
     \"line\\
     strings\"
    "
    =>
    "[\"multiline     strings\"..]"

unary_binary:
    "a + b * c - d / e % f + g - h * i + j % k * l ^ !m++ & n | o || p && q"
    =>
    "[((((((((a + (b * c)) - ((d / e) % f)) + g) - (h * i)) + ((j % k) * l)) ^ ((!(m++)) & n)) | o) || (p && q))..]"

ternary_blocks:
    "a * b + c - d / e % f * g + h & i | j ^ k && l ||
        m * n + o - p * q / r + s % t
        ? u
        : v && w ^ x | y && z; !a"
    =>
    "[(((((((((a * b) + c) - (((d / e) % f) * g)) + h) & i) | (j ^ k)) && l) || ((((m * n) + o) - ((p * q) / r)) + (s % t))) ? u : ((v && ((w ^ x) | y)) && z)), (!a)..]"

parens_assign:
    "ex2 = a * (b + c - d / e % f * g) +
                          (h > i ? j : k) * (l && m || n ^ o) / (p ? q : r) +
                          t &
                      u |
                  v &&
              w
          ? x
          : y ^ z"
    =>
    "[(ex2 = (((((((a * (((b + c) - (((d / e) % f) * g)))) + (((((h > i) ? j : k)) * (((l && m) || (n ^ o)))) / ((p ? q : r)))) + t) & u) | v) && w) ? x : (y ^ z)))..]"

list_initialiser:
    "n[3][3] = {{1, 2, 3}[2 + !3 * m[3]], {1, 2, 3}[2 + 1] + 2};"
    =>
    "[(((n[3])[3]) = {({1, 2, 3}[(2 + ((!3) * (m[3])))]), (({1, 2, 3}[(2 + 1)]) + 2)}), \u{2205} ..]"

nested_parens_bracket:
    "n[3][(3+(1+2))]={{1,2,3}[2+!3*m[m[(a+m[(2)])]]],{1,2,3}[2+1]+2}"
    =>
    "[(((n[3])[((3 + ((1 + 2))))]) = {({1, 2, 3}[(2 + ((!3) * (m[(m[((a + (m[(2)])))])])))]), (({1, 2, 3}[(2 + 1)]) + 2)})..]"

nested_braces:
    "{
    ;
    ;//test
    ;/*on nested*/
    ;///braces
    {
        a=1;
        b=2;
    };
    c=3;
}"
    =>
    "[[\u{2205} , \u{2205} , \u{2205} , \u{2205} , [(a = 1), (b = 2), \u{2205} ], (c = 3), \u{2205} ]..]"

char_array:
    "char x[4] = {'b', 12+'5', '3', '\0' };"
    =>
    "[(((char x)[4]) = {'b', (12 + '5'), '3', '\0'}), \u{2205} ..]"

nested_block_functions:
        "f(a+b) { g(!x) {     a = 1;     b = 2; } c = 3;
}
"
    =>
    "[(f°((a + b))), [(g°((!x))), [(a = 1), (b = 2), \u{2205} ], (c = 3), \u{2205} ]..]"

functions:
        "main() { a = f(b) + d; }c = 3;"
    =>
    "[(main°()), [(a = ((f°(b)) + d)), \u{2205} ], (c = 3), \u{2205} ..]"

blocks:
        "f(x, y + 2) {
    a = 1;
    { b = 2U }
}
c = 3  "
    =>
    "[(f°((x , (y + 2)))), [(a = 1), \u{2205} , [(b = 2)]], (c = 3)..]"


nested_functions:
    "a = f(b <<= !g(!c) + d);"
    =>
    "[(a = (f°((b <<= ((!(g°((!c)))) + d))))), \u{2205} ..]"


functions_blocks:
    "main() { a = f(b + g(c) + d); } "
    =>
    "[(main°()), [(a = (f°(((b + (g°(c))) + d)))), \u{2205} ]..]"

keywords_functions:
    "main() { x = sizeof(align(x)); }"
    =>
    "[(main°()), [(x = (sizeof°((align°(x))))), \u{2205} ]..]"

keywords_attributes_functions:
    "int main() {
    const int volatile static short _Thread_local y;
    static_assert(sizeof(x = 2) + 1 == 2);
    }"
    =>
    "[((int main)°()), [(const int volatile static short thread_local y), (static_assert°((((sizeof°((x = 2))) + 1) == 2))), \u{2205} ]..]"

indirection:
    "int *a *b = *c * d + e"
    =>
    "[((int * 'a' * b) = (((*c) * d) + e))..]"

operators_assign:
    "a + b ? c * !e : d = x[3]"
    =>
    "[(((a + b) ? (c * (!e)) : d) = (x[3]))..]"


);

macro_rules! make_string_error_tests {
    ($($name:ident: $input:expr => $output:expr)*) => {
        mod parser_string_error {
            $(
                #[test]
                fn $name() {
                    super::test_string_error($input, $output)
                }
            )*
        }

    };
}

make_string_error_tests!(

lengths_literal:
"x = 'c' blob;"
=>
":1:9: parser error: Found 2 consecutive literals: block [(x = 'c')..] followed by blob.
    1 | x = 'c' blob;
                ^~~~
"

lengths_symbols:
    "<<="
    =>
":1:1: parser error: Tried to call binary operator <<= on without a left argument.
    1 | <<=
        ^~~
"

digraphs:
    "%:include <stdio.h>"
    =>
":1:1: lexer error: Found invalid character '#', found by replacing digraph '%:'.
    1 | %:include <stdio.h>
        ^~
"

trigraphs:
    "
char b??(5??) = ??< 'b', 'l', 'o',??/
                    'b', '\0' ??>;
int x = 1 ??' ??- 2 ??! 3;
" =>
":2:7: lexer warning: Trigraphs are deprecated in C23. Please remove them: replace '??(' by '['.
    2 | char b??(5??) = ??< 'b', 'l', 'o',??/
              ^~~
:2:11: lexer warning: Trigraphs are deprecated in C23. Please remove them: replace '??)' by ']'.
    2 | char b??(5??) = ??< 'b', 'l', 'o',??/
                  ^~~
:2:17: lexer warning: Trigraphs are deprecated in C23. Please remove them: replace '??<' by '{'.
    2 | char b??(5??) = ??< 'b', 'l', 'o',??/
                        ^~~
:2:35: lexer warning: Trigraphs are deprecated in C23. Please remove them: replace '??/' by '\\'.
    2 | char b??(5??) = ??< 'b', 'l', 'o',??/
                                          ^~~
:3:30: lexer warning: Trigraphs are deprecated in C23. Please remove them: replace '??>' by '}'.
    3 |                     'b', '\0' ??>;
                                     ^~~
:4:11: lexer warning: Trigraphs are deprecated in C23. Please remove them: replace '??'' by '^'.
    4 | int x = 1 ??' ??- 2 ??! 3;
                  ^~~
:4:15: lexer warning: Trigraphs are deprecated in C23. Please remove them: replace '??-' by '~'.
    4 | int x = 1 ??' ??- 2 ??! 3;
                      ^~~
:4:21: lexer warning: Trigraphs are deprecated in C23. Please remove them: replace '??!' by '|'.
    4 | int x = 1 ??' ??- 2 ??! 3;
                            ^~~
"

);
