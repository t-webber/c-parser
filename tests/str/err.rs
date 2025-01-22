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

#[rustfmt::skip]
make_string_error_tests!(

invalid_char:
    "#"
    =>
":1:1: lexer error: Character '#' not supported.
    1 | #
        ^
"

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
    "
    =>
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

escape_eol:
    "\\ "
    =>
":1:2: lexer suggestion: Found whitespace after '\\' at EOL. Please remove the space.
    1 | \\ 
         ^
"

suggestion_then_error:
    "f(x,) )"
    =>
":1:2: parser suggestion: Found extra comma in function argument list. Please remove the comma.
    1 | f(x,) )
         ^
:1:7: parser error: Mismatched ')'. Perhaps you forgot an opening '('?
    1 | f(x,) )
              ^
"

in_parens:
    "(static_assert const)"
    =>
":1:16: parser error: Can't push attribute to full variable
    1 | (static_assert const)
                       ^~~~~
"

empty_digit:
    "0x"
    =>
":1:1: lexer error: Invalid number constant: found no digits between prefix and suffix. Please add at least one digit.
    1 | 0x
        ^~
"

signed_unsigned:
    "-1u"
    =>
":1:2: lexer warning: Found an unsigned constant after a negative sign. Consider removing the `u` prefix.
    1 | -1u
         ^~
"

overflow:
    "0xffffffffffffffffffffffffffffffffffffffffffffff"
    =>
":1:1: lexer error: Overflow: 0xffffffffffffffffffffffffffffffffffffffffffffff is too big in traditional number
    1 | 0xffffffffffffffffffffffffffffffffffffffffffffff
        ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
"

overflow_warning:
    "0xffffffffffff.fp2"
    =>
":1:1: lexer warning: Overflow: 0xffffffffffff.fp2 is too big in traditional number
    1 | 0xffffffffffff.fp2
        ^~~~~~~~~~~~~~~~~~
"

escape_out_ctx:
    "\\a"
    =>
":1:1: lexer error: Escape characters are only authorised in strings or chars.
    1 | \\a
        ^
"

char_2_chars:
    "'ab'"
    =>
":1:3: lexer error: A char must contain only one character.
    1 | 'ab'
          ^
"

plus_trigraph:
    "+??'"
    =>
":1:2: lexer warning: Trigraphs are deprecated in C23. Please remove them: replace '??'' by '^'.
    1 | +??'
         ^~~
"

invalid_exponent:
    "0xf.fpa"
    => 
":1:1: lexer error: Invalid number constant: invalid character for exponent. Expected an ascii digit, but found 'a'
    1 | 0xf.fpa
        ^~~~~~~
"

empty_hex:
    "0x"
    =>
":1:1: lexer error: Invalid number constant: found no digits between prefix and suffix. Please add at least one digit.
    1 | 0x
        ^~
"

invalid_char_octal:
    "0z"
    => 
":1:1: lexer error: Invalid number constant: found illegal character 'z' in octal representation.
    1 | 0z
        ^~
"
invalid_char_number:
    "00z"
    => 
":1:1: lexer error: Invalid number constant: found invalid character 'z' in octal base.
    1 | 00z
        ^~~
"

float_binary:
    "0b1."
    =>
":1:1: lexer error: Invalid number constant: a binary must be an integer.
    1 | 0b1.
        ^~~~
"

long_float:
    "0.fl"
    =>
":1:1: lexer error: Invalid number constant: a `float` can't be `long`. Did you mean `long double`? Remove the leading 'f' if that is the case.
    1 | 0.fl
        ^~~~
"

float_not_double:
    "0f"
    =>
":1:1: lexer error: Invalid number constant: a 'f' suffix only works on `double` constants. Please insert a full stop or an 'e' exponent character before the 'f'.
    1 | 0f
        ^~
"

hex_float_without_exp:
    "0xf.f"
    =>
":1:1: lexer error: Hexadecimal float must contain exponent after full stop. Please add missing 'p'.
    1 | 0xf.f
        ^~~~~
"

);
