crate::ast!(

lengths_literal: "x = 'c' blob;"

lengths_symbols: "<<="

suggestion_then_error: "f(x,) )"

in_parens: "(static_assert const)"

nomad_else: "else"

nomad_brace: "{"

nomad_bracket: "a[3]]"

invalid_keyword: "const sizeof *x = 1;"

successive_numbers: "a 2"

successive_numbers_long: "a 22222"

two_colons: "const x : :"

sizeof_bitfield: "sizeof :"

comma_colon: "const x, :"

declaration_operator: "const int a +"

bitfield_number_name: "const int a : 2 name"

bitfield_operator: "const int a : +"

bitfield_not_number: "const int a : 'b'"

);
