crate::make_string_error_tests!(

lengths_literal:
    "x = 'c' blob;"
    =>
":1:9: error: Found 2 consecutive literals: block [(x = 'c')..] followed by blob.
    1 | x = 'c' blob;
                ^~~~
"

lengths_symbols:
    "<<="
    =>
":1:1: error: Tried to call binary operator <<= on without a left argument.
    1 | <<=
        ^~~
"

suggestion_then_error:
    "f(x,) )"
    =>
":1:2: suggestion: Found extra comma in function argument list. Please remove the comma.
    1 | f(x,) )
         ^
:1:7: error: Mismatched ')'. Perhaps you forgot an opening '('?
    1 | f(x,) )
              ^
"

in_parens:
    "(static_assert const)"
    =>
":1:16: error: Found attribute const after function keyword static_assert, but this is not allowed.
    1 | (static_assert const)
                       ^~~~~
"

nomad_else:
    "else"
    =>
":1:1: error: Found nomad `else` without `if`.
    1 | else
        ^~~~
"

nomad_brace:
    "{"
    =>
":1:1: error: Mismatched '{': reached end of block. Perhaps you forgot a closing '}'?
    1 | {
        ^
"

nomad_bracket:
    "a[3]]"
    =>
":1:5: error: Mismatched ']'. Perhaps you forgot an opening '['?
    1 | a[3]]
            ^
"

invalid_keyword:
    "const sizeof *x = 1;"
    =>
":1:7: error: Invalid keyword in current context. Perhaps a missing ';'
    1 | const sizeof *x = 1;
              ^~~~~~
"

successive_numbers: "a 2" =>
":1:3: error: Found 2 consecutive literals: block [a..] followed by 2.
    1 | a 2
          ^
"


successive_numbers_long: "a 22222" =>
":1:3: error: Found 2 consecutive literals: block [a..] followed by 22222.
    1 | a 22222
          ^~~~~
"

two_colons: "const x : :" =>
":1:11: error: found 2 successive colons in struct declaration
    1 | const x : :
                  ^
"

sizeof_bitfield: "sizeof :" =>
":1:8: error: found `:` after keyword sizeof: colon is only valid after user-defined label
    1 | sizeof :
               ^
"
comma_colon: "const x, :" =>
":1:10: error: Expected variable name, found `:`
    1 | const x, :
                 ^
"

declaration_operator: "const int a +" =>
":1:13: error: Can't push operator in empty declaration: missing `=`.
    1 | const int a +
                    ^
"

bitfield_number_name: "const int a : 2 name" =>
":1:17: error: Found unexpected identifier after bitfield specifier
    1 | const int a : 2 name
                        ^~~~
"

bitfield_operator: "const int a : +" =>
":1:15: error: Found operator after bitfield specifier but this is not allowed
    1 | const int a : +
                      ^
"


bitfield_not_number: "const int a : 'b'" =>
":1:15: error: Expected bitfield size, but `:` is followed by a non-number token
    1 | const int a : 'b'
                      ^~~
"

);
