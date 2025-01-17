macro_rules! make_string_error_tests {
    ($($name:ident: $input:expr => $output:expr)*) => {
        mod parser_string_error {
            $(
                #[test]
                fn $name() {
                    super::super::test_string_error($input, $output)
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
