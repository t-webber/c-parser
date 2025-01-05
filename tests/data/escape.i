int main(void) {
    char str1[] =
        "Hello, world!\nThis is a test string with special characters: \\n, "
        "\\t, \\\\";

    char str3[] = "Tab character here: \t and \?N a backslash: \\\\";

    char path[] = "C:\\Users\\Name\\Documents\\file.txt";

    char multiline[] =
        "This is a multi-line string with escaped newline characters: \nSecond "
        "line here.";

    char ch1 = 'a';
    char ch2 = '\n';
    char ch3 = '\x41';
    char ch4 = '\032';
    char ch5 = '\x7F';

    char complex_string[] =
        "Complex string with special sequences: \x1B[31mRed text\x1B[0m";

    char raw_string[] =
        "(This is a raw string where backslashes \n do not escape.)";

    printf("%s\n", str1);
    printf("%s\n", str3);
    printf("Path: %s\n", path);
    printf("Multiline String: %s\n", multiline);
    printf("Character literals: '%c', '%c', '%c', '%c', '%c'\n", ch1, ch2, ch3,
           ch4, ch5);
    printf("Complex string: %s\n", complex_string);
    printf("Raw string: %s\n", raw_string);

    return 0;
}
