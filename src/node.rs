use crate::SgfToken;

/// A game node, containing a vector of tokens
#[derive(Debug, PartialEq, Clone)]
pub struct GameNode {
    pub tokens: Vec<SgfToken>,
}

impl GameNode {
    /// Gets a vector of all `SgfToken::Unknown` tokens
    pub fn get_unknown_tokens(&self) -> Vec<&SgfToken> {
        self.tokens
            .iter()
            .filter(|token| match token {
                SgfToken::Unknown(_) => true,
                _ => false,
            })
            .collect()
    }

    /// Gets a vector of all `SgfToken::Invalid` tokens
    pub fn get_invalid_tokens(&self) -> Vec<&SgfToken> {
        self.tokens
            .iter()
            .filter(|token| match token {
                SgfToken::Invalid(_) => true,
                _ => false,
            })
            .collect()
    }
}

impl Into<String> for &GameNode {
    fn into(self) -> String {
        let mut token_strings: Vec<String> = self.tokens.iter().map(|t| t.into()).collect();
        token_strings.sort();
        let (_, out) = token_strings
            .iter()
            .fold((None, vec![";"]), |(prev, mut out), token| {
                let offset = token.find('[').unwrap_or_else(|| token.len());
                match prev {
                    Some(ref prop) if token.starts_with(prop) => {
                        out.push(&token[offset..]);
                        (prev, out)
                    }
                    _ => {
                        out.push(&token);
                        (Some(&token[0..offset]), out)
                    }
                }
            });
        out.join("")
    }
}

impl Into<String> for GameNode {
    fn into(self) -> String {
        (&self).into()
    }
}
