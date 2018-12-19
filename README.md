[![Build Status](https://travis-ci.com/mipli/sgf-parser.svg?branch=master)](https://travis-ci.com/mipli/sgf-parser)
[![Crate](https://img.shields.io/crates/v/sgf-parser.svg)](https://crates.io/crates/sgf-parser)

# SGF Parser

An SGF Parser for Rust.

Very rudimentary implementation for now, but supports most SGF properties and simple tree branching.

## Usage

```rust
use sgf_parser::*

let tree = Result<SgfTree> = parse("(;EV[event]PB[black]PW[white]C[comment];B[aa])");
```
