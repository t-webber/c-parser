use expressions::prelude::*;

fn test_parser_on_string(input: &str, output: &str) {
    let mut location = Location::from(String::new());
    let Res {
        errors: lex_errors,
        result: tokens,
    } = lex_file(input, &mut location);
    if !lex_errors.is_empty() {
        display_errors(lex_errors, &[(String::new(), input)], "lexing");
        panic!();
    }
    let Res {
        errors: pars_errors,
        result: node,
    } = parse_tokens(tokens);
    if !pars_errors.is_empty() {
        display_errors(pars_errors, &[(String::new(), input)], "parsing");
        panic!();
    }
    assert!(
        output == format!("{node}"),
        "Mismatch! Expected:\n{output}\n!= Computed\n{node}"
    );
}

#[test]
fn parser_0() {
    // unary, binary
    test_parser_on_string("!m++", "!(m++)");
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
        "[(ex2 = (((((((a * ((b + c) - (((d / e) % f) * g))) + ((((h > i) ? j : k) * ((l && m) || (n ^ o))) / (p ? q : r))) + t) & u) | v) && w) ? x : (y ^ z)))]",
    );
}
