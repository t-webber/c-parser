use crate::make_string_tests;

make_string_tests!(


unary_binary:
    "z -= a +~ b * (c *= 2) - d / e % f + g - h * i + j % k * l ^ !m++ & n | o || p && q"
    =>
    "[(z -= ((((((((a + ((~b) * ((c *= 2)))) - ((d / e) % f)) + g) - (h * i)) + ((j % k) * l)) ^ ((!(m++)) & n)) | o) || (p && q)))..]"

ternary_blocks:
    "z &= (a /= z) * (b %= y) + c - d / e % f * g + h & i | j ^ k && l ||
        m * n + o - p * q / r + s % t
        ? u
        : v && w ^ x | y && z != 2; ! a>>b"
    =>
    "[(z &= (((((((((((a /= z)) * ((b %= y))) + c) - (((d / e) % f) * g)) + h) & i) | (j ^ k)) && l) || ((((m * n) + o) - ((p * q) / r)) + (s % t))) ? u : ((v && ((w ^ x) | y)) && (z != 2)))), ((!a) >> b)..]"

assign:
    "a + b >= 0 ? (c ^= 0) * !(e|=1) : (d >>= x[3])"
    =>
    "[(((a + b) >= 0) ? (((c ^= 0)) * (!((e |= 1)))) : ((d >>= (x[3]))))..]"

incr_comment:
    "a*b++/*x*/"
    =>
    "[(a * (b++))..]"

);
