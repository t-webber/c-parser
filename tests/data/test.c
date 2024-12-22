// int ex1 = a + b * c - d / e % f + g - h * i + j % k * l ^ m & n | o && p ||
// q;
// int ex2 = a * (b + c - d / e % f * g) +
//                           (h > i ? j : k) * (l && m || n ^ o) / (p ? q : r) +
//                           t &
//                       u |
//                   v &&
//               w
//           ? x
//           : y ^ z,
//     a + b;
// int ex3 = a * b + c - d / e % f * g + h & i | j ^ k && l ||
//               m * n + o - p * q / r + s % t
//           ? u
//           : v && w ^ x | y && z;
int main() {
    int a, b, c, d;
    int ex4 = a + [(b + c) + d];
}