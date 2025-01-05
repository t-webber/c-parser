int main()
{
    int a = 5, b = 10, c;

    // An unusual ternary inside a printf
    printf("Result: %d\n", (a > b) ? (a * b) : (a + b));

    // Using macros with inline assembly
    c = A(b, c);
    printf("Macro A result: %d\n", c);

    __asm__(
        "movl %0, %%eax"
        "addl $10, %%eax"
        : "=a"(c)
        : "r"(c));
    printf("Inline ASM result: %d\n", c);

    // Nested preprocessor conditionals

    // Testing a for loop with odd increment/decrement
    for (int i = 0; i < 100; i += 3)
    {
        if (i % 5 == 0)
        {
            continue;
        }
        if (i > 50)
            break;
        printf("i: %d\n", i);
    }

    printf("Macro with result: %d\n", (a > 50) ? (A(b, c) + B(a)) : (A(b, c) - B(a)));

    // Testing switch cases and fallthrough
    SWITCH_CASE(b);

    // Pointer arithmetic
    int *ptr = &a;
    *(ptr++) = 42;
    printf("Pointer manipulation result: %d\n", *ptr);

    // Complex struct declaration
    struct S
    {
        int x;
        float y;
        char z;
    };

    struct S s = {42, 3.14, 'a'};
    printf("Struct result: %d, %.2f, %c\n", s.x, s.y, s.z);

    // Anonymous union
    union
    {
        int i;
        float f;
        char c;
    } u;

    u.i = 42;
    printf("Union result: %d\n", u.i);

    // A mix of casting and pointer tricks
    double pi = 3.141592653589793;
    void *ptr2 = &pi;
    printf("Pointer cast result: %.10f\n", *((double *)ptr2));

    // Complex expression to test parsing
    int result = (((5 + 3) * 2) - (12 / 4)) % 3;
    printf("Complex expression result: %d\n", result);

    // A very odd combination of operators
    int weird = (5 & 3) | (12 ^ 7) && 1 << 2;
    printf("Weird result: %d\n", weird);

    return 0;
}
