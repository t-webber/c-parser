use std::fs;

use ccompiler::prelude::*;
use number_types::Number;
use tokens_types::TokenValue;

const PREFIX: &str = "./tests/data/lexer-";

#[expect(clippy::unwrap_used)]
fn test_lexer_on_file(file: &str) {
    let path = format!("{PREFIX}{file}.c");
    let content = fs::read_to_string(&path).unwrap();
    let mut location = Location::from(path.clone());
    let _tokens = lex_file(&content, &mut location).unwrap_or_display(&[(path, &content)], "lexer");
}

fn test_lexer_on_number(content: &str, expected: Number) {
    let path = String::new();
    let mut location = Location::from(path.as_str());
    let tokens = lex_file(content, &mut location).unwrap_or_display(&[(path, content)], "lexer");
    assert!(tokens.len() == 1, "Lexer error: cut expression into 2 tokens, but only a number was expected: {content} was cut into {tokens:?}");
    let value = tokens.first().unwrap().get_value();
    if let TokenValue::Number(nb) = value {
        assert!(
            *nb == expected,
            "Lexer error: computed wrong number: Expected: {expected:?}\n != Computed: {value:?}"
        );
    } else {
        panic!("Lexer error: waiting for Number, but lexer returned {value:?}")
    }
}

#[test]
fn lexer_escape() {
    test_lexer_on_file("escape");
}

#[test]
fn lexer_general() {
    test_lexer_on_file("general");
}

macro_rules! gen_number_test {
    ($($name:ident: $input:expr => $output:expr;)*) => {
        $(
            #[test]
            fn $name() {
                test_lexer_on_number($input, $output)
            }
        )*
    };
}

gen_number_test!(
    lexer_numbers_1: "1" => Number::Int(1);
    lexer_numbers_2: "0xf.ep+02f" => Number::Float(63.5);
    lexer_numbers_3: "1.23e+10" => Number::Double(1.23e10);
    lexer_numbers_4: "3.14159265358979323846e-2" => Number::Double(0.031415926535897934);
    lexer_numbers_5: "0x1.abc2p+4f" => Number::Float(26.734863);
    lexer_numbers_6: "0.0e-0" => Number::Double(0.);
    lexer_numbers_7: "0x1.2p+3f" => Number::Float(9.);
    lexer_numbers_8: "1e+1000" => Number::Double(f64::INFINITY);
    lexer_numbers_9: "1e-1000" => Number::Double(0.);
    lexer_numbers_10: "0x1.23p+4" => Number::Double(18.1875);
    lexer_numbers_11: "1.23E4f" => Number::Float(12300.);
    lexer_numbers_12: "9.87E-3f" => Number::Float(0.00987);
    lexer_numbers_13: "0x1.abc3p+10" => Number::Double(1711.046875);
    lexer_numbers_14: "0x10.0p+3f" => Number::Float(128.);
    lexer_numbers_15: "0xA.Fp+2" => Number::Double(43.75);
    lexer_numbers_16: "0x1.1p-2" => Number::Double(0.265625);
    lexer_numbers_17: "0xF.FFFp+3" => Number::Double(127.998046875);
    lexer_numbers_18: "0b101010" => Number::Int(42);
    lexer_numbers_19: "072" => Number::Int(58);
    lexer_numbers_20: "0xA7F" => Number::Int(2687);
    lexer_numbers_21: "12345" => Number::Int(12345);
    lexer_numbers_22: "1.23e+10" => Number::Double(1.23e10);
    lexer_numbers_23: "4.56e-5" => Number::Double(4.56e-5);
    lexer_numbers_24: "7.89E-2" => Number::Double(7.89e-2);
    lexer_numbers_25: "1.23E+100" => Number::Double(1.23e100);
    lexer_numbers_26: "1.23F" => Number::Float(1.23);
    // lexer_numbers_27: "4.56L" => Number::LongDouble(4.56); //TODO: long double not supported
    lexer_numbers_28: ".5" => Number::Double(0.5);
    lexer_numbers_29: "5." => Number::Double(5.);
    lexer_numbers_30: "1e10" => Number::Double(1e10);
    lexer_numbers_31: "3.45E-2" => Number::Double(3.45e-2);
    lexer_numbers_32: "0b11111111" => Number::Int(255);
    lexer_numbers_33: "0xABC12345" => Number::UInt(2881561413);
    lexer_numbers_34: "04567U" => Number::UInt(2423);
    lexer_numbers_35: "1000000000000000LL" => Number::LongLong(1000000000000000);
    lexer_numbers_36: "123.456f" => Number::Float(123.456);
    lexer_numbers_37: "789.0123" => Number::Double(789.0123);
    lexer_numbers_38: "0.0001e5f" => Number::Float(10.);
);
