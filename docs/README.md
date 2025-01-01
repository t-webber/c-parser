# C parser

This is a rust library that lexes and parses C source files.

## Standard

For the moment, this parser is only meant to  support C23 standard C code. No extensions (e.g. GCC extensions) are implemented. The input file is supposed already preprocessed.

## Lexer

The lexer takes as input the preprocessed C source code, and transforms into a valid token: keywords, number constants, identifiers, symbols, strings and chars.

## Parser

The parser takes these tokens and tries to build an Abstract Syntax Tree (AST). The AST is not meant to be valid as it is building AST so it contains empty nodes while building that are meant to disappear before the end of the parsing stage.
