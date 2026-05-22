// #include <stdio.h>

struct Point {
    int x, y;
};

struct ComplexStruct {
    int a, b;
    struct Point p;
};

int square(int x) { return x * x; }

static void print_point(Point pt) { printf("Point: (%d, %d)\n", pt.x, pt.y); }

int main(void) {
    ComplexStruct cs = {5, 10, {3, 4}};
    // (void)cs;
    Point pt = {7, 8};
    int a = 3, b = 4;
    double result = 0.0;

    result = a * b + (square(a) - square(b)) / (a + b);
    printf("Result of the complex calculation: %f\n", result);

    print_point(pt);
    pt.x = pt.y = pt.x + pt.y;
    print_point(pt);

    // int arr[5] = {1, 2, [4] = 10};

    ComplexStruct dynamic_cs = {20, 30, {11, 12}};
    printf("Dynamic struct values: %d, %d, (%d, %d)\n", dynamic_cs.a,
           dynamic_cs.b, dynamic_cs.p.x, dynamic_cs.p.y);

    int *ptr = &arr[0];
    ptr += 2;
    printf("Pointer arithmetic result: %d\n", *ptr);

    // return 0;
}