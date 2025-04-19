# Jsompiler Lexer

`jsompiler_lexer` is a lexer for JavaScript.

## Structure

- `jsx/` - Handles jsx
- `comment.rs` – Logic for handling single-line, multi-line, and hashbang comments.
- `identifier.rs` – Tokenization of identifiers and reserved words.
- `lib.rs` – Entry point of the crate; ties all modules together.
- `number.rs` – Parsing for number literals.
- `operator_punctuation.rs` – Operator and punctuation-specific token logic.
- `string.rs` – Handling of string literals and escape characters.
- `symbol.rs` – Defines token types used, stores mappings for keywords, operators, and delimiters.
- `test.rs` – Unit tests for various lexer behaviors.

## Features

- Tokenization of keywords, identifiers, numbers, and strings
- Support for almost all JavaScript operators and delimiters
- Error handling for invalid lexemes
- Easily extendable token definitions

## LEXER

Lexer or Lexical Analyzer is the first phase of the compiler which reads input characters(source code) and produces a sequence of tokens that the parser uses for syntax analysis.

## Tokens, Patterns, Lexemes

Terms like "tokens", "patterns", "lexeme" are generally used in the Lexer. Pattern describes a rule(regular expression or grammar rules) that must be matched by sequence of characters(lexemes) to form a token.

```js
const pi = 3.1416;
```

| Lexemes | Tokens            |
| ------- | ----------------- |
| const   | Keyword           |
| pi      | Identifier        |
| =       | Assignment symbol |
| 3.1416  | Number            |

## Tokens Supported

### Keywords:

`const`, `const`, `function`, `void`, `typeof`, `if`, `return`, `else`, `instanceof`, `in`, `while`, `for`, `do`, `true`, `false`, `exports`, `break`, `continue`, `switch`, `null`, `case`, `debugger`, `class`, `new`, `this`, `super`, `import`, `export`, `default`, `try`, `catch`, `finally`, `throw`, `enum`, `extends`, `delete`

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

## Installation

Add `jsompiler_lexer` to your `Cargo.toml`:

```toml
[dependencies]
jsompiler_lexer = "0.2.0"
```

OR
Run the following Cargo command in your project directory:

```bash
cargo add jsompiler_lexer
```

## Usage

Add this crate to your project

```rust
use jsompiler_lexer::Lexer;

let file = read_to_string("./test.js");
if file.is_err() {
	return;
}
let file = file.unwrap();
let mut lexer = Lexer::new(file);
lexer.scan_all_tokens();
println!("{:#?}", lexer.tokens);
println!("{:#?}", lexer.errors);
```
