int complexFunction(int a, int b) {
    if (a > b) {
        if (a % 2 == 0) {
            switch (b) {
                case 1:
                    printf("Case 1\n");
                    break;
                case 2:
                    printf("Case 2\n");
                    break;
                default:
                    printf("Default case\n");
                    break;
            }
        } else {
            printf("A is odd\n");
        }
    } else {
        for (int i = 0; i < b; i++) {
            if (i % 2 == 0) {
                printf("Even: %d\n", i);
            } else {
                printf("Odd: %d\n", i);
            }
        }
    }
    return a + b;
}

int main() {
    int x = 5, y = 10;
    
    if (x > 0 && y > 0) {
        printf("Positive numbers\n");
    } else if (x < 0 || y < 0) {
        printf("One of the numbers is negative\n");
    } else {
        printf("Both numbers are zero or negative\n");
    }

    while (x < 20) {
        if (x == 15) {
            printf("Halfway there!\n");
            break;
        }
        printf("Incrementing x: %d\n", x);
        x += 2;
    }

    do {
        if (x % 3 == 0) {
            printf("x is divisible by 3\n");
        } else {
            printf("x is not divisible by 3\n");
        }
        x--;
    } while (x > 0);

    goto skip;
    
    for (int i = 0; i < 3; i++) {
        if (i == 1) {
            goto end_loop;
        }
        printf("Loop iteration: %d\n", i);
    }
    
    end_loop:
    printf("Exited loop\n");

skip:
    printf("Skipping code\n");

    // Testing a more complex case with nested loops and conditionals
    for (int i = 0; i < 5; i++) {
        for (int j = 0; j < 5; j++) {
            if (i == j) {
                printf("i equals j at (%d, %d)\n", i, j);
            } else {
                printf("i: %d, j: %d\n", i, j);
            }
        }
    }

    // Test ternary operators
    int result = (x > y) ? complexFunction(x, y) : complexFunction(y, x);
    printf("Result of complexFunction: %d\n", result);
    
    return 0;
}
