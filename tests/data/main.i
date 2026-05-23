
static unsigned naive(unsigned word) {
        unsigned count = 0;
        for (unsigned i = 0; i < 32; ++i) count += word >> i & 1;
        return count;
}

int main(int argc) {
        return naive(argc);
}
