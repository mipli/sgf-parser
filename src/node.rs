
use crate::{SgfToken};

/// A game node, containing a vector of tokens 
#[derive(Debug, PartialEq, Clone)]
pub struct GameNode {
    pub tokens: Vec<SgfToken>,
}

impl GameNode {
    /// Gets a vector of all `SgfToken::Unknown` tokens
    pub fn get_unknown_tokens(&self) -> Vec<&SgfToken> {
        self
            .tokens
            .iter()
            .filter(|token| {
                 match token {
                    SgfToken::Unknown(_) => true,
                    _ => false
                 }
            })
            .collect::<Vec<_>>()
    }

    /// Gets a vector of all `SgfToken::Invalid` tokens
    pub fn get_invalid_tokens(&self) -> Vec<&SgfToken> {
        self
            .tokens
            .iter()
            .filter(|token| {
                 match token {
                    SgfToken::Invalid(_) => true,
                    _ => false
                 }
            })
            .collect::<Vec<_>>()
    }
}
