crate::ssa!(

literal_definition: "int x = 2;" => "Symbols:\n  x0 = 2"

literal_declaration: "int y;" => "Symbols:\n  x0 = \u{2205} "

literal_valid_redeclaration: "int y; { int y = 2; }" =>
"Symbols:
  x0 = \u{2205} 
  x1 = 2"

);
