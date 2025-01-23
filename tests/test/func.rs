crate::make_string_tests!(


nested_block_functions:
    "f(a+b) { g(!x) {     a = 1;     b = 2; } c = 3;
    }
    "
    =>
    "[(f°((a + b))), [(g°((!x))), [(a = 1), (b = 2), \u{2205} ], (c = 3), \u{2205} ]..]"

simple:
    "main() { a = f(b) + d; }c = 3;"
    =>
    "[(main°()), [(a = ((f°(b)) + d)), \u{2205} ], (c = 3), \u{2205} ..]"


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

keywords_attributes_functions:
    "int main() {
    const int volatile static short _Thread_local y;
    static_assert(sizeof(x = 2) + 1 == 2);
    }"
    =>
    "[((int:main)°()), [(const int volatile static short thread_local:y), (static_assert°((((sizeof°((x = 2))) + 1) == 2))), \u{2205} ]..]"


function_argument_priority:
    "main(!f(x+y,!u), g(f(h(x,y),z),t),u)"
    =>
    "[(main°((!(f°((x + y), (!u)))), (g°((f°((h°(x, y)), z)), t)), u))..]"



);
