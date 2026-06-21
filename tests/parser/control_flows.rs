crate::ast!(

for_loops: "for(int i = 0; i < 9+1; i++) printf(\"i = %d\", i);"

structs: "struct A { int x }; struct A a;"

enumeration: "enum A { FIRST, SECOND }; enum A a;"

union: "union A { bool active }; union A a;"

successive_ctrl_flow: "break; return 0*1; for(int x = 2; x<10;x++) x"

conditional_simple: "if (a) b else if (c) d else e; if(x) y;z"

nested_conditional: "if (z) x * y else if (!c) {if (x*y << 2) {x} else {4}} else { x }"

conditional_return: "if (a) return b; else return c; return d"

conditional_operators: "if (z) x * y else if (!c) {if (x*y << 2) return x; else return 4;} f()"

iterators: "while (1) for (int x = 1; x<CONST;  x++) if (x) return a<<=2, 1+a; else continue;0"

empty_block: "if (a) {} else {}"

break_inside_nested_loops: "for(int i = 0; i < 3; i++) { for(int j = 0; j < 3; j++) { if(i% 4==2) break; } }"

nested_if_else: "if(a) { if(b) c = 1; else c = 2; } else { if(d) c = 3; else c = 4; }"

logical_operators_conditional: "if(a && b || c) x = 1; else y = 2;"

empty_else_block: "if(a) x = 1; else ;"

while_loop_with_break: "while(a) { if(b) break; c = 5; }"

switch_case: "switch(x) { case 1: y = 2; break; case 2: y = 3; default: y = 4; }"

nested_loops_with_control_flow:
    "for(int i = 0; i < 3; i++)
        for(int j = 1; j < 4; j++) {
            if(i == j) continue;
            printf(\"i = %d, j = %d\", i, j);
        }
    int x;
    "

 continue_inside_for_loop:
    "for(int i = 0; i < 5; i++)
        { if(i == 3) continue; printf(\"%d\", i); x}
    y"

multiple_variables: "int i = 1, j = 2;"

for_loop_multiple_variables:
    "for(int i = 0, j = 5; i < 10 && j > 0; i++, j--)
        printf(\"i = %d, j = %d\", i, j);
    int x;"

typedef_struct_definition: "typedef struct a { int x[]; const *volatile *int y; } b"

typedef_struct: "typedef struct a b"

typedef_int: "typedef const int *c"

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

goto_statement: "int x = 0; goto label_name; x = 1; label_name: x = 2;"

switch_with_empty_case: "switch(x) { case 1:; case 2: y = 3; break; }"

while_with_continue: "while (a) { if (b) continue; c = 5; }"

else_if_chain_with_break: "if (a) { x = 1; } else if (b) { break; } else { y = 2; }"

for_loop_with_empty_body: "for (int i = 0; i < 10; i++);"

nested_while_with_break: "while (a) { while (b) { if (c) break; d = 5; } }"

switch_with_fallthrough: "switch(x) { case 1: y = 2; case 2: y = 3; break; }"

empty_switch_case: "switch(x) {}"

nested_switch: "switch(x) { case 1: switch(y) { case 2: z = 3; break; } break; }"

while_with_empty_body: "while (a);"

do_while_with_empty_body: "do; while (a);"

nested_do_while: "do { do { x = 1; } while (y); } while (z);"


typedef_equal: "typedef a ="

);
