//! # SGF Parser for Rust
//!
//! A sgf parser for rust. Supports all SGF properties, and tree branching.
//!
//! Usin `pest` for the actual parsing part.
//!
//!
//! ## Coming features
//!
//! - reading marks
//! - support converting back to SGF
//!
//! # Example usage
//! ```rust
//! use sgf_parser::*;
//!
//! let tree: Result<GameTree, SgfError> = parse("(;EV[event]PB[black]PW[white]C[comment];B[aa])");
//!
//! let tree = tree.unwrap();
//! let unknown_nodes = tree.get_unknown_nodes();
//! assert_eq!(unknown_nodes.len(), 0);
//!
//! let invalid_nodes = tree.get_invalid_nodes();
//! assert_eq!(invalid_nodes.len(), 0);
//!
//! tree.iter().for_each(|node| {
//!   assert!(!node.tokens.is_empty());
//! });
//! ```

#![feature(slice_patterns)]
#![deny(rust_2018_idioms)]

mod error;
mod model;
mod parser;

pub use crate::error::{SgfError, SgfErrorKind};
pub use crate::model::*;
pub use crate::parser::parse;
