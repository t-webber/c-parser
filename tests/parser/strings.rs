crate::make_string_error_tests!(

successive_multiline_string:
    "
    blob 
    
    \"multi\"
     \"line\\
     strings\"
    "
    =>
    r#"
:4:4: error: Found 2 consecutive literals: block [blob..] followed by "multiline     strings".
:4:4: error: Multi-line error occurred. Starts here...
    4 |     "multi"
           ^~~~~~~~
:6:13: error: ...and ends here.
    6 |      strings"
                    ^

"#

escape_eol:
    "\\ "
    =>
":1:2: suggestion: Found whitespace after '\\' at EOL. Please remove the space.
    1 | \\ 
         ^
"
);

crate::make_string_error_tests!(

invalid_char:
    "$"
    =>
":1:1: error: Character '$' not supported.
    1 | $
        ^
"

escape_out_ctx:
    "\\a"
    =>
":1:1: error: Escape characters are only authorised in strings or chars.
    1 | \\a
        ^
"

char_2_chars:
    "'ab'"
    =>
":1:3: error: A char must contain only one character.
    1 | 'ab'
          ^
"


empty_unclosed_char:
    "'"
    =>
":1:2: error: Found an empty char, but chars must contain one character. Did you mean '\\''?
    1 | '
         ^
"

escape_non_escapable_char: "'\\z'" =>
":1:2: warning: Escape ignored. Escaping character 'z' has no effect. Please remove the '\\'.
    1 | '\\z'
         ^~
"

escape_missing_hex: "'\\x'" =>
":1:2: error: invalid hexdigit ': expected 1 hexdigit after \\x prefix, but only got 0
    1 | '\\x'
         ^~
"


escape_too_big_octal: "'\\765'" =>
":1:2: warning: octal value too big: exceeds 0o377: will be computed modulo 255
    1 | '\\765'
         ^~~~
"

escape_missing_short: "'\\u1'" =>
":1:2: error: invalid hexdigit ': expected 4 hexdigits after \\u prefix, but only got 1
    1 | '\\u1'
         ^~~
"

escape_missing_many_long: "'\\U1'" =>
":1:2: error: invalid hexdigit ': expected 8 hexdigits after \\U prefix, but only got 1
    1 | '\\U1'
         ^~~
"
escape_missing_one_long: "'\\U0000000'" =>
":1:2: error: invalid hexdigit ': expected 8 hexdigits after \\U prefix, but only got 7
    1 | '\\U0000000'
         ^~~~~~~~~
"

escape_not_char_long: "\"\\Uffffffff\"" =>
r#":1:2: error: escaped sequence expands to 4294967295 which is not a valid char.
    1 | "\Uffffffff"
         ^~~~~~~~~~
"#

escape_not_char_long2: "\"\\Uffffffffo\"" =>
r#":1:2: error: escaped sequence expands to 4294967295 which is not a valid char.
    1 | "\Uffffffffo"
         ^~~~~~~~~~
"#

escape_not_char_short: "'\\uD900'" =>
r":1:2: error: escaped sequence expands to 55552 which is not a valid char.
    1 | '\uD900'
         ^~~~~~
"


digraphs:
    "
%:include <stdio.h>
??=include <stdio.h>
"
    =>
":2:1: error: Found invalid character '#', found by replacing digraph '%:'.
    2 | %:include <stdio.h>
        ^~
:3:1: error: use of trigraphs: replace '??=' by '#'.
    3 | ??=include <stdio.h>
        ^~~
"

trigraphs:
    "
char b??(5??) = ??< 'b', 'l', 'o',??/
                    'b', '\0' ??>;
int x = 1 ??' ??- 2 ??! 3;
    "
    =>
":2:7: error: use of trigraphs: replace '??(' by '['.
    2 | char b??(5??) = ??< 'b', 'l', 'o',??/
              ^~~
:2:11: error: use of trigraphs: replace '??)' by ']'.
    2 | char b??(5??) = ??< 'b', 'l', 'o',??/
                  ^~~
:2:17: error: use of trigraphs: replace '??<' by '{'.
    2 | char b??(5??) = ??< 'b', 'l', 'o',??/
                        ^~~
:2:35: error: use of trigraphs: replace '??/' by '\\'.
    2 | char b??(5??) = ??< 'b', 'l', 'o',??/
                                          ^~~
:3:30: error: use of trigraphs: replace '??>' by '}'.
    3 |                     'b', '\0' ??>;
                                     ^~~
:4:11: error: use of trigraphs: replace '??'' by '^'.
    4 | int x = 1 ??' ??- 2 ??! 3;
                  ^~~
:4:15: error: use of trigraphs: replace '??-' by '~'.
    4 | int x = 1 ??' ??- 2 ??! 3;
                      ^~~
:4:21: error: use of trigraphs: replace '??!' by '|'.
    4 | int x = 1 ??' ??- 2 ??! 3;
                            ^~~
"

);
