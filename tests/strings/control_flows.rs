use crate::make_string_tests;

make_string_tests!(

for_loops:
    "for(int i = 0; i < 9+1; i++) printf(\"i = %d\", i);"
    =>
    "[<for ([(int:(i = 0)), (i < (9 + 1)), (i++)]) (printf°(\"i = %d\", i))>..]"

structs:
    "struct A { int x };
    struct A a;"
    =>
    "[<struct A [(int:x)]>, (struct A:a), \u{2205} ..]"

successive_ctrl_flow:
    "break;
    return 0*1;
    for(int x = 2; x<10;x++) x"
    =>
    "[<break>, <return (0 * 1)>, <for ([(int:(x = 2)), (x < 10), (x++)]) x..>..]"

conditional_simple:
    "if (a) b else if (c) d else e; if(x) y;z"
    =>
    "[<if (a) b else <if (c) d else e>>, <if (x) y.\u{b2}.>, z..]"

nested_conditional:
    "if (z) x * y else if (!c) {if (x*y << 2) {x} else {4}} else { x }"
    =>
    "[<if (z) (x * y) else <if ((!c)) [<if (((x * y) << 2)) [x] else [4]>] else [x]>>..]"

conditional_return:
    "if (a) return b; else return c; return d"
    =>
    "[<if (a) <return b> else <return c>>, <return d..>..]"

conditional_operators:
    "if (z) x * y else if (!c) {if (x*y << 2) return x; else return 4;} f()"
    =>
    "[<if (z) (x * y) else <if ((!c)) [<if (((x * y) << 2)) <return x> else <return 4>>].\u{b2}.>.\u{b2}.>, (f°())..]"

iterators:
    "while (1) for (int x = 1; x<CONST;  x++) if (x) return a<<=2, 1+a; else continue;0"
    =>
    "[<while (1) <for ([(int:(x = 1)), (x < CONST), (x++)]) <if (x) <return ((a <<= 2) , (1 + a))> else <continue>>>>, 0..]"

empty_block:
    "if (a) {} else {}"
    =>
    "[<if (a) [] else []>..]"

break_inside_nested_loops:
    "for(int i = 0; i < 3; i++) { for(int j = 0; j < 3; j++) { if(i% 4==2) break; } }"
    =>
    "[<for ([(int:(i = 0)), (i < 3), (i++)]) [<for ([(int:(j = 0)), (j < 3), (j++)]) [<if (((i % 4) == 2)) <break>.\u{b2}.>]>]>..]"

nested_if_else:
    "if(a) { if(b) c = 1; else c = 2; } else { if(d) c = 3; else c = 4; }"
    =>
    "[<if (a) [<if (b) (c = 1) else (c = 2)>] else [<if (d) (c = 3) else (c = 4)>]>..]"


logical_operators_conditional:
    "if(a && b || c) x = 1; else y = 2;"
    =>
    "[<if (((a && b) || c)) (x = 1) else (y = 2)>..]"

empty_else_block:
    "if(a) x = 1; else ;"
    =>
    "[<if (a) (x = 1) else \u{2205} >..]"

while_loop_with_break:
    "while(a) { if(b) break; c = 5; }"
    =>
    "[<while (a) [<if (b) <break>.\u{b2}.>, (c = 5), \u{2205} ]>..]"

switch_case:
    "switch(x) { case 1: y = 2; break; case 2: y = 3; default: y = 4; }"
    =>
    "[<switch (x) [<case 1: [(y = 2), <break>, \u{2205} ..]..>, <case 2: [(y = 3), \u{2205} ..]..>, <default: [(y = 4), \u{2205} ..]..>]>..]"

nested_loops_with_control_flow:
    "for(int i = 0; i < 3; i++)
        for(int j = 1; j < 4; j++) {
            if(i == j) continue;
            printf(\"i = %d, j = %d\", i, j);
        }
    int x;
    "
     =>
    "[<for ([(int:(i = 0)), (i < 3), (i++)]) <for ([(int:(j = 1)), (j < 4), (j++)]) [<if ((i == j)) <continue>.\u{b2}.>, (printf°(\"i = %d, j = %d\", i, j)), \u{2205} ]>>, (int:x), \u{2205} ..]"

 continue_inside_for_loop:
    "for(int i = 0; i < 5; i++)
        { if(i == 3) continue; printf(\"%d\", i); x}
    y"
    =>
    "[<for ([(int:(i = 0)), (i < 5), (i++)]) [<if ((i == 3)) <continue>.\u{b2}.>, (printf°(\"%d\", i)), x]>, y..]"

multiple_variables:
    "int i = 1, j = 2;"
    =>
    "[(int:(i = 1), (j = 2)), \u{2205} ..]"

for_loop_multiple_variables:
    "for(int i = 0, j = 5; i < 10 && j > 0; i++, j--)
        printf(\"i = %d, j = %d\", i, j);
    int x;"
    =>
    "[<for ([(int:(i = 0), (j = 5)), ((i < 10) && (j > 0)), ((i++) , (j--))]) (printf°(\"i = %d, j = %d\", i, j))>, (int:x), \u{2205} ..]"

typedef_struct_definition:
    "typedef struct a { int x[]; const *volatile *int y; } b"
    =>
    "[<typedef <struct a [((int:x)[]), (const * volatile * int:y), \u{2205} ]> b>..]"

typedef_struct:
    "typedef struct a b"
    =>
    "[<typedef (struct a:b)..>..]"

typedef_int:
    "typedef const int *c"
    =>
    "[<typedef (const int *:c)..>..]"

do_while:
    "int f(int x) {
        do {
        if (x++) {
            return x;
            }
        } while (x <= 10);
        return -1;
    }
    "
    =>
    "[((int:f)°((int:x))), [<do [<if ((x++)) [<return x>].\u{b2}.>] while ((x <= 10))>, <return (-1)>]..]"

goto_statement:
    "int x = 0; goto label_name; x = 1; label_name: x = 2;"
    =>
    "[(int:(x = 0)), <goto label_name>, (x = 1), <label_name: (x = 2)>, \u{2205} ..]"

switch_with_empty_case:
    "switch(x) { case 1:; case 2: y = 3; break; }"
    =>
    "[<switch (x) [<case 1: \u{2205} ..>, <case 2: [(y = 3), <break>, \u{2205} ..]..>]>..]"

while_with_continue:
    "while (a) { if (b) continue; c = 5; }"
    =>
    "[<while (a) [<if (b) <continue>.\u{b2}.>, (c = 5), \u{2205} ]>..]"

else_if_chain_with_break:
    "if (a) { x = 1; } else if (b) { break; } else { y = 2; }"
    =>
    "[<if (a) [(x = 1), \u{2205} ] else <if (b) [<break>, \u{2205} ] else [(y = 2), \u{2205} ]>>..]"

for_loop_with_empty_body:
    "for (int i = 0; i < 10; i++);"
    =>
    "[<for ([(int:(i = 0)), (i < 10), (i++)]) \u{2205} >..]"

nested_while_with_break:
    "while (a) { while (b) { if (c) break; d = 5; } }"
    =>
    "[<while (a) [<while (b) [<if (c) <break>.\u{b2}.>, (d = 5), \u{2205} ]>]>..]"

switch_with_fallthrough:
    "switch(x) { case 1: y = 2; case 2: y = 3; break; }"
    =>
    "[<switch (x) [<case 1: [(y = 2), \u{2205} ..]..>, <case 2: [(y = 3), <break>, \u{2205} ..]..>]>..]"


);
