# SGF Parser

An SGF Parser for Rust.

Very rudimentary implementation for now, but supports most SGF properties and simple tree branching.

## Usage

```rust
use sgf_parser::*

let tree = Result<SgfTree> = parse("(;EV[event]PB[black]PW[white]C[comment];B[aa])");
```
