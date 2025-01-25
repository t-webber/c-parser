#[rustfmt::skip]
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
:4:4: parser error: Found 2 consecutive literals: block [blob..] followed by "multiline     strings".
:4:4: parser error: Multi-line error occurred. Starts here...
    4 |     "multi"
           ^~~~~~~~
:6:13: parser error: ...and ends here.
    6 |      strings"
                    ^

"#

escape_eol:
    "\\ "
    =>
":1:2: lexer suggestion: Found whitespace after '\\' at EOL. Please remove the space.
    1 | \\ 
         ^
"
);

crate::make_string_error_tests!(

invalid_char:
    "#"
    =>
":1:1: lexer error: Character '#' not supported.
    1 | #
        ^
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


empty_unclosed_char:
    "'"
    =>
":1:2: lexer error: Found an empty char, but chars must contain one character. Did you mean '\\''?
    1 | '
         ^
"

invalid_escape:
    "
    '\\z'
    '\\u1'
    '\\777'
    \"\\U99999999\"
    '\\U1'
    '\\uD900'
    '\\U0000000'
    '\\x'
    "
    =>
":2:6: lexer warning: Escape ignored. Escaping character 'z' has no effect. Please remove the '\\'.
    2 |     '\\z'
             ^~
:3:6: lexer error: Invalid escaped short unicode number: must contain at least 4 digits, but found only 1
    3 |     '\\u1'
             ^~~
:4:6: lexer error: Escape sequence was too long, creating more than one character, but it doesn't fit into a char.
    4 |     '\\777'
             ^~~
:5:6: lexer error: Invalid escape character code
    5 |     \"\\U99999999\"
             ^~~~~~~~~~
:6:6: lexer error: Invalid escaped unicode number: An escaped big unicode must contain 8 hexadecimal digits, found only 1. Did you mean to use lowercase \\u?
    6 |     '\\U1'
             ^~~
:7:6: lexer error: Invalid escape character code
    7 |     '\\uD900'
             ^~~~~~
:8:6: lexer error: Invalid escaped unicode number: must contain at least 8 digits, but found only 7
    8 |     '\\U0000000'
             ^~~~~~~~~
:9:6: lexer error: Invalid escaped hexadecimal number: must contain at least 1 digits, but found only 0
    9 |     '\\x'
             ^~
"

digraphs:
    "
%:include <stdio.h>
??=include <stdio.h>
"
    =>
":2:1: lexer error: Found invalid character '#', found by replacing digraph '%:'.
    2 | %:include <stdio.h>
        ^~
:3:1: lexer warning: Trigraphs are deprecated in C23. Please remove them: replace '??=' by '#'.
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
