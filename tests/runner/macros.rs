/// Convenience macro to create tests with the right scope.
#[macro_export]
macro_rules! one_test {
    ($name:ident, $scope:ident, $input:expr) => {
        #[test]
        fn $name() {
            $crate::runner::test(
                module_path!(),
                stringify!($name),
                $input,
                $crate::runner::run::TestScope::$scope,
            )
        }
    };
}

/// Convenience macro to create ast tests.
#[macro_export]
macro_rules! ast {
    ($($name:ident: $input:expr)*) => {
        $($crate::one_test!($name, Ast, $input);)*
    };
}

/// Convenience macro to create ast tests while ignoring errors.
#[macro_export]
macro_rules! ast_no_error {
    ($($name:ident: $input:expr)*) => {
        $($crate::one_test!($name, AstNoError, $input);)*
    };
}

/// Convenience macro to create ssa tests.
#[macro_export]
macro_rules! ssa {
    ($($name:ident: $input:expr)*) => {
        $($crate::one_test!($name, Ssa, $input);)*
    };
}
