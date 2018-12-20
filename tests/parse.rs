#[cfg(test)]
mod tests {
    use sgf_parser::*;

    #[test]
    fn can_parse_komi() {
        assert_eq!(
            parse("(;KM[6.5])"),
            Ok(SgfGameTree {
                root: SgfNode {
                    tokens: vec![
                        SgfToken::Komi(6.5f32)
                    ],
                    children: vec![]
                    }
                })
        );
    }

    #[test]
    fn can_parse_game_tree() {
        assert_eq!(
            parse("(;B[dc]BL[3498])"),
            Ok(SgfGameTree {
                root: SgfNode {
                    tokens: vec![
                        SgfToken::Move(Move {
                            stone: Stone::Black,
                            coordinate: (4, 3)
                        }),
                        SgfToken::Time(Time {
                            stone: Stone::Black,
                            time: 3498
                        })
                    ],
                    children: vec![]
                }
            })
        );
    }

    #[test]
    fn can_parse_game_tree_two_nodes() {
        assert_eq!(
            parse("(;B[dc];W[ef])"),
            Ok(SgfGameTree {
                root: SgfNode {
                    tokens: vec![SgfToken::Move(Move {
                        stone: Stone::Black,
                        coordinate: (4, 3)
                    })],
                    children: vec![SgfNode {
                        tokens: vec![SgfToken::Move(Move {
                            stone: Stone::White,
                            coordinate: (5, 6)
                        })],
                        children: vec![]
                    }]
                }
            })
        );
    }

    #[test]
    fn can_parse_game_tree_simple_branch() {
        assert_eq!(
            parse("(;B[aa](;W[bb])(;W[cc]))"),
            Ok(SgfGameTree {
                root: SgfNode {
                    tokens: vec![SgfToken::Move(Move {
                        stone: Stone::Black,
                        coordinate: (1, 1)
                    })],
                    children: vec![
                        SgfNode {
                            tokens: vec![SgfToken::Move(Move {
                                stone: Stone::White,
                                coordinate: (2, 2)
                            })],
                            children: vec![]
                        },
                        SgfNode {
                            tokens: vec![SgfToken::Move(Move {
                                stone: Stone::White,
                                coordinate: (3, 3)
                            })],
                            children: vec![]
                        }
                    ]
                }
            })
        );
    }

    #[test]
    fn can_parse_game_information() {
        assert_eq!(
            parse("(;EV[event]PB[black]PW[white]C[comment];B[aa])"),
            Ok(SgfGameTree {
                root: SgfNode {
                    tokens: vec![
                        SgfToken::Event("event".to_string()),
                        SgfToken::PlayerName(Player {
                            stone: Stone::Black,
                            name: "black".to_string()
                        }),
                        SgfToken::PlayerName(Player {
                            stone: Stone::White,
                            name: "white".to_string()
                        }),
                        SgfToken::Comment("comment".to_string()),
                    ],
                    children: vec![SgfNode {
                        tokens: vec![SgfToken::Move(Move {
                            stone: Stone::Black,
                            coordinate: (1, 1)
                        })],
                        children: vec![]
                    }]
                }
            })
        );
    }
}
