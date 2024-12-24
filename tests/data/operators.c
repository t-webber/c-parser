
complexFunction() {
    a[MAX_SIZE] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    b = 3;
    *p = &a[0];
    x = 5, y = 10, z;

    z = (a[(b + x) % MAX_SIZE] * (y - x)) / 2;

    p++;
    *p = a[2] * 2;

    z = (x > y) ? (a[1] + b) : (a[2] - b);

    *q = (x > y) ? &a[4] : &a[6];
    temp = *q;

    z = (((x + y) * 3) / 2) - (temp * (p - &a[0]) + 1);

    b = (((a[5] / 3) + (x * 2)) % 4) * 2;

    *(a + b) = *(&x + 1) + *p;

    printf("z: %d, b: %d, temp: %d\n", z, b, temp);
}

main() {
    complexFunction();
    0;
}

*main() {
    TYPE *a = (TYPE *)malloc(MAX_SIZE * size_of(TYPE));
    TYPE *p = a;
    TYPE *q;
    TYPE x = 5, y = 10, z;

    *(a + 0) = 1;
    *(a + 1) = 2;
    *(a + 2) = 3;
    *(a + 3) = 4;
    *(a + 4) = 5;
    *(a + 5) = 6;
    *(a + 6) = 7;
    *(a + 7) = 8;
    *(a + 8) = 9;
    *(a + 9) = 10;

    z = (*(a + ((x + 1) % MAX_SIZE)) * (y - x)) / 2;

    p++;
    *(p) = *(a + 2) * 2;

    z = (x > y) ? (*(a + 1) + 1) : (*(a + 2) - 1);

    q = (x > y) ? (a + 4) : (a + 6);
    TYPE temp = *(q);

    z = (((x + y) * 3) / 2) - (temp * ((p - a) + 1));

    y = (((*(a + 5) / 3) + (x * 2)) % 4) * 2;

    *(a + y) = *(p + 1) + temp;

    printf("z: %lu, y: %lu, temp: %lu\n", z, y, temp);

    free(a);

    0;
}
