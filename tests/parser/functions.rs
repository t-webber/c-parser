crate::ast!(

simple: "main() { a = f(b) + d; }c = true;"

nested_functions: "a = f(b <<= !g(!c) + d);"

functions_blocks: "main() { a = f(b + g(c) + d); } "

keywords_functions: "main() { x = sizeof(align(x)); }"

function_argument_priority: "main(!f(x+y,!u), g(f(h(x,y),z),t),u)"

alignoff: "int x = alignof(int);"

ualignof: "int x = _Alignof(int);"

keywords_attributes_functions_err: "int main() {
    const int volatile static short _Thread_local y;
    static_assert(sizeof(x = 2) + 1 == 2);
    }"

);

crate::ast_no_error!(

keywords_attributes_functions: "int main() {
    const int volatile static short _Thread_local y;
    static_assert(sizeof(x = false) + 1 == 2);
    }"

heavy: "inline _Noreturn long auto extern signed _Atomic _BigInt default unsigned register restrict _Complex _Generic _NoReturn constexpr _Decimal64 _Imaginary _Decimal32 _Decimal128 _AlignAs alignas f();"

);
