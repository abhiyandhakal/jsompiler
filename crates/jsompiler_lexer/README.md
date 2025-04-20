# Jsompiler Lexer

The `jsompiler_lexer` is responsible for the **lexical analysis** part of the `jsompiler` project. It takes raw JavaScript source code and converts it into a stream of tokens, which are used by the parser for syntax analysis.
**Note:** This project is currently under active development. Interfaces and behavior may change as the project evolves.

## Features

- Tokenization of keywords, identifiers, numbers, and strings
- Support for almost all JavaScript operators and delimiters
- Error handling for invalid lexemes
- Easily extendable token definitions
- **Limitation:** Unicode escape sequences (e.g., `\u0041`) are **not yet supported**.

## Installation

Add `jsompiler_lexer` to your `Cargo.toml`:

```toml
[dependencies]
jsompiler_lexer = "0.2.1"
```

OR
Run the following Cargo command in your project directory:

```bash
cargo add jsompiler_lexer
```

## Example

```js
const pi = 3.1416;
```

| Lexemes | Tokens                         |
| ------- | ------------------------------ |
| const   | Keyword(Const)                 |
| pi      | Identifier("pi")               |
| =       | Operator(EqualTo)              |
| 3.1416  | Literal(Number(Value(3.1416))) |
| ;       | Delimiter(Semicolon)           |
| EOF     | EOF                            |

## Usage

Add this crate to your project

```rust
use jsompiler_lexer::Lexer;

let source = "const pi = 3.1416;";
let mut lexer = Lexer::new(source);
lexer.scan_all_tokens();
println!("{:#?}", lexer.tokens);
println!("{:#?}", lexer.errors);
```

# API Documentation

## Tokens Supported

### Keywords:

`const`, `function`, `void`, `typeof`, `if`, `return`, `else`, `instanceof`, `in`, `while`, `for`, `do`, `true`, `false`, `exports`, `break`, `continue`, `switch`, `null`, `case`, `debugger`, `class`, `new`, `this`, `super`, `import`, `export`, `default`, `try`, `catch`, `finally`, `throw`, `enum`, `extends`, `delete`

### Contextual Keywords:

`let`, `package`, `interface`, `get`, `set`, `arguments`, `public`, `eval`, `protected`, `private`, `meta`, `target`, `async`, `await`, `from`, `as`, `from`, `as`, `of`, `yield`, `static`, `with`, `implements`

### Operators

#### Arithmetic Operators

`+`, `-`, `*`, `/`, `%`, `**`, `++`, `--`

#### Assignment Operators

`=`, `+=`, `-=`, `*=`, `/=`, `%=`, `**=`, `&=`, `|=`, `^=`, `<<=`, `>>=`, `>>>=`, `&&=`, `||=`, `??=`

#### Comparison Operators

`==`, `===`, `!=`, `!==`, `>`, `<`, `>=`, `<=`

#### Logical Operators

`&&`, `||`, `!`, `??`

#### Bitwise Operators

`&`, `|`, `^`, `~`, `<<`, `>>`, `>>>`

#### Miscellaneous Operators

`=>`, `?`, `:`, `?.`, `...`

### Delimiters

`(`, `)`, `{`, `}` ,`[`, `]`, `;`, `,`, `.`

### Comments

`//`, `/*`

### Literal Types

`undefined`, `null`, `NaN`, `Infinity`
