crate::make_string_tests!(

digraphs:
    "
    int arr<:3:> = <%1, 2, 3%>; // Equivalent to int arr[3];
    arr<:1:> = 42;              // Equivalent to arr[1] = 42;
    "
    =>
    "[(((int:arr)[3]) = {1, 2, 3}), ((arr[1]) = 42), \u{2205} ..]"


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


blocks:
    "f(x, y + 2) {
        a = 1;
        { b = 2U }
    }
    c = 3  "
    =>
    "[(f°(x, (y + 2))), [(a = 1), \u{2205} , [(b = 2)]], (c = 3)..]"


);

crate::make_string_error_tests!(

open_parens:
    "("
    =>
":1:1: parser error: Mismatched '(': reached end of block. Perhaps you forgot a closing ')'?
    1 | (
        ^
"

open_brace:
    "{"
    =>
":1:1: parser error: Mismatched '{': reached end of block. Perhaps you forgot a closing '}'?
    1 | {
        ^
"

open_bracket:
    "["
    =>
":1:1: parser error: Mismatched '[': reached end of block. Perhaps you forgot a closing ']'?
    1 | [
        ^
"

close_parens:
    ")"
    =>
":1:1: parser error: Mismatched ')'. Perhaps you forgot an opening '('?
    1 | )
        ^
"

close_brace:
    "}"
    =>
":1:1: parser error: Mismatched '}'. Perhaps you forgot an opening '{'?
    1 | }
        ^
"

close_bracket:
    "]"
    =>
":1:1: parser error: Mismatched ']'. Perhaps you forgot an opening '['?
    1 | ]
        ^
"

);
