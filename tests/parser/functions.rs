crate::ast!(

nested_block_functions:
    "f(a+b) { g(!x) {     a = NULL;     b = 2; } c = 3;
    }
    "
    =>
    "[(f°((a + b))), [(g°((!x))), [(a = NULL), (b = 2), \u{2205} ], (c = 3), \u{2205} ]..]"

simple:
    "main() { a = f(b) + d; }c = true;"
    =>
    "[(main°()), [(a = ((f°(b)) + d)), \u{2205} ], (c = true), \u{2205} ..]"


nested_functions:
    "a = f(b <<= !g(!c) + d);"
    =>
    "[(a = (f°((b <<= ((!(g°((!c)))) + d))))), \u{2205} ..]"


functions_blocks:
    "main() { a = f(b + g(c) + d); } "
    =>
    "[(main°()), [(a = (f°(((b + (g°(c))) + d)))), \u{2205} ]..]"

keywords_functions:
    "main() { x = sizeof(align(x)); }"
    =>
    "[(main°()), [(x = (sizeof°((align°(x))))), \u{2205} ]..]"

function_argument_priority:
    "main(!f(x+y,!u), g(f(h(x,y),z),t),u)"
    =>
    "[(main°((!(f°((x + y), (!u)))), (g°((f°((h°(x, y)), z)), t)), u))..]"


alignoff:
    "int x = alignof(int);"
    =>
    "[(int:(x = (alignof°((int:))))), \u{2205} ..]"

ualignof:
    "int x = _Alignof(int);" =>
":1:9: warning: Underscore operators are deprecated since C23. Consider using the new keyword: alignof
    1 | int x = _Alignof(int);
                ^~~~~~~~
"

keywords_attributes_functions_err:
    "int main() {
    const int volatile static short _Thread_local y;
    static_assert(sizeof(x = 2) + 1 == 2);
    }"
    =>
":2:37: warning: Underscore operators are deprecated since C23. Consider using the new keyword: thread_local
    2 |     const int volatile static short _Thread_local y;
                                            ^~~~~~~~~~~~~
"

);

crate::ast_no_error!(

keywords_attributes_functions:
    "int main() {
    const int volatile static short _Thread_local y;
    static_assert(sizeof(x = 2) + 1 == 2);
    }"
    =>
    "[((int:main)°()), [(const int volatile static short thread_local:y), (static_assert°((((sizeof°((x = 2))) + 1) == 2))), \u{2205} ]..]"

);
