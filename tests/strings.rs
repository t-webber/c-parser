use c_parser::*;

const SEP: &str = "\n--------------------\n";

fn test_string(content: &str, output: &str) {
    let files = &[(String::new(), content)];
    let mut location = Location::from(String::new());
    eprint!("{SEP}Content = {content}{SEP}");
    let tokens = lex_file(content, &mut location).unwrap_or_display(files, "lexer");
    eprint!("{SEP}Tokens = {}{SEP}", display_tokens(&tokens));
    let node = parse_tokens(tokens).unwrap_or_display(files, "parser");
    let displayed = format!("{node}");
    assert!(
        output == displayed,
        "{SEP}Mismatch! Expected:\n!{output:?}!\n!= Computed\n!{displayed:?}!{SEP}Len o = {} | Len d = {}{SEP}",
        output.len(),
        displayed.len()
    );
}

fn test_string_error(content: &str, output: &str) {
    let files = &[(String::new(), content)];
    let mut location = Location::from(String::new());
    let res = lex_file(content, &mut location);
    let displayed = if res.errors_empty() {
        let tokens = res.unwrap_or_display(files, "lexer");
        parse_tokens(tokens).get_displayed_errors(files, "parser")
    } else {
        res.get_displayed_errors(files, "lexer")
    };
    assert!(
        output == displayed,
        "{SEP}Mismatch! Expected:\n!{output}!\n!= Computed\n!{displayed}!{SEP}Len o = {} | Len d = {}{SEP}",
        output.len(),
        displayed.len()
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
    arr<:1:> = 42;              // Equivalent to arr[1] = 42;
    "
    =>
    "[(((int:arr)[3]) = {1, 2, 3}), ((arr[1]) = 42), \u{2205} ..]"

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
    "[(((char:x)[4]) = {'b', (12 + '5'), '3', '\0'}), \u{2205} ..]"

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
    "[(f°(x, (y + 2))), [(a = 1), \u{2205} , [(b = 2)]], (c = 3)..]"


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
    "[((int:main)°()), [(const int volatile static short thread_local:y), (static_assert°((((sizeof°((x = 2))) + 1) == 2))), \u{2205} ]..]"

indirection:
    "int *a *b = *c * d + e"
    =>
    "[(int * a *:(b = (((*c) * d) + e)))..]"

operators_assign:
    "a + b ? c * !e : (d = x[3])"
    =>
    "[((a + b) ? (c * (!e)) : ((d = (x[3]))))..]"

function_argument_priority:
    "main(!f(x+y,!u), g(f(h(x,y),z),t),u)"
    =>
    "[(main°((!(f°((x + y), (!u)))), (g°((f°((h°(x, y)), z)), t)), u))..]"

for_loops:
    "for(int i = 0; i < 9+1; i++) printf(\"i = %d\", i);"
    =>
    "[<for ([(int:(i = 0)), (i < (9 + 1)), (i++)]) (printf°(\"i = %d\", i))..>, \u{2205} ..]"

structs:
    "struct A { int x };
    struct A a;"
    =>
    "[<struct A [(int:x)]>, (struct A:a), \u{2205} ..]"

successive_ctrl_flow:
    "break;
    return 0*1;
    for(int x = 2; x<10;x++) x"
    =>
    "[<break>, <return (0 * 1)>, <for ([(int:(x = 2)), (x < 10), (x++)]) x..>..]"

conditional_simple:
    "if (a) b else if (c) d else e; if(x) y;z"
    =>
    "[<if (a) b else <if (c) d else e>>, <if (x) y.\u{b2}.>, z..]"

nested_conditional:
    "if (z) x * y else if (!c) {if (x*y << 2) {x} else {4}}"
    =>
    "[<if (z) (x * y) else <if ((!c)) [<if (((x * y) << 2)) [x] else [4]>].\u{b2}.>>..]"

conditional_return:
    "if (a) return b; else return c; return d"
    =>
    "[<if (a) <return b> else <return c>>, <return d..>..]"

conditional_operators:
    "if (z) x * y else if (!c) {if (x*y << 2) return x; else return 4;}"
    =>
    "[<if (z) (x * y) else <if ((!c)) [<if (((x * y) << 2)) <return x> else <return 4>>].\u{b2}.>>..]"

iterators:
    "while (1) for (int x = 1; x<CONST;  x++) if (x) return a<<=2, 1+a; else continue;"
    =>
    "[<while (1) <for ([(int:(x = 1)), (x < CONST), (x++)]) <if (x) <return ((a <<= 2) , (1 + a))> else <continue>>..>..>..]"

empty_block:
    "if (a) {} else {}"
    =>
    "[<if (a) [] else []>..]"

break_inside_nested_loops:
    "for(int i = 0; i < 3; i++) { for(int j = 0; j < 3; j++) { if(i% 4==2) break; } }"
    =>
    "[<for ([(int:(i = 0)), (i < 3), (i++)]) [<for ([(int:(j = 0)), (j < 3), (j++)]) [<if (((i % 4) == 2)) <break>.\u{b2}.>]>]>..]"

nested_if_else:
    "if(a) { if(b) c = 1; else c = 2; } else { if(d) c = 3; else c = 4; }"
    =>
    "[<if (a) [<if (b) (c = 1) else (c = 2)>] else [<if (d) (c = 3) else (c = 4)>]>..]"


logical_operators_conditional:
    "if(a && b || c) x = 1; else y = 2;"
    =>
    "[<if (((a && b) || c)) (x = 1) else (y = 2)>..]"


empty_else_block:
    "if(a) x = 1; else ;"
    =>
    "[<if (a) (x = 1) else \u{2205} >..]"

while_loop_with_break:
    "while(a) { if(b) break; c = 5; }"
    =>
    "[<while (a) [<if (b) <break>.\u{b2}.>, (c = 5), \u{2205} ]>..]"

switch_case:
    "switch(x) { case 1: y = 2; break; case 2: y = 3; default: y = 4; }"
    =>
    "[<switch (x) [<case 1: [(y = 2), <break>, \u{2205} ..]..>, <case 2: [(y = 3), \u{2205} ..]..>, <default: [(y = 4), \u{2205} ..]..>]>..]"

nested_loops_with_control_flow:
    "for(int i = 0; i < 3; i++)
        for(int j = 1; j < 4; j++) {
            if(i == j) continue;
            printf(\"i = %d, j = %d\", i, j);
        }
    "
     =>
    "[<for ([(int:(i = 0)), (i < 3), (i++)]) <for ([(int:(j = 1)), (j < 4), (j++)]) [<if ((i == j)) <continue>.\u{b2}.>, (printf°(\"i = %d, j = %d\", i, j)), \u{2205} ]>>..]"

 continue_inside_for_loop:
    "for(int i = 0; i < 5; i++)
        { if(i == 3) continue; printf(\"%d\", i); x}
    y"
    =>
    "[<for ([(int:(i = 0)), (i < 5), (i++)]) [<if ((i == 3)) <continue>.\u{b2}.>, (printf°(\"%d\", i)), x]>, y..]"

multiple_variables:
    "int i = 1, j = 2;"
    =>
    "[(int:(i = 1), (j = 2)), \u{2205} ..]"

for_loop_multiple_variables:
    "for(int i = 0, j = 5; i < 10 && j > 0; i++, j--)
        printf(\"i = %d, j = %d\", i, j);"
    =>
    "[<for ([(int:(i = 0), (j = 5)), ((i < 10) && (j > 0)), ((i++) , (j--))]) (printf°(\"i = %d, j = %d\", i, j))..>, \u{2205} ..]"

typedef_struct_definition:
    "typedef struct a { int x[]; const *volatile *int y; } b"
    =>
    "[<typedef <struct a [((int:x)[]), (const * volatile * int:y), \u{2205} ]> b>..]"

typedef_struct:
    "typedef struct a b"
    =>
    "[<typedef (struct a:b)>..]"

typedef_int:
    "typedef const int *c"
    =>
    "[<typedef (const int *:c)>..]"

array_access:
    "*a->b[3] = c[3].d[1]"
    =>
    "[((*((a -> b)[3])) = (((c[3]) . d)[1]))..]"

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
    "
:1:9: parser error: Found 2 consecutive literals: block [(x = 'c')..] followed by blob.
    1 | x = 'c' blob;
                ^~~~
"

lengths_symbols:
    "<<="
    =>
    "
:1:1: parser error: Tried to call binary operator <<= on without a left argument.
    1 | <<=
        ^~~
"

digraphs:
    "%:include <stdio.h>"
    =>
    "
:1:1: lexer error: Found invalid character '#', found by replacing digraph '%:'.
    1 | %:include <stdio.h>
        ^~
"

trigraphs:
    "
char b??(5??) = ??< 'b', 'l', 'o',??/
                    'b', '\0' ??>;
int x = 1 ??' ??- 2 ??! 3;
    "
    =>
    "
:2:7: lexer warning: Trigraphs are deprecated in C23. Please remove them: replace '??(' by '['.
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
