// #include <stdio.h>

int main()
{
    // Weird floating point literals with funky exponents and fractions
    float weird1 = 0xf.ep+02f;                 // Hexadecimal floating point with exponent
    double weird2 = 1.23e+10;                  // Regular scientific notation
    double weird3 = 3.14159265358979323846e-2; // High precision scientific notation
    float weird4 = 0x1.abc2p+4f;               // Hexadecimal floating point with 'p' exponent
    double weird5 = 0.0e-0;                    // Testing zero with 'p' notation
    float weird6 = 0x1.2p+3f;                  // Hexadecimal with fractional part
    double weird7 = 1e+1000;                   // Extremely large exponent
    double weird8 = 1e-1000;                   // Extremely small exponent
    double weird9 = 0x1.23p+4;                 // Another hexadecimal with 'p' notation

    // Weird combinations of scientific notation
    float weird10 = 1.23E4f + 9.87E-3f;
    double weird11 = 1.23e-3 + 3.14E+2;
    double weird12 = 1.5E5 / 0.25E+10;

    // Weird decimal + hex + exponent mix
    double weird13 = 1234.5678 + 0x1.abc3p+10;

    // Weird number formats testing
    float weird14 = 0x10.0p+3f;  // Hex with 'p' notation (hexadecimal floating-point)
    double weird15 = 0xA.Fp+2;   // Hexadecimal fractional part with 'p' exponent
    double weird16 = 0x1.1p-2;   // Another hexadecimal floating-point with negative exponent
    double weird17 = 0xF.FFFp+3; // Hexadecimal floating-point with several fractional digits

    // Output the weird literals to see how they get parsed
    printf("Weird 1 (0xf.ep+02f): %.7f\n", weird1);
    printf("Weird 2 (1.23e+10): %.7f\n", weird2);
    printf("Weird 3 (3.14159265358979323846e-2): %.7f\n", weird3);
    printf("Weird 4 (0x1.abc2p+4f): %.7f\n", weird4);
    printf("Weird 5 (0.0p-0): %.7f\n", weird5);
    printf("Weird 6 (0x1.2p+3f): %.7f\n", weird6);
    printf("Weird 7 (1e+1000): %.7e\n", weird7);
    printf("Weird 8 (1e-1000): %.7e\n", weird8);
    printf("Weird 9 (0x1.23p+4): %.7f\n", weird9);
    printf("Weird 10 (1.23E4f + 9.87E-3f): %.7f\n", weird10);
    printf("Weird 11 (1.23e-3 + 3.14E+2): %.7f\n", weird11);
    printf("Weird 12 (1.5E5 / 0.25E+10): %.7f\n", weird12);
    printf("Weird 13 (1234.5678 + 0x1.abc3p+10): %.7f\n", weird13);
    printf("Weird 14 (0x10.0p+3f): %.7f\n", weird14);
    printf("Weird 15 (0xA.Fp+2): %.7f\n", weird15);
    printf("Weird 16 (0x1.1p-2): %.7f\n", weird16);
    printf("Weird 17 (0xF.FFFp+3): %.7f\n", weird17);

    return 0;
}
