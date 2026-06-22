crate::ast!(

digraphs:
    "
    int arr<:3:> = <%1, 2, 3%>; // Equivalent to int arr[3];
    arr<:1:> = 42;              // Equivalent to arr[1] = 42;
    "

parens_assign:
    "ex2 = a * (b + c - d / e % f * g) +
    (h > i ? j : k) * (l && m || n ^ o) / (p ? q : r) +
                    t &
                u |
            v &&
        w
    ? x
    : y ^ z"

list_initialiser: "n[3][3] = {{1, 2, 3}[2 + !3 * m[3]], {1, 2, 3}[2 + 1] + 2};"

nested_parens_bracket: "n[3][(3+(1+2))]={{1,2,3}[2+!3*m[m[(a+m[(2)])]]],{1,2,3}[2+1]+2}"

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


blocks:
    "f(x, y) {
        a = 1;
        { b = 2U }
    }
    c = 3  "

list_initialiser_in_body: "a ? {1, 2, 3} : {4, 5, 6}"
list_initialiser_unary: "!{1, 2, 3}"

list_init_cast: "(a)b{c}"
list_init_cast_full: "(a)(b){c}"

open_parens: "("
open_brace: "{"
open_bracket: "["
close_parens: ")"
close_brace: "}"
close_bracket: "]"

);
