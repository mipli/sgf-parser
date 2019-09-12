#[cfg(test)]
mod tree_tests {
    use sgf_parser::Action::Move;
    use sgf_parser::*;

    #[test]
    fn can_convert_game_tree_without_variations() {
        let tree = GameTree {
            nodes: vec![
                GameNode {
                    tokens: vec![
                        SgfToken::PlayerName {
                            color: Color::Black,
                            name: "black".to_string(),
                        },
                        SgfToken::PlayerName {
                            color: Color::White,
                            name: "white".to_string(),
                        },
                    ],
                },
                GameNode {
                    tokens: vec![SgfToken::Move {
                        color: Color::Black,
                        action: Move(3, 3),
                    }],
                },
                GameNode {
                    tokens: vec![SgfToken::Move {
                        color: Color::White,
                        action: Move(16, 16),
                    }],
                },
            ],
            variations: vec![],
        };
        let string_tree: String = tree.into();
        assert_eq!(string_tree, "(;PB[black]PW[white];B[cc];W[pp])");
    }

    #[test]
    fn can_convert_game_tree_with_variations() {
        let tree = GameTree {
            nodes: vec![
                GameNode {
                    tokens: vec![
                        SgfToken::PlayerName {
                            color: Color::Black,
                            name: "black".to_string(),
                        },
                        SgfToken::PlayerName {
                            color: Color::White,
                            name: "white".to_string(),
                        },
                    ],
                },
                GameNode {
                    tokens: vec![SgfToken::Move {
                        color: Color::Black,
                        action: Move(3, 3),
                    }],
                },
                GameNode {
                    tokens: vec![SgfToken::Move {
                        color: Color::White,
                        action: Move(16, 16),
                    }],
                },
            ],
            variations: vec![
                GameTree {
                    nodes: vec![GameNode {
                        tokens: vec![SgfToken::Move {
                            color: Color::Black,
                            action: Move(4, 16),
                        }],
                    }],
                    variations: vec![],
                },
                GameTree {
                    nodes: vec![GameNode {
                        tokens: vec![SgfToken::Move {
                            color: Color::Black,
                            action: Move(16, 4),
                        }],
                    }],
                    variations: vec![],
                },
            ],
        };
        let string_tree: String = tree.into();
        assert_eq!(
            string_tree,
            "(;PB[black]PW[white];B[cc];W[pp](;B[dp])(;B[pd]))"
        );
    }
}
