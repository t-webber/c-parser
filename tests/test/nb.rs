crate::make_string_error_tests!(

plus_trigraph:
    "+??'"
    =>
":1:2: lexer warning: Trigraphs are deprecated in C23. Please remove them: replace '??'' by '^'.
    1 | +??'
         ^~~
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

overflow_exp:
    "0x0.0p999999999999999999"
    =>
":1:1: lexer error: Failed to parse exponent: too large
    1 | 0x0.0p999999999999999999
        ^~~~~~~~~~~~~~~~~~~~~~~~
"

empty_exp:
    "0x0.0p"
    =>
":1:1: lexer error: Invalid number constant: Illegal floating point constant: found empty exponent, but at least one digit was expected.
    1 | 0x0.0p
        ^~~~~~
"

overflow_unsigned:
    "999999999999999999999u
    -999999999999999999999"
    =>
":1:1: lexer error: Overflow: 999999999999999999999u is too big in traditional number
    1 | 999999999999999999999u
        ^~~~~~~~~~~~~~~~~~~~~~
:2:6: lexer error: Overflow: 999999999999999999999 is too big in traditional number
    2 |     -999999999999999999999
             ^~~~~~~~~~~~~~~~~~~~~
"


invalid_suffix:
"1uu
2lll
3i
4.ll
5.l
6.fu
7.u
"=>
":1:1: lexer error: found 2 'u' characters.
    1 | 1uu
        ^~~
:2:1: lexer error: found 3 'l' characters, but max is 2 (`long long`).
    2 | 2lll
        ^~~~
:3:1: lexer error: imaginary constants are a GCC extension.
    3 | 3i
        ^~
:4:1: lexer error: Invalid number constant: `long long double` doesn't exist.
    4 | 4.ll
        ^~~~
:5:1: lexer error: Invalid number constant: `long double` not supported yet.
    5 | 5.l
        ^~~
:6:1: lexer error: Invalid number constant: a `float` can't be `unsigned`.
    6 | 6.fu
        ^~~~
:7:1: lexer error: Invalid number constant: a `double` can't be `unsigned`.
    7 | 7.u
        ^~~
"

);
