crate::make_string_error_tests!(

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

nomad_else:
    "else"
    =>
":1:1: parser error: Found nomad `else` without `if`.
    1 | else
        ^~~~
"

);
