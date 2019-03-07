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
//! let tree: GameTree = parse("(;EV[event]PB[black]PW[white]C[comment];B[aa])").unwrap();
//! ```

#![feature(slice_patterns)]
#![deny(rust_2018_idioms)]

mod model;
mod parser;
mod error;

pub use crate::model::*;
pub use crate::error::{SgfError, SgfErrorKind};
pub use crate::parser::parse;
