crate::ast!(

successive_multiline_string:
    "
    blob 
    
    \"multi\"
     \"line\\
     strings\"
    "

escape_eol: "\\ "

invalid_char: "$"

escape_out_ctx: "\\a"

char_2_chars:
    "'ab'"

empty_unclosed_char:
    "'"

escape_non_escapable_char: "'\\z'"

escape_missing_hex: "'\\x'"

escape_too_big_octal: "'\\765'"

escape_missing_short: "'\\u1'"

escape_missing_many_long: "'\\U1'"

escape_missing_one_long: "'\\U0000000'"

escape_not_char_long: "\"\\Uffffffff\""

escape_not_char_long2: "\"\\Uffffffffo\""

escape_not_char_short: "'\\uD900'"

digraphs:
    "
%:include <stdio.h>
??=include <stdio.h>
"

trigraphs:
    "
char b??(5??) = ??< 'b', 'l', 'o',??/
                    'b', '\0' ??>;
int x = 1 ??' ??- 2 ??! 3;
    "

);
