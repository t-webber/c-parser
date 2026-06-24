//! Manual lexer tests

#![expect(clippy::restriction, reason = "tests")]

use c_parser::lex;

#[test]
fn dbg_keyword() {
    let expected = "[Token { location: \"\", value: Keyword(Sizeof) }]";
    assert_eq!(format!("{:?}", lex("sizeof", "").unwrap_or_display(&[]).unwrap()), expected);
}
