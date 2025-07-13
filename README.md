# strings

A Rust library for string processing, text normalization, and name parsing with Spanish language support.

## Features

- **Text Sanitization**: Clean up whitespace, normalize Unicode characters, and remove accents
- **Name Processing**: Format names with proper capitalization and compare by first letters
- **Spanish Language Support**: Parse Spanish name components (`de`, `la`, `del`, `y`)
- **Iterator-based API**: Memory-efficient processing with lazy evaluation

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
strings = "0.0.1"
```

### Basic Examples

```rust
use strings::*;

// Sanitize whitespace
let clean = sanitize_spaces("\t\t\n Hello,\n\n\t \r\n world!\n\t\n");
assert_eq!(clean, "Hello, world!");

// Normalize names
let formatted: String = sanitize_name_iter("CARLOS FERNANDO").collect();
assert_eq!(formatted, "Carlos Fernando");

// Compare names by first letters
assert!(compare_first_letter("Bob A. Wilson", "Bob Antonio W."));

// Extract words
let words: Vec<&str> = iter_words("Hello   world\t\ntest").collect();
assert_eq!(words, vec!["Hello", "world", "test"]);
```

### Spanish Name Parsing

```rust
use strings::Partes;
use logos::Logos;

let name = "Sarah Elizabeth Martinez de la Cruz";
let tokens: Vec<Partes> = Partes::lexer(name)
    .map(|t| t.unwrap())
    .collect();
```

## Dependencies

- `logos` - Fast lexical analysis
- `unicode-normalization` - Unicode normalization support
- `edit-distance` - String distance calculations

## License

This project is licensed under the terms specified in the `Cargo.toml` file.