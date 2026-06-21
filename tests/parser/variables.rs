crate::ast!(

char_array: "char x[4] = {'b', 12+'5', '3', '\0' };"

indirection: "int *a *b = *c * d + e"

array_access: "*a->b[3] = c[3].d[1]"

multiline_string: "\"multi\"
     \"line\\
     strings\"
    "

cast: "(type)x"

cast_ptr: "(int)&x"

cast_str: "(void*)\"Hello World\""

cast_expr: "(double)(x+++y)"

cast_int: "(int)-1;"

cast_struct_access: "(float)data.int_val"

cast_higher_precedence: "(float)x+y"

escape_ok: "\"\\111\""

escape_0: "'\0'"

escape_255: "'\\377'"

custom_indirection_assign: "a*b = c"

block_comment: "/*
 *  hello
 * world
 */
"

underscore: "!_a_"

signed_number: "-42"

mul_assign: // TODO: this isn't supposed to work for the general c, only for c qualifier (const,
            // volatile, restrict, atomic, etc.)
    "b * c d = 0"

bitfield: "struct { int a : 2 }; "

escape_4_digits_err: "\"\\45079\""

escape_257_err: "'\\402'"

escape_x_too_long: r#""\x1029293""#

);

crate::ast_no_error!(

escape_4_digits: "\"\\45079\""

escape_257: "'\\402'"

escape_in_string: "\" \\0 \\a \\b \\t \\n \\v \\f \\r \\e \\\" \\' \\? \\\\ \\u0192 \\U00100009 \\x1029 \\123 \""

);
