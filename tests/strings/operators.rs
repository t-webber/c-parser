use crate::make_string_tests;

make_string_tests!(

    digraphs:
    "
    int arr<:3:> = <%1, 2, 3%>; // Equivalent to int arr[3];
    arr<:1:> = 42;              // Equivalent to arr[1] = 42;
    "
    =>
    "[(((int:arr)[3]) = {1, 2, 3}), ((arr[1]) = 42), \u{2205} ..]"

multiline_string:
    "\"multi\"
     \"line\\
     strings\"
    "
    =>
    "[\"multiline     strings\"..]"

unary_binary:
    "a + b * c - d / e % f + g - h * i + j % k * l ^ !m++ & n | o || p && q"
    =>
    "[((((((((a + (b * c)) - ((d / e) % f)) + g) - (h * i)) + ((j % k) * l)) ^ ((!(m++)) & n)) | o) || (p && q))..]"

ternary_blocks:
    "a * b + c - d / e % f * g + h & i | j ^ k && l ||
        m * n + o - p * q / r + s % t
        ? u
        : v && w ^ x | y && z; !a"
    =>
    "[(((((((((a * b) + c) - (((d / e) % f) * g)) + h) & i) | (j ^ k)) && l) || ((((m * n) + o) - ((p * q) / r)) + (s % t))) ? u : ((v && ((w ^ x) | y)) && z)), (!a)..]"

parens_assign:
    "ex2 = a * (b + c - d / e % f * g) +
                          (h > i ? j : k) * (l && m || n ^ o) / (p ? q : r) +
                          t &
                      u |
                  v &&
              w
          ? x
          : y ^ z"
    =>
    "[(ex2 = (((((((a * (((b + c) - (((d / e) % f) * g)))) + (((((h > i) ? j : k)) * (((l && m) || (n ^ o)))) / ((p ? q : r)))) + t) & u) | v) && w) ? x : (y ^ z)))..]"

list_initialiser:
    "n[3][3] = {{1, 2, 3}[2 + !3 * m[3]], {1, 2, 3}[2 + 1] + 2};"
    =>
    "[(((n[3])[3]) = {({1, 2, 3}[(2 + ((!3) * (m[3])))]), (({1, 2, 3}[(2 + 1)]) + 2)}), \u{2205} ..]"

nested_parens_bracket:
    "n[3][(3+(1+2))]={{1,2,3}[2+!3*m[m[(a+m[(2)])]]],{1,2,3}[2+1]+2}"
    =>
    "[(((n[3])[((3 + ((1 + 2))))]) = {({1, 2, 3}[(2 + ((!3) * (m[(m[((a + (m[(2)])))])])))]), (({1, 2, 3}[(2 + 1)]) + 2)})..]"

nested_braces:
    "{
    ;
    ;//test
    ;/*on nested*/
    ;///braces
    {
        a=1;
        b=2;
    };
    c=3;
}"
    =>
    "[[\u{2205} , \u{2205} , \u{2205} , \u{2205} , [(a = 1), (b = 2), \u{2205} ], (c = 3), \u{2205} ]..]"

char_array:
    "char x[4] = {'b', 12+'5', '3', '\0' };"
    =>
    "[(((char:x)[4]) = {'b', (12 + '5'), '3', '\0'}), \u{2205} ..]"

nested_block_functions:
        "f(a+b) { g(!x) {     a = 1;     b = 2; } c = 3;
}
"
    =>
    "[(f°((a + b))), [(g°((!x))), [(a = 1), (b = 2), \u{2205} ], (c = 3), \u{2205} ]..]"

functions:
        "main() { a = f(b) + d; }c = 3;"
    =>
    "[(main°()), [(a = ((f°(b)) + d)), \u{2205} ], (c = 3), \u{2205} ..]"

blocks:
        "f(x, y + 2) {
    a = 1;
    { b = 2U }
}
c = 3  "
    =>
    "[(f°(x, (y + 2))), [(a = 1), \u{2205} , [(b = 2)]], (c = 3)..]"


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

indirection:
    "int *a *b = *c * d + e"
    =>
    "[(int * a *:(b = (((*c) * d) + e)))..]"

operators_assign:
    "a + b ? c * !e : (d = x[3])"
    =>
    "[((a + b) ? (c * (!e)) : ((d = (x[3]))))..]"

function_argument_priority:
    "main(!f(x+y,!u), g(f(h(x,y),z),t),u)"
    =>
    "[(main°((!(f°((x + y), (!u)))), (g°((f°((h°(x, y)), z)), t)), u))..]"


array_access:
    "*a->b[3] = c[3].d[1]"
    =>
    "[((*((a -> b)[3])) = (((c[3]) . d)[1]))..]"

);
