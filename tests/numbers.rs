use c_parser::*;

fn test_number(content: &str, expected: Number) {
    let path = String::new();
    let mut location = LocationPointer::from(path.as_str());
    let tokens = lex_file(content, &mut location).unwrap_or_display(&[(path, content)], "lexer");
    assert!(
        tokens.len() == 1,
        "Lexer error: cut expression into 2 tokens, but only a number was expected: {content} was cut into {}",
        display_tokens(&tokens)
    );
    let value = tokens.first().unwrap().as_value();
    if let TokenValue::Number(nb) = value {
        assert!(
            *nb == expected,
            "Lexer error: computed wrong number: Expected: {expected:?}\n != Computed: {value:?}"
        );
    } else {
        panic!("Lexer error: waiting for Number, but lexer returned {value:?}")
    }
}

macro_rules! gen_number_test {
    ($($name:ident: $input:expr => $output:expr;)*) => {
        $(
            #[test]
            fn $name() {
                test_number($input, $output)
            }
        )*
    };
}

gen_number_test!(
    numbers_1: "0" => Number::Int(0);
    numbers_2: "10999999999999999999u" => Number::ULong(10999999999999999999);
    numbers_3: "25u" => Number::UInt(25);
    numbers_4: "25l" => Number::Long(25);
    numbers_5: "25ul" => Number::ULong(25);
    numbers_6: "25ull" => Number::ULongLong(25);
    numbers_7: "0xf.ep+02f" => Number::Float(63.5);
    numbers_8: "1.23e+10" => Number::Double(1.23e10);
    numbers_9: "3.14159265358979323846e-2" => Number::Double(0.031415926535897934);
    numbers_10: "0x1.abc2p+4f" => Number::Float(26.734863);
    numbers_11: "0.0e-0" => Number::Double(0.);
    numbers_12: "0x1.2p+3f" => Number::Float(9.);
    numbers_13: "1e+1000" => Number::Double(f64::INFINITY);
    numbers_14: "1e-1000" => Number::Double(0.);
    numbers_15: "0x1.23p+4" => Number::Double(18.1875);
    numbers_16: "1.23E4f" => Number::Float(12300.);
    numbers_17: "9.87E-3f" => Number::Float(0.00987);
    numbers_18: "0x1.abc3p+10" => Number::Double(1711.046875);
    numbers_19: "0x10.0p+3f" => Number::Float(128.);
    numbers_20: "0xA.Fp+2" => Number::Double(43.75);
    numbers_21: "0x1.1p-2" => Number::Double(0.265625);
    numbers_22: "0xF.FFFp+3" => Number::Double(127.998046875);
    numbers_23: "0b101010" => Number::Int(42);
    numbers_24: "072" => Number::Int(58);
    numbers_25: "0x1234567AF" => Number::Long(0x1234567AF);
    numbers_26: "12345" => Number::Int(12345);
    numbers_27: "1.23e+10" => Number::Double(1.23e10);
    numbers_28: "4.56e-5" => Number::Double(4.56e-5);
    numbers_29: "7.89E-2" => Number::Double(7.89e-2);
    numbers_30: "1.23E+100" => Number::Double(1.23e100);
    numbers_31: "1.23F" => Number::Float(1.23);
    // numbers_32: "4.56L" => Number::LongDouble(4.56); // long double not supported
    numbers_33: ".5" => Number::Double(0.5);
    numbers_34: "5." => Number::Double(5.);
    numbers_35: "1e10" => Number::Double(1e10);
    numbers_36: "3.45E-2" => Number::Double(3.45e-2);
    numbers_37: "0b11111111" => Number::Int(255);
    numbers_38: "0xABC12345" => Number::UInt(2881561413);
    numbers_39: "04567U" => Number::UInt(2423);
    numbers_40: "1000000000000000LL" => Number::LongLong(1000000000000000);
    numbers_41: "123.456f" => Number::Float(123.456);
    numbers_42: "789.0123" => Number::Double(789.0123);
    numbers_43: "0.0001e5f" => Number::Float(10.);
    numbers_44: "0x0.123456789abcdefp0" => Number::Double(0.07111111111111111);

);
