crate::ast!(

char_array:
    "char x[4] = {'b', 12+'5', '3', '\0' };"
    =>
    "[(((char:x)[4]) = {'b', (12 + '5'), '3', '\0'}), \u{2205} ..]"

indirection:
    "int *a *b = *c * d + e"
    =>
    "[(int * a *:(b = (((*c) * d) + e)))..]"

array_access:
    "*a->b[3] = c[3].d[1]"
    =>
    "[((*((a -> b)[3])) = (((c[3]) . d)[1]))..]"

multiline_string:
    "\"multi\"
     \"line\\
     strings\"
    "
    =>
    "[\"multiline     strings\"..]"

cast:
    "(type)x"
    =>
    "[((type)°x..)..]"

cast_ptr:
    "(int)&x"
    =>
    "[((int)°(&x)..)..]"

cast_str:
    "(void*)\"Hello World\""
    =>
    "[((void *)°\"Hello World\"..)..]"

cast_expr:
    "(double)(x+++y)"
    =>
    "[((double)°(((x++) + y)))..]"

cast_int:
    "(int)-1;"
    =>
    "[((int)°(-1)), \u{2205} ..]"

cast_struct_access:
    "(float)data.int_val"
    =>
    "[((float)°(data . int_val)..)..]"

cast_higher_precedence:
    "(float)x+y"
    =>
    "[(((float)°x..) + y)..]"

escape_ok: "\"\\111\"" => "[\"I\"..]"
escape_0: "'\0'" => "['\0'..]"
escape_257: "'\\402'" => "['\u{2}'..]"
escape_255: "'\\377'" => "['\u{ff}'..]"
escape_4_digits: "\"\\45079\"" => "[\"(79\"..]"


custom_indirection_assign:
    "a*b = c"
    =>
    "[((a * b) = c)..]"

block_comment:
"/*
 *  hello
 * world
 */
"
    => "[..]"

underscore:
    "!_a_"
    =>
    "[(!_a_)..]"

signed_number:
    "-42"
    =>
    "[(-42)..]"

escape_in_string:
    "\" \\0 \\a \\b \\t \\n \\v \\f \\r \\e \\\" \\' \\? \\\\ \\u0192 \\U00100009 \\x1029 \\123 \""
    =>
    "[\" \0 \u{7} \u{8} \t \n \u{b} \u{c} \r \u{1b} \" ' ? \\ ƒ \u{100009} \u{29} S \"..]"

mul_assign: // TODO: this isn't supposed to work for the general c, only for c qualifier (const,
            // volatile, restrict, atomic, etc.)
    "b * c d = 0"
    =>
    "[((b * c:d) = 0)..]"

bitfield: "struct { int a : 2 }; " =>
"[<struct \u{2205}  [(int:(a:2))]>, \u{2205} ..]"

);
