# Jsompiler Parser

The `jsompiler_parser` crate handles the **syntax analysis** part of the `jsompiler` project. It processes the token stream from the lexer and builds an **Abstract Syntax Tree (AST)** that represents the structure of JavaScript code.

## Features

- Parses JavaScript syntax into an AST
- Basic error handling and recovery
- Handles operator precedence and associativity
- Detects and reports syntax errors with location context
- Modular design with separate components for expressions, statements, etc.

**Note:** This project is under active development. Grammar coverage is partial and subject to change as the project evolves.

# API Documentation

## Supported Syntax

The parser currently supports `PrimaryExpression` forms, including:

- this
- Identifiers
- Literals
- Array and Object literals
- Function and Class expressions
- Generator and Async Function expressions

The parser currently supports key JavaScript statements including:

- BlockStatement
- VariableStatement
- ExpressionStatement
- IfStatement
- ReturnStatement
- WhileStatement
- ForStatement

## To Be Implemented

- Additional statement types:
  - `SwitchStatement`, `BreakStatement`, `ContinueStatement`, `WithStatement`,`ThrowStatement`, etc.
- Support for try-catch-finally blocks
- Parsing of import/export modules
- Improved error recovery and diagnostics
- More comprehensive unit tests

## Specification

This parser is designed to follow the [ECMAScript Language Specification](https://tc39.es/ecma262/), as defined by the TC39 committee. Grammar rules and AST structures are implemented to closely align with the standard.
