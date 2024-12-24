use ccompiler::prelude::*;

fn test_parser_on_string(content: &str, output: &str) {
    let files = &[(String::new(), content)];
    let mut location = Location::from(String::new());
    let tokens = lex_file(content, &mut location).unwrap_or_display(files, "lexer");
    println!("Tokens = {:?}", &tokens);
    let node = parse_tokens(tokens).unwrap_or_display(files, "parser");
    assert!(
        output == format!("{node}"),
        "Mismatch! Expected:\n{output}\n!= Computed\n{node}"
    );
}

#[test]
fn parser_1() {
    // unary, binary
    test_parser_on_string(
        "a + b * c - d / e % f + g - h * i + j % k * l ^ !m++ & n | o || p && q",
        "[((((((((a + (b * c)) - ((d / e) % f)) + g) - (h * i)) + ((j % k) * l)) ^ ((!(m++)) & n)) | o) || (p && q))]",
    );
}

#[test]
fn parser_2() {
    // unary, binary, ternary, blocks
    test_parser_on_string(
        "a * b + c - d / e % f * g + h & i | j ^ k && l ||
        m * n + o - p * q / r + s % t
        ? u
        : v && w ^ x | y && z; !a",
        "[(((((((((a * b) + c) - (((d / e) % f) * g)) + h) & i) | (j ^ k)) && l) || ((((m * n) + o) - ((p * q) / r)) + (s % t))) ? u : ((v && ((w ^ x) | y)) && z)), (!a)]",
    );
}

#[test]
fn parser_3() {
    // unary, binary, ternary, parens, assign
    test_parser_on_string(
        "ex2 = a * (b + c - d / e % f * g) +
                          (h > i ? j : k) * (l && m || n ^ o) / (p ? q : r) +
                          t &
                      u |
                  v &&
              w
          ? x
          : y ^ z",
          "[(ex2 = (((((((a * (((b + c) - (((d / e) % f) * g)))) + (((((h > i) ? j : k)) * (((l && m) || (n ^ o)))) / ((p ? q : r)))) + t) & u) | v) && w) ? x : (y ^ z)))]");
}
