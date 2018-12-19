//! # SGF Parser for Rust
//!
//! A sgf parser for rust, using `nom`. 
//!
//! Supports most basic SGF properties, and tree branching.
//!
//! ## Output
//! 
//! Output is a `SgfGameTree`, containing a single root `SgfNode`.
//!
//! ## Coming features
//!
//! - reading marks
//! - support for all SGF properties
//! - support converting back to SGF
//!
//! # Examples
//! ```rust
//! use sgf_parser::*;
//!
//! let tree: SgfGameTree = parse("(;EV[event]PB[black]PW[white]C[comment];B[aa])").unwrap();
//! ```

#[macro_use]
extern crate nom;

mod model;
mod parser;

pub use crate::model::*;
pub use crate::parser::parse;