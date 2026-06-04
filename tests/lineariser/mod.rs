mod declarations;

/// Convenience macro to test parsing on a string.
#[macro_export]
macro_rules! test_ssa {
        ($($name:ident: $input:expr => $output:expr)*) => {
            $(
                #[test]
                fn $name() {
                    $crate::test_string($input, $output)
                }
            )*
        };
}

/// Convenience macro to test parsing on a string, when the parsing is supposed
/// to return an error.
#[macro_export]
macro_rules! test_ssa_error {
    ($($name:ident: $input:expr => $output:expr)*) => {
        $(
            #[test]
            fn $name() {
                $crate::test_string_error($input, $output)
            }
        )*

    };
}
