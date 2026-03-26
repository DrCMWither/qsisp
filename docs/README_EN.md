# qƨisp

[DE](README_DE.md)|[EN](README_EN.md)|[FR](README_FR.md)|[ZH](README.md)

### Programming Language: Create Your Own!

---

## What is qƨisp?

**qƨisp** (if your font doesn’t support character "ƨ", call it *qU+01A8isp*. And no, *qsisp* is not the official name!) is a locale-driven, multilingual Lisp dialect.

It adapts itself based on your system language:

* Keywords;
* Parentheses;
* Strings;
* Comments;
* Your mental state!

## Features

### Locale-based syntax

Write code in your own language—literally! You don’t need to learn qƨisp, or even Lisp keywords: You might not even need to know English, qƨisp will adapt to you.

## Side Effects

* You will never look at ASCII parentheses the same way again;
* Mental breakdown;
* IDE: **I quit.**

## Example

```lisp
“begin
  “define x (10)
  “if (<= x 10)
    “print (x)
””””
```

Try rewriting this in the French version.

Or mix all languages together.

We strongly recommend against doing that.


## Local Run / Development

1. Clone the repository:

```bash
git clone https://github.com/DrCMWither/qsisp.git
cd qsisp
```

2. Make sure your local environment is up to date. This project requires `Rust >= 1.75` and `cargo`.

3. Run directly or build a release version:

```bash
cargo run -- example/test.qs
cargo build --release
```

## Awards

* Cross-Cultural Programming Horror Award
* Special Mention for Psychological Damage at the Reader Layer
* Hardest-to-Stomach Lisp Dialect of 2026

## Roadmap

* Proper RTL parsing
* Mixed-language AST
* IDE plugin (basically unusable)
* Formal semantics (if anyone survives)

*By the way — the name of this language is not a palindrome.*