#[cfg(test)]
mod parser_tests {
    use sgf_parser::*;

    #[test]
    fn can_parse_komi() {
        assert_eq!(
            parse("(;KM[6.5])"),
            Ok(GameTree {
                nodes: vec![
                        GameNode {
                            tokens: vec![SgfToken::Komi(6.5f32)]
                        }
                    ],
                variations: vec![]
            })
        );
    }

    #[test]
    fn can_ignore_lowercase_characters() {
        assert_eq!(
            parse("(;CopyRight[2017])"),
            Ok(GameTree {
                nodes: vec![
                    GameNode {
                        tokens: vec![
                            SgfToken::Copyright("2017".to_string())
                        ],
                    }
                ],
                variations: vec![]
            })
        );
    }

    #[test]
    fn can_parse_game_tree_single_node() {
        assert_eq!(
            parse("(;B[dc]BL[3498])"),
            Ok(GameTree {
                nodes: vec![
                    GameNode {
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
                    }
                ],
                variations: vec![]
            })
        );
    }

    #[test]
    fn can_parse_game_tree_two_nodes() {
        assert_eq!(
            parse("(;B[dc];W[ef])"),
            Ok(GameTree {
                nodes: vec![
                    GameNode {
                        tokens: vec![
                            SgfToken::Move(Move {
                                stone: Stone::Black,
                                coordinate: (4, 3)
                            }),
                        ],
                    },
                    GameNode {
                        tokens: vec![
                            SgfToken::Move(Move {
                                stone: Stone::White,
                                coordinate: (5, 6)
                            }),
                        ],
                    }
                ],
                variations: vec![]
            })
        );
    }

    #[test]
    fn can_parse_game_tree_simple_branch() {
        assert_eq!(
            parse("(;B[aa](;W[bb])(;W[cc]))"),
            Ok(GameTree {
                nodes: vec![
                    GameNode {
                        tokens: vec![
                            SgfToken::Move(Move {
                                stone: Stone::Black,
                                coordinate: (1, 1)
                            }),
                        ],
                    },
                ],
                variations: vec![
                    GameTree {
                        nodes: vec![
                            GameNode {
                                tokens: vec![
                                    SgfToken::Move(Move {
                                        stone: Stone::White,
                                        coordinate: (2, 2)
                                    }),
                                ],
                            },
                        ],
                        variations: vec![]
                    },
                    GameTree {
                        nodes: vec![
                            GameNode {
                                tokens: vec![
                                    SgfToken::Move(Move {
                                        stone: Stone::White,
                                        coordinate: (3, 3)
                                    }),
                                ],
                            },
                        ],
                        variations: vec![]
                    }
                ]
            })
        );
    }

    #[test]
    fn can_parse_game_information() {
        assert_eq!(
            parse("(;EV[event]PB[black]PW[white]C[comment];B[aa])"),
            Ok(GameTree {
                nodes: vec![
                    GameNode {
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
                    },
                    GameNode {
                        tokens: vec![
                            SgfToken::Move(Move {
                                stone: Stone::Black,
                                coordinate: (1, 1)
                            }),
                        ],
                    }
                ],
                variations: vec![]
            })
        );
    }
    /*

    #[test]
    fn can_parse_unkown_tags() {
        assert_eq!(
            parse("(;B[dc];FO[asdf];W[ef])"),
            Ok(SgfGameTree {
                root: SgfNode {
                    tokens: vec![SgfToken::Move(Move {
                        stone: Stone::Black,
                        coordinate: (4, 3)
                    })],
                    invalid: vec![],
                    children: vec![SgfNode {
                        tokens: vec![],
                        invalid: vec![SgfToken::Unknown("FO[asdf]".to_string())],
                        children: vec![SgfNode {
                            tokens: vec![SgfToken::Move(Move {
                                stone: Stone::White,
                                coordinate: (5, 6)
                            })],
                            invalid: vec![],
                            children: vec![]
                        }]
                    }]
                }
            })
        );
    }
    */
}
