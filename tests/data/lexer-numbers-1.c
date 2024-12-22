int main()
{
    // Testing weird number formats
    int bin = 0b101010; // Binary literal (base 2)
    int oct = 072;      // Octal literal (base 8)
    int hex = 0xA7F;    // Hexadecimal literal (base 16)
    int dec = 12345;    // Decimal literal (base 10)

    printf("Binary 0b101010: %d\n", bin);
    printf("Octal 072: %d\n", oct);
    printf("Hexadecimal 0xA7F: %d\n", hex);
    printf("Decimal 12345: %d\n", dec);

    // Weird exponent notations
    float weird_exp = 1.23e+10;         // Scientific notation with a positive exponent
    float weird_exp_neg = 4.56e-5;      // Scientific notation with a negative exponent
    double weird_exp_float = 7.89E-2;   // Scientific notation with uppercase "E"
    double weird_exp_large = 1.23E+100; // Large exponent

    printf("Weird exponent 1.23e+10: %.2f\n", weird_exp);
    printf("Weird exponent 4.56e-5: %.5f\n", weird_exp_neg);
    printf("Weird exponent 7.89E-2: %.2f\n", weird_exp_float);
    printf("Weird exponent 1.23E+100: %.2e\n", weird_exp_large);
    // Testing floating-point with unusual syntaxes
    float float_point_1 = 1.23F; // Float literal with 'F'
    // double double_point_1 = 4.56L;       // Double literal with 'L'
    float point_zero = .5;               // Floating point with leading decimal
    float point_zero_2 = 5.;             // Floating point with trailing decimal
    double point_exponential = 1e10;     // Exponent notation without the decimal part
    double point_exponential2 = 3.45E-2; // Exponent with negative power and lowercase "e"

    printf("Float with F: %.2f\n", float_point_1);
    // printf("Double with L: %.2lf\n", double_point_1);
    printf("Point with leading decimal: %.1f\n", point_zero);
    printf("Point with trailing decimal: %.1f\n", point_zero_2);
    printf("Exponent without decimal part: %.2e\n", point_exponential);
    printf("Exponent with negative power: %.4e\n", point_exponential2);

    // Testing strange numbers with mixed syntax
    int weird_num_1 = 0b11111111; // Binary with apostrophe
    // int weird_num_2 = 0xABC12345;              // Hexadecimal with apostrophe
    unsigned int weird_unsigned = 04567U;      // Octal with 'U' suffix
    long long weird_long = 1000000000000000LL; // Long long with 'LL' suffix
    float weird_float = 123.456f;              // Float with 'f' suffix
    double weird_double = 789.0123;            // Double with 'D' suffix

    printf("Binary with apostrophe: %d\n", weird_num_1);
    // printf("Hexadecimal with apostrophe: %d\n", weird_num_2);
    printf("Unsigned octal 04567U: %u\n", weird_unsigned);
    printf("Long long 1000000000000000LL: %lld\n", weird_long);
    printf("Float 123.456f: %.3f\n", weird_float);
    printf("Double 789.0123D: %.4lf\n", weird_double);

    // Complex floating-point expressions
    float complex_expr1 = 1.23e4f + 9.87e-3f;
    double complex_expr2 = 4.56E+2 - 7.89E-5;
    float complex_expr3 = 1.2E+3f * 5.6e-4f;

    printf("Complex floating-point 1: %.5f\n", complex_expr1);
    printf("Complex floating-point 2: %.5f\n", complex_expr2);
    printf("Complex floating-point 3: %.5f\n", complex_expr3);

    // Mixed number and exponent expressions
    float mixed_expr = 0b1010 + 0xFA * 0.0001e5f;
    printf("Mixed expression result: %.2f\n", mixed_expr);

    return 0;
}
