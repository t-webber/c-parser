use crate::make_string_tests;

make_string_tests!(

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
    "[(type)°x..]"

cast_ptr:
    "(int)&x"
    =>
    "[(int)°(&x)....]"

cast_str:
    "(void*)\"Hello World\""
    =>
    "[(void *)°\"Hello World\"..]"

cast_expr:
    "(double)(x+++y)"
    =>
    "[(double)°(((x++) + y))..]"

);
