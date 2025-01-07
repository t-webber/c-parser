# Precedence and Associativity of C operators

| Precedence |   Operator   |                  Description                   | Associativity |
| :--------: | :----------: | :--------------------------------------------: | :-----------: |
|     1      |    ++ --     |     Suffix/postfix increment and decrement     | Left-to-right |
|     "      |      ()      |                 Function call                  |       "       |
|     "      |      []      |               Array subscripting               |       "       |
|     "      |      .       |         Struct and union member access         |       "       |
|     "      |      ->      | Struct and union member access through pointer |       "       |
|     "      | (type){list} |            Compound literal `(C99)`            |       "       |
|     2      |    ++ --     |     Prefix increment and decrement [cf 1.]     | Right-to-left |
|     "      |     + -      |              Unary plus and minus              |       "       |
|     "      |     ! ~      |          Logical NOT and bitwise NOT           |       "       |
|     "      |    (type)    |                      Cast                      |       "       |
|     "      |      *       |           Indirection (dereference)            |       "       |
|     "      |      &       |                   Address-of                   |       "       |
|     "      |    sizeof    |                Size-of [cf 2.]                 |       "       |
|     "      |   _Alignof   |         Alignment requirement `(C11)`          |       "       |
|     3      |    * / %     |    Multiplication, division, and remainder     | Left-to-right |
|     4      |     + -      |            Addition and subtraction            |       "       |
|     5      |    << >>     |       Bitwise left shift and right shift       |       "       |
|     6      |     < <=     |        For relational operators < and ≤        |       "       |
|            |     > >=     |        For relational operators > and ≥        |       "       |
|     7      |    == !=     |             For relational = and ≠             |       "       |
|     8      |      &       |                  Bitwise AND                   |       "       |
|     9      |      ^       |           Bitwise XOR (exclusive or)           |       "       |
|     10     |              |           Bitwise OR (inclusive or)            |       "       |
|     11     |      &&      |                  Logical AND                   |       "       |
|     12     |     \|\|     |                   Logical OR                   |       "       |
|     13     |      ?:      |          Ternary conditional [cf 3.]           | Right-to-left |
| 14 [cf 4.] |      =       |               Simple assignment                |       "       |
|     "      |    += -=     |        Assignment by sum and difference        |       "       |
|     "      |   *= /= %=   | Assignment by product, quotient, and remainder |       "       |
|     "      |   <<= >>=    |   Assignment by bitwise left and right shift   |       "       |
|     "      |  &= ^= \|=   |     Assignment by bitwise AND, XOR, and OR     |       "       |
|     15     |      ,       |                     Comma                      | Left-to-right |

## Notes

1. The operand of prefix `++` and `--` can't be a type cast. This rule grammatically forbids some expressions that would be semantically invalid anyway. Some compilers ignore this rule and detect the invalidity semantically.
2. The operand of sizeof can't be a type cast: the expression `sizeof (int) \* p` is unambiguously interpreted as `(sizeof(int)) * p`, but not `sizeof((int)*p)`.
3. The expression in the middle of the conditional operator (between ? and:) is parsed as if parenthesized: its precedence relative to ?: is ignored.
4. Assignment operators' left operands must be unary (level-2 non-cast) expressions. This rule grammatically forbids some expressions that would be semantically invalid anyway. Many compilers ignore this rule and detect the invalidity semantically. For example, `e = a < d ? a++ : a = d` is an expression that cannot be parsed because of this rule. However, many compilers ignore this rule and parse it as `e = ( ((a < d) ? (a++) : a) = d )`, and then give an error because it is semantically invalid.

> Link to the original material: [CppReference](https://en.cppreference.com/w/c/language/operator_precedence).
