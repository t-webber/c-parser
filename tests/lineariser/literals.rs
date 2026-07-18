//! Lineariser tests.

crate::ssa!(

lin_lit_char: "int x = 'c'"

lin_lit_bool: "int x = true"

lin_lit_null: "int *x = nullptr"

lin_lit_str: r#"int *x = "abcdef"; "#

lin_lit_int: "int x = 1"

lin_lit_long: "int x = 1L"

lin_lit_long_long: "int x = 1LL"

lin_lit_uint: "int x = 1U"

lin_lit_ulong: "int x = 1UL"

lin_lit_ulonglong: "int x = 1ULL"

lin_lit_float: "int x = 1.2f"

lin_lit_double: "int x = 1.2"

lin_lit_long_double: "int x = 1.2l"

);
