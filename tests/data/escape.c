
// #include "stdio.h"

int main(void)
{
    // String literals with various escape sequences
    char str1[] = "Hello, world!\nThis is a test string with special characters: \\n, \\t, \\\\";
    // char str2[] = "A string with unicode: \u0043BBBB \u0041BBBB \u0031BBBB \u0042BBBB ";
    char str3[] = "Tab character here: \t and \?N a backslash: \\\\";

    // Special escape characters for paths or Windows
    char path[] = "C:\\Users\\Name\\Documents\\file.txt";

    // Multiline string with escape sequences
    char multiline[] = "This is a multi-line string with escaped newline characters: \nSecond line here.";

    // Character literals
    char ch1 = 'a';    // Simple character
    char ch2 = '\n';   // Newline as a character
    char ch3 = '\x41'; // Hexadecimal escape for 'A'
    char ch4 = '\032'; // Octal escape for 'Z'
    char ch5 = '\x7F'; // Hexadecimal for delete character (DEL)

    // Complex escape sequences in strings
    char complex_string[] = "Complex string with special sequences: \x1B[31mRed text\x1B[0m";

    // Test a raw string literal (C11 feature)
    char raw_string[] = "(This is a raw string where backslashes \n do not escape.)";

    // Print output to verify
    printf("%s\n", str1);
    printf("%s\n", str3);
    printf("Path: %s\n", path);
    printf("Multiline String: %s\n", multiline);
    printf("Character literals: '%c', '%c', '%c', '%c', '%c'\n", ch1, ch2, ch3, ch4, ch5);
    printf("Complex string: %s\n", complex_string);
    printf("Raw string: %s\n", raw_string);

    return 0;
}
