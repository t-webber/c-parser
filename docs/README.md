# C parser

[![github](https://img.shields.io/badge/GitHub-t--webber/c--parser-blue?logo=GitHub)](https://github.com/t-webber/c-parser)
[![license](https://img.shields.io/badge/License-GPL3.0-darkgreen)](https://github.com/t-webber/c-parser?tab=GPL-3.0-1-ov-file)
[![rust-version](https://img.shields.io/badge/Rust--version-1.85+-purple?logo=Rust)](https://releases.rs/docs/1.85.0/)
[![rust-edition](https://img.shields.io/badge/Rust--edition-2024-darkred?logo=Rust)](https://doc.rust-lang.org/stable/edition-guide/rust-2024/)

This is a rust library that lexes and parses C source files.

## Standard

For the moment, this parser is only meant to support C23 standard C code. No extensions (e.g. GCC extensions) are implemented. The input file is supposed already preprocessed.

## Lexer

The lexer takes as input the preprocessed C source code, and transforms into a valid token: keywords, number constants, identifiers, symbols, strings and chars.

## Parser

The parser takes these tokens and tries to build an Abstract Syntax Tree (AST). The AST is not meant to be valid as it is building AST so it contains empty nodes while building that are meant to disappear before the end of the parsing stage.

## Examples

```rust
use c_parser::*;

// replace this with your file's name (the file name is needed to display errors nicely)
let filename = String::new();

// replace this with the file's content
let content = r#"
    int main(int argc, char ** argv) {
        printf("Nb of arguments: %d\n", argc);
        for(size_t i = 0; i<argc; i++) {
            printf("\targv[%i] = %d\n", i, argv[i]);
        }
        return 0;
    }"#;

let files = &[(filename.clone(), content)];


// file reader to display the errors
let mut location = Location::from(filename);

// lexer
let tokens = lex_file(content, &mut location).unwrap_or_display(files, "lexer");

// parser
let node = parse_tokens(tokens).unwrap_or_display(files, "parser");

// you can now use the Ast!
println!("{node}");
```
