use c_parser::*;

const SEP: &str = "\n--------------------\n";

fn test_string(content: &str, output: &str) {
    let files = &[(String::new(), content)];
    let mut location = Location::from(String::new());
    eprintln!("{SEP}Content = {content}{SEP}");
    let tokens = lex_file(content, &mut location).unwrap_or_display(files, "lexer");
    eprintln!("{SEP}Tokens = {}{SEP}", display_tokens(&tokens));
    let node = parse_tokens(tokens).unwrap_or_display(files, "parser");
    assert!(
        output == format!("{node}"),
        "{SEP}Mismatch! Expected:\n{output}\n!= Computed\n{node}{SEP}"
    );
}

// fn test_string_error(content: &str, output: &str) {
//     let files = &[(String::new(), content)];
//     let mut location = Location::from(String::new());
//     let tokens = lex_file(content, &mut location).unwrap_or_display(files,
// "lexer");     eprintln!("Tokens = {}", display_tokens(&tokens));
//     let displayed = parse_tokens(tokens).get_displayed_errors(files,
// "parser");     assert!(
//         output == displayed,
//         "Mismatch! Expected:\n!{output}!\n!= Computed\n!{displayed}!"
//     );
// }

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

parens_asign:
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

);

// macro_rules! make_string_error_tests {
//     ($($name:ident: $input:expr => $output:expr)*) => {
//         mod parser_string_error {
//             $(
//                 #[test]
//                 fn $name() {
//                     super::test_parser_error_on_string($input, $output)
//                 }
//             )*
//         }

//     };
// }
